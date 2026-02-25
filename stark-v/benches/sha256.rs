use stark_v::stark_v_bench_properties;
use stark_v::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use stark_v_sdk::StarkVCompiler;
use utils::harness::ProvingSystem;
use utils::zkvm::helpers::load_or_compile_program;
use utils::zkvm::SHA256_BENCH;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,                                               // target
    ProvingSystem::StarkV,                                             // system
    None,                                                              // feature
    "sha256_mem_stark_v",                                              // mem_binary_name
    stark_v_bench_properties(),                                        // properties
    { load_or_compile_program(&StarkVCompiler::new(), SHA256_BENCH) }, // shared_init
    prepare_sha256,                                                    // prepare
    |_, _| 0,           // num_constraints (components.n_constraints())
    prove_sha256,       // prove
    verify_sha256,      // verify
    preprocessing_size, // prep_size
    proof_size,         // proof_size
    execution_cycles    // execution_cycles
);
