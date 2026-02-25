use stark_v::stark_v_bench_properties;
use stark_v::{
    execution_cycles, prepare_keccak, preprocessing_size, proof_size, prove, verify_keccak,
};
use stark_v_sdk::StarkVCompiler;
use utils::harness::ProvingSystem;
use utils::zkvm::helpers::load_or_compile_program;
use utils::zkvm::KECCAK_BENCH;

utils::define_benchmark_harness!(
    BenchTarget::Keccak,                                               // target
    ProvingSystem::StarkV,                                             // system
    None,                                                              // feature
    "keccak_mem_stark_v",                                              // mem_binary_name
    stark_v_bench_properties(),                                        // properties
    { load_or_compile_program(&StarkVCompiler::new(), KECCAK_BENCH) }, // shared_init
    prepare_keccak,                                                    // prepare
    |_, _| 0,           // num_constraints (components.n_constraints())
    prove,              // prove
    verify_keccak,      // verify
    preprocessing_size, // prep_size
    proof_size,         // proof_size
    execution_cycles    // execution_cycles
);
