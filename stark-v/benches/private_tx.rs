use stark_v::{
    execution_cycles, load_or_compile, prepare_private_tx, preprocessing_size, proof_size,
    prove_bench, stark_v_bench_properties, verify_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::PrivateTx,                                            // target
    ProvingSystem::StarkV,                                             // system
    None,                                                              // feature
    "private_tx_mem_stark_v",                                          // mem_binary_name
    stark_v_bench_properties(),                                        // properties
    |_| false,                                                         // uses_precompile
    { load_or_compile("private_tx") },                                 // shared_init
    prepare_private_tx,                                                // prepare
    |_, _| 0,                                                          // num_constraints
    |prepared, program| prove_bench(prepared, program),                // prove
    |prepared, proof, program| verify_bench(prepared, proof, program), // verify
    |prepared, program| preprocessing_size(prepared, program),         // prep_size
    |proof, program| proof_size(proof, program),                       // proof_size
    execution_cycles                                                   // execution_cycles
);
