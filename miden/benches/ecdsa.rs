use ere_miden::compiler::MidenAsm;
use miden::{
    execution_cycles, miden_bench_properties, prepare_ecdsa, preprocessing_size, proof_size,
    prove_ecdsa, verify_ecdsa,
};
use utils::harness::ProvingSystem;
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_or_compile_program;

utils::define_benchmark_harness!(
    BenchTarget::Ecdsa,
    ProvingSystem::Miden,
    None,
    "ecdsa_mem_miden",
    miden_bench_properties(),
    { load_or_compile_program(&MidenAsm, ECDSA_BENCH) },
    |size, prog| prepare_ecdsa(size, prog).expect("prepare_ecdsa"),
    |_, _| 0,
    prove_ecdsa,
    |p, proof, s| verify_ecdsa(p, proof, s).expect("verify_ecdsa"),
    preprocessing_size,
    proof_size,
    execution_cycles
);
