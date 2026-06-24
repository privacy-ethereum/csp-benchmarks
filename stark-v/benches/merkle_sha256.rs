use stark_v::{
    execution_cycles, load_or_compile, prepare_merkle_sha256, preprocessing_size, proof_size,
    prove_bench, stark_v_bench_properties, verify_bench,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::MerkleSha256,
    ProvingSystem::StarkV,
    None,
    "merkle_sha256_mem_stark_v",
    stark_v_bench_properties(),
    |_| false,
    { load_or_compile("merkle_sha256") },
    prepare_merkle_sha256,
    |_, _| 0,
    |prepared, program| prove_bench(prepared, program),
    |prepared, proof, program| verify_bench(prepared, proof, program),
    |prepared, program| preprocessing_size(prepared, program),
    |proof, program| proof_size(proof, program),
    execution_cycles
);
