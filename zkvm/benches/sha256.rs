use ere_jolt::JOLT_TARGET;
use ere_risc0::RV32_IM_RISC0_ZKVM_ELF;
use ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF;
use utils::{
    bench::{measure_peak_memory, write_csv},
    metadata::SHA2_INPUTS,
};
use zkvm_csp_benchmarks::{
    benchmark::Benchmark,
    programs::{
        JoltBuilder, Risc0Builder, Sp1Builder,
        sha256::{Sha256, Sha256Config, Sha256Generator},
    },
    traits::Program,
};

/// SHA256 RISC0 benchmark.
fn risc0_sha256() {
    let configs = SHA2_INPUTS.map(Sha256Config::new);

    let generator = Sha256Generator;
    let benchmark = Benchmark::new(
        &RV32_IM_RISC0_ZKVM_ELF,
        "risc0",
        Sha256::NAME,
        &Risc0Builder,
    )
    .unwrap();

    let mut results = Vec::new();
    for config in &configs {
        let (mut output, peak_memory) = measure_peak_memory(|| {
            benchmark
                .run::<Sha256, Sha256Config, Sha256Generator>(&generator, config)
                .unwrap()
        });
        output.1.peak_memory = peak_memory;
        results.push(output.1);
    }

    write_csv("risc0-sha256.csv", &results);
}

/// SHA256 SP1 benchmark.
fn sp1_sha256() {
    let configs = SHA2_INPUTS.map(Sha256Config::new);

    let generator = Sha256Generator;
    let benchmark =
        Benchmark::new(&RV32_IM_SUCCINCT_ZKVM_ELF, "sp1", Sha256::NAME, &Sp1Builder).unwrap();

    let mut results = Vec::new();
    for config in &configs {
        let (mut output, peak_memory) = measure_peak_memory(|| {
            benchmark
                .run::<Sha256, Sha256Config, Sha256Generator>(&generator, config)
                .unwrap()
        });
        output.1.peak_memory = peak_memory;
        results.push(output.1);
    }

    write_csv("sp1-sha256.csv", &results);
}

/// SHA256 Jolt benchmark.
fn jolt_sha256() {
    let configs = SHA2_INPUTS.map(Sha256Config::new);

    let generator = Sha256Generator;
    let benchmark = Benchmark::new(&JOLT_TARGET, "jolt", Sha256::NAME, &JoltBuilder).unwrap();

    let mut results = Vec::new();
    for config in &configs {
        let (mut output, peak_memory) = measure_peak_memory(|| {
            benchmark
                .run::<Sha256, Sha256Config, Sha256Generator>(&generator, config)
                .unwrap()
        });
        output.1.peak_memory = peak_memory;
        results.push(output.1);
    }

    write_csv("jolt-sha256.csv", &results);
}

fn main() {
    risc0_sha256();
    jolt_sha256();
    sp1_sha256();
}
