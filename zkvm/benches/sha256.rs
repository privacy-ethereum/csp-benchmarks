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
    traits::{InputBuilder, Program, ZkVMBuilder},
};
use zkvm_interface::{Compiler, zkVM};

/// Runs the SHA256 benchmark for the given zkVM.
fn sha256_benchmark<C, V, B>(compiler: &C, vm_builder: &B, vm_name: &'static str)
where
    C: Compiler,
    V: zkVM + InputBuilder<Sha256, Data = Vec<u8>>,
    B: ZkVMBuilder<C, V>,
{
    let configs = SHA2_INPUTS.map(Sha256Config::new);
    let benchmark = Benchmark::new(compiler, vm_name, Sha256::NAME, vm_builder).unwrap();

    let mut results = Vec::new();
    for config in &configs {
        let (mut output, peak_memory) = measure_peak_memory(|| {
            benchmark
                .run::<Sha256, _, _>(&Sha256Generator, config)
                .unwrap()
        });
        output.1.peak_memory = peak_memory;
        results.push(output.1);
    }

    write_csv(&format!("{}-sha256.csv", vm_name), &results);
}

fn main() {
    sha256_benchmark(&RV32_IM_RISC0_ZKVM_ELF, &Risc0Builder, "risc0");
    sha256_benchmark(&RV32_IM_SUCCINCT_ZKVM_ELF, &Sp1Builder, "sp1");
    sha256_benchmark(&JOLT_TARGET, &JoltBuilder, "jolt");
}
