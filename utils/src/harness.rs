use crate::bench::{Metrics, compile_binary, run_measure_mem_script, write_json_metrics};
use crate::metadata::SHA2_INPUTS;
use criterion::{BatchSize, Criterion};

const SAMPLE_SIZE: usize = 10;

#[derive(Clone, Copy, Debug)]
pub enum BenchTarget {
    Sha256,
    Ecdsa,
    // Add more targets here
}

impl BenchTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            BenchTarget::Sha256 => "sha256",
            BenchTarget::Ecdsa => "ecdsa",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ProvingSystem {
    Binius64,
    Plonky2,
    Expander,
    // Extend as needed
}

impl ProvingSystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProvingSystem::Binius64 => "binius64",
            ProvingSystem::Plonky2 => "plonky2",
            ProvingSystem::Expander => "expander",
        }
    }
}

#[derive(Clone, Debug)]
pub struct BenchHarnessConfig<'a> {
    pub target: BenchTarget,
    pub system: ProvingSystem,
    pub feature: Option<&'a str>,
    pub is_zkvm: bool,
    pub fixed_input_size: Option<usize>,
    pub mem_binary_name: Option<&'a str>,
}

impl<'a> BenchHarnessConfig<'a> {
    pub fn sha256(
        system: ProvingSystem,
        feature: Option<&'a str>,
        mem_binary_name: Option<&'a str>,
    ) -> Self {
        BenchHarnessConfig {
            target: BenchTarget::Sha256,
            system,
            feature,
            is_zkvm: false,
            fixed_input_size: None,
            mem_binary_name,
        }
    }
}

impl<'a> BenchHarnessConfig<'a> {
    pub fn new(target: BenchTarget, system: ProvingSystem) -> Self {
        BenchHarnessConfig {
            target,
            system,
            feature: None,
            is_zkvm: false,
            fixed_input_size: None,
            mem_binary_name: None,
        }
    }
}

fn feat_suffix(feat: Option<&str>) -> String {
    match feat {
        Some(f) if !f.is_empty() => format!("_{}", f),
        _ => String::new(),
    }
}

fn group_id(target: &str, size: usize, system: &str, feat: Option<&str>) -> String {
    format!("{}_{}_{}{}", target, size, system, feat_suffix(feat))
}

fn bench_id(target: &str, size: usize, system: &str, feat: Option<&str>, which: &str) -> String {
    format!(
        "{}_{}_{}{}_{}",
        target,
        size,
        system,
        feat_suffix(feat),
        which
    )
}

fn metrics_filename(target: &str, size: usize, system: &str, feat: Option<&str>) -> String {
    match feat {
        Some(f) if !f.is_empty() => format!("{}_{}_{}_{}_metrics.json", target, size, system, f),
        _ => format!("{}_{}_{}_metrics.json", target, size, system),
    }
}

fn mem_report_filename(target: &str, size: usize, system: &str, feat: Option<&str>) -> String {
    match feat {
        Some(f) if !f.is_empty() => format!("{}_{}_{}_{}_mem_report.json", target, size, system, f),
        _ => format!("{}_{}_{}_mem_report.json", target, size, system),
    }
}

fn default_mem_binary_name(target: &str) -> String {
    format!("{}_mem", target)
}

fn input_sizes_for(target: BenchTarget, _fixed: Option<usize>) -> Vec<usize> {
    match target {
        BenchTarget::Sha256 => SHA2_INPUTS.to_vec(),
        BenchTarget::Ecdsa => vec![32],
    }
}

pub fn run_benchmarks_fn<
    PreparedContext,
    Proof,
    PrepareFactory,
    Prepare,
    ProveFactory,
    Prove,
    VerifyFactory,
    Verify,
    PrepSizeFactory,
    PrepSize,
    ProofSizeFactory,
    ProofSize,
>(
    c: &mut Criterion,
    cfg: BenchHarnessConfig<'_>,
    prepare_factory: PrepareFactory,
    prove_factory: ProveFactory,
    verify_factory: VerifyFactory,
    preprocessing_size_factory: PrepSizeFactory,
    proof_size_factory: ProofSizeFactory,
) where
    PrepareFactory: Fn() -> Prepare + Copy,
    Prepare: FnMut(usize) -> PreparedContext,
    ProveFactory: Fn() -> Prove + Copy,
    Prove: FnMut(&PreparedContext) -> Proof,
    VerifyFactory: Fn() -> Verify + Copy,
    Verify: FnMut(&PreparedContext, &Proof),
    PrepSizeFactory: Fn() -> PrepSize + Copy,
    PrepSize: FnMut(&PreparedContext) -> usize,
    ProofSizeFactory: Fn() -> ProofSize + Copy,
    ProofSize: FnMut(&Proof) -> usize,
{
    let target_str = cfg.target.as_str();
    let system_str = cfg.system.as_str();

    let mem_bin_name_ref: &str = match cfg.mem_binary_name {
        Some(name) => name,
        None => {
            let s = default_mem_binary_name(target_str);
            Box::leak(s.into_boxed_str())
        }
    };

    for size in input_sizes_for(cfg.target, cfg.fixed_input_size) {
        let mut prepare = prepare_factory();
        let prepared_context = prepare(size);

        let mut metrics = Metrics::new(
            system_str.to_string(),
            cfg.feature.unwrap_or("").to_string(),
            cfg.is_zkvm,
            target_str.to_string(),
            size,
        );
        let mut pre_sz = preprocessing_size_factory();
        metrics.preprocessing_size = pre_sz(&prepared_context);

        let mut do_prove = prove_factory();
        let proof = do_prove(&prepared_context);
        let mut pf_sz = proof_size_factory();
        metrics.proof_size = pf_sz(&proof);

        let metrics_file = metrics_filename(target_str, size, system_str, cfg.feature);
        write_json_metrics(&metrics_file, &metrics);

        measure_ram(&cfg, target_str, system_str, mem_bin_name_ref, size);

        let gid = group_id(target_str, size, system_str, cfg.feature);
        let mut group = c.benchmark_group(gid);
        group.sample_size(SAMPLE_SIZE);

        let prove_id = bench_id(target_str, size, system_str, cfg.feature, "prove");
        let prepare_factory_p = prepare_factory;
        let prove_factory_p = prove_factory;
        group.bench_function(prove_id, move |bench| {
            let mut prepare = prepare_factory_p();
            let mut do_prove = prove_factory_p();
            bench.iter_batched(
                move || prepare(size),
                move |prepared| {
                    let _ = (do_prove)(&prepared);
                },
                BatchSize::SmallInput,
            );
        });

        let verify_id = bench_id(target_str, size, system_str, cfg.feature, "verify");
        let prepare_factory_v = prepare_factory;
        let prove_factory_v = prove_factory;
        let verify_factory_v = verify_factory;
        group.bench_function(verify_id, move |bench| {
            let mut prepare = prepare_factory_v();
            let mut do_prove = prove_factory_v();
            let mut do_verify = verify_factory_v();
            bench.iter_batched(
                move || {
                    let prepared = prepare(size);
                    let proof_local = (do_prove)(&prepared);
                    (prepared, proof_local)
                },
                move |(prepared, proof_local)| {
                    (do_verify)(&prepared, &proof_local);
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

#[allow(clippy::too_many_arguments)]
pub fn run_benchmarks_with_state_fn<
    SharedState: Copy,
    PreparedContext,
    Proof,
    PrepareFn,
    ProveFn,
    VerifyFn,
    PrepSizeFn,
    ProofSizeFn,
>(
    c: &mut Criterion,
    cfg: BenchHarnessConfig<'_>,
    shared: SharedState,
    mut prepare: PrepareFn,
    mut prove: ProveFn,
    mut verify: VerifyFn,
    mut preprocessing_size: PrepSizeFn,
    mut proof_size: ProofSizeFn,
) where
    PrepareFn: FnMut(usize, &SharedState) -> PreparedContext + Copy,
    ProveFn: FnMut(&PreparedContext, &SharedState) -> Proof + Copy,
    VerifyFn: FnMut(&PreparedContext, &Proof, &SharedState),
    PrepSizeFn: FnMut(&PreparedContext, &SharedState) -> usize,
    ProofSizeFn: FnMut(&Proof, &SharedState) -> usize,
{
    let target_str = cfg.target.as_str();
    let system_str = cfg.system.as_str();

    let mem_bin_name_ref: &str = match cfg.mem_binary_name {
        Some(name) => name,
        None => {
            let s = default_mem_binary_name(target_str);
            Box::leak(s.into_boxed_str())
        }
    };

    for size in input_sizes_for(cfg.target, cfg.fixed_input_size) {
        let prepared_context = prepare(size, &shared);

        let mut metrics = Metrics::new(
            system_str.to_string(),
            cfg.feature.unwrap_or("").to_string(),
            cfg.is_zkvm,
            target_str.to_string(),
            size,
        );
        metrics.preprocessing_size = preprocessing_size(&prepared_context, &shared);

        let proof = prove(&prepared_context, &shared);
        metrics.proof_size = proof_size(&proof, &shared);

        let metrics_file = metrics_filename(target_str, size, system_str, cfg.feature);
        write_json_metrics(&metrics_file, &metrics);

        measure_ram(&cfg, target_str, system_str, mem_bin_name_ref, size);

        let gid = group_id(target_str, size, system_str, cfg.feature);
        let mut group = c.benchmark_group(gid);
        group.sample_size(SAMPLE_SIZE);

        let prove_id = bench_id(target_str, size, system_str, cfg.feature, "prove");
        group.bench_function(prove_id, move |bench| {
            bench.iter_batched(
                move || prepare(size, &shared),
                move |prepared| {
                    let _ = (prove)(&prepared, &shared);
                },
                BatchSize::SmallInput,
            );
        });

        let verify_id = bench_id(target_str, size, system_str, cfg.feature, "verify");

        group.bench_function(verify_id, |bench| {
            bench.iter_batched(
                || {
                    let prepared = prepare(size, &shared);
                    let proof_local = (prove)(&prepared, &shared);
                    (prepared, proof_local)
                },
                |(prepared, proof_local)| {
                    (verify)(&prepared, &proof_local, &shared);
                },
                BatchSize::SmallInput,
            );
        });

        group.finish();
    }
}

fn measure_ram(
    cfg: &BenchHarnessConfig<'_>,
    target_str: &'static str,
    system_str: &'static str,
    mem_bin_name_ref: &str,
    size: usize,
) {
    compile_binary(mem_bin_name_ref);
    let bin_path = format!("../target/release/{}", mem_bin_name_ref);
    let mem_json = mem_report_filename(target_str, size, system_str, cfg.feature);
    run_measure_mem_script(&mem_json, &bin_path, size);
}

#[macro_export]
macro_rules! define_benchmark_harness {
    // No shared state
    ($public_group_ident:ident, $cfg:expr,
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let cfg = $cfg;
            ::utils::harness::run_benchmarks_fn(
                c,
                cfg,
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
    // With shared state (e.g., MPI config in Polyhedra Expander)
    ($public_group_ident:ident, $shared_init:block, $cfg:expr,
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let cfg = $cfg;
            ::utils::harness::run_benchmarks_with_state_fn(
                c,
                cfg,
                &$shared_init,
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
}
