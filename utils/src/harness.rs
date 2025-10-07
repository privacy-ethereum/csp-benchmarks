use crate::bench::{Metrics, compile_binary, run_measure_mem_script, write_json_metrics};
use crate::metadata::SHA2_INPUTS;
use criterion::{BatchSize, Criterion};

const SAMPLE_SIZE: usize = 10;

#[derive(Clone, Copy, Debug)]
pub enum BenchTarget {
    Sha256,
    Ecdsa,
    Keccak,
    // Add more targets here
}

impl BenchTarget {
    pub fn as_str(&self) -> &'static str {
        match self {
            BenchTarget::Sha256 => "sha256",
            BenchTarget::Ecdsa => "ecdsa",
            BenchTarget::Keccak => "keccak",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ProvingSystem {
    Binius64,
    Expander,
    Plonky2,
    Powdr,
    Provekit,
    Circom,
    Risc0,
    Sp1,
    Jolt,
    Miden,
    // Extend as needed
}

impl ProvingSystem {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProvingSystem::Binius64 => "binius64",
            ProvingSystem::Expander => "expander",
            ProvingSystem::Plonky2 => "plonky2",
            ProvingSystem::Powdr => "powdr",
            ProvingSystem::Provekit => "provekit",
            ProvingSystem::Circom => "circom",
            ProvingSystem::Risc0 => "risc0",
            ProvingSystem::Sp1 => "sp1",
            ProvingSystem::Jolt => "jolt",
            ProvingSystem::Miden => "miden",
        }
    }

    pub fn is_zkvm(&self) -> bool {
        matches!(
            self,
            ProvingSystem::Risc0 | ProvingSystem::Sp1 | ProvingSystem::Jolt | ProvingSystem::Miden
        )
    }
}

#[derive(Clone, Debug)]
pub struct BenchHarnessConfig<'a> {
    pub target: BenchTarget,
    pub system: ProvingSystem,
    pub feature: Option<&'a str>,
    pub is_zkvm: bool,
    pub fixed_input_size: Option<usize>,
    pub mem_binary_name: &'a str,
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

fn mem_report_filename(target: &str, size: usize, system: &str, feat: Option<&str>) -> String {
    match feat {
        Some(f) if !f.is_empty() => format!("{}_{}_{}_{}_mem_report.json", target, size, system, f),
        _ => format!("{}_{}_{}_mem_report.json", target, size, system),
    }
}

fn input_sizes_for(target: BenchTarget, _fixed: Option<usize>) -> Vec<usize> {
    match target {
        BenchTarget::Sha256 => SHA2_INPUTS.to_vec(),
        BenchTarget::Ecdsa => vec![32],
        BenchTarget::Keccak => SHA2_INPUTS.to_vec(),
    }
}

pub fn run_benchmarks_fn<
    PreparedContext,
    Proof,
    PrepareFn,
    ProveFn,
    VerifyFn,
    PrepSizeFn,
    ProofSizeFn,
    ExecutionCyclesFn: Fn(&PreparedContext) -> u64,
>(
    c: &mut Criterion,
    cfg: BenchHarnessConfig<'_>,
    mut prepare: PrepareFn,
    mut prove: ProveFn,
    mut verify: VerifyFn,
    mut preprocessing_size: PrepSizeFn,
    mut proof_size: ProofSizeFn,
    execution_cycles: Option<ExecutionCyclesFn>,
) where
    PrepareFn: FnMut(usize) -> PreparedContext + Copy,
    ProveFn: FnMut(&PreparedContext) -> Proof + Copy,
    VerifyFn: FnMut(&PreparedContext, &Proof),
    PrepSizeFn: FnMut(&PreparedContext) -> usize,
    ProofSizeFn: FnMut(&Proof) -> usize,
{
    let target_str = cfg.target.as_str();
    let system_str = cfg.system.as_str();

    for size in input_sizes_for(cfg.target, cfg.fixed_input_size) {
        let prepared_context = prepare(size);

        let mut metrics = init_metrics(&cfg, target_str, system_str, size);
        metrics.preprocessing_size = preprocessing_size(&prepared_context);
        let proof = prove(&prepared_context);
        metrics.proof_size = proof_size(&proof);

        if let Some(ref cycles_fn) = execution_cycles {
            metrics.cycles = cycles_fn(&prepared_context);
        }

        write_json_metrics(target_str, size, system_str, cfg.feature, &metrics);

        measure_ram(&cfg, target_str, system_str, cfg.mem_binary_name, size);

        let mut group = init_bench_group(c, &cfg, target_str, system_str, size);

        let prove_id = bench_id(target_str, size, system_str, cfg.feature, "prove");
        group.bench_function(prove_id, move |bench| {
            bench.iter_batched(
                || prepare(size),
                |prepared| {
                    let _ = (prove)(&prepared);
                },
                BatchSize::SmallInput,
            );
        });

        let verify_id = bench_id(target_str, size, system_str, cfg.feature, "verify");
        group.bench_function(verify_id, |bench| {
            bench.iter_batched(
                || {
                    let prepared = prepare(size);
                    let proof_local = (prove)(&prepared);
                    (prepared, proof_local)
                },
                |(prepared, proof_local)| {
                    (verify)(&prepared, &proof_local);
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
    ExecutionCyclesFn: Fn(&PreparedContext) -> u64,
>(
    c: &mut Criterion,
    cfg: BenchHarnessConfig<'_>,
    shared: SharedState,
    mut prepare: PrepareFn,
    mut prove: ProveFn,
    mut verify: VerifyFn,
    mut preprocessing_size: PrepSizeFn,
    mut proof_size: ProofSizeFn,
    execution_cycles: Option<ExecutionCyclesFn>,
) where
    PrepareFn: FnMut(usize, &SharedState) -> PreparedContext + Copy,
    ProveFn: FnMut(&PreparedContext, &SharedState) -> Proof + Copy,
    VerifyFn: FnMut(&PreparedContext, &Proof, &SharedState),
    PrepSizeFn: FnMut(&PreparedContext, &SharedState) -> usize,
    ProofSizeFn: FnMut(&Proof, &SharedState) -> usize,
{
    let target_str = cfg.target.as_str();
    let system_str = cfg.system.as_str();

    for size in input_sizes_for(cfg.target, cfg.fixed_input_size) {
        let prepared_context = prepare(size, &shared);

        let mut metrics = init_metrics(&cfg, target_str, system_str, size);
        metrics.preprocessing_size = preprocessing_size(&prepared_context, &shared);

        let proof = prove(&prepared_context, &shared);
        metrics.proof_size = proof_size(&proof, &shared);

        if let Some(ref cycles_fn) = execution_cycles {
            metrics.cycles = cycles_fn(&prepared_context);
        }

        write_json_metrics(target_str, size, system_str, cfg.feature, &metrics);

        measure_ram(&cfg, target_str, system_str, cfg.mem_binary_name, size);

        let mut group = init_bench_group(c, &cfg, target_str, system_str, size);

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

fn init_bench_group<'a>(
    c: &'a mut Criterion,
    cfg: &BenchHarnessConfig<'a>,
    target_str: &'static str,
    system_str: &'static str,
    size: usize,
) -> criterion::BenchmarkGroup<'a, criterion::measurement::WallTime> {
    let gid = group_id(target_str, size, system_str, cfg.feature);
    let mut group = c.benchmark_group(gid);
    group.sample_size(SAMPLE_SIZE);
    group
}

fn init_metrics(
    cfg: &BenchHarnessConfig<'_>,
    target_str: &'static str,
    system_str: &'static str,
    size: usize,
) -> Metrics {
    Metrics::new(
        system_str.to_string(),
        cfg.feature.unwrap_or("").to_string(),
        cfg.is_zkvm,
        target_str.to_string(),
        size,
    )
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
macro_rules! __define_benchmark_harness {
    // With shared state
    ($public_group_ident:ident, $target:expr, $system:expr, $feature:expr, $mem_binary_name:expr, { $($shared_init:tt)* },
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let system = $system;
            let cfg = ::utils::harness::BenchHarnessConfig {
                target: $target,
                system,
                feature: $feature,
                mem_binary_name: $mem_binary_name,
                fixed_input_size: None,
                is_zkvm: system.is_zkvm(),
            };
            ::utils::harness::run_benchmarks_with_state_fn(
                c,
                cfg,
                &{ $($shared_init)* },
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
                None::<fn(&_) -> u64>,
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
    // With execution_cycles
    ($public_group_ident:ident, $target:expr, $system:expr, $feature:expr, $mem_binary_name:expr,
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr, $execution_cycles:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let system = $system;
            let cfg = ::utils::harness::BenchHarnessConfig {
                target: $target,
                system,
                feature: $feature,
                mem_binary_name: $mem_binary_name,
                fixed_input_size: None,
                is_zkvm: system.is_zkvm(),
            };
            ::utils::harness::run_benchmarks_fn(
                c,
                cfg,
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
                Some($execution_cycles),
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
    // With shared state - old syntax fallback (6 params)
    ($public_group_ident:ident, $target:expr, $system:expr, $feature:expr, $mem_binary_name:expr, $shared_init:expr,
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let system = $system;
            let cfg = ::utils::harness::BenchHarnessConfig {
                target: $target,
                system,
                feature: $feature,
                mem_binary_name: $mem_binary_name,
                fixed_input_size: None,
                is_zkvm: system.is_zkvm(),
            };
            ::utils::harness::run_benchmarks_with_state_fn(
                c,
                cfg,
                &$shared_init,
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
                None::<fn(&_) -> u64>,
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
    // Without execution_cycles
    ($public_group_ident:ident, $target:expr, $system:expr, $feature:expr, $mem_binary_name:expr,
        $prepare:expr, $prove:expr, $verify:expr, $prep_size:expr, $proof_size:expr
    ) => {
        fn criterion_benchmarks(c: &mut ::criterion::Criterion) {
            let system = $system;
            let cfg = ::utils::harness::BenchHarnessConfig {
                target: $target,
                system,
                feature: $feature,
                mem_binary_name: $mem_binary_name,
                fixed_input_size: None,
                is_zkvm: system.is_zkvm(),
            };
            ::utils::harness::run_benchmarks_fn(
                c,
                cfg,
                $prepare,
                $prove,
                $verify,
                $prep_size,
                $proof_size,
                None::<fn(&_) -> u64>,
            );
        }
        ::criterion::criterion_group!($public_group_ident, criterion_benchmarks);
        ::criterion::criterion_main!($public_group_ident);
    };
}

#[macro_export]
macro_rules! define_benchmark_harness {
    (BenchTarget::Sha256, $($rest:tt)*) => {
        $crate::__define_benchmark_harness!(sha256, $crate::harness::BenchTarget::Sha256, $($rest)*);
    };
    (BenchTarget::Ecdsa, $($rest:tt)*) => {
        $crate::__define_benchmark_harness!(ecdsa, $crate::harness::BenchTarget::Ecdsa, $($rest)*);
    };
    (BenchTarget::Keccak, $($rest:tt)*) => {
        $crate::__define_benchmark_harness!(keccak, $crate::harness::BenchTarget::Keccak, $($rest)*);
    };
}
