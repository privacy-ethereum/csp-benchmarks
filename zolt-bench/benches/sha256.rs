use utils::harness::{BenchTarget, ProvingSystem};
use zolt_bench::{
    prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
    zolt_bench_properties, PreparedZoltSha256, ZoltProofResult, ZoltSharedState,
};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Zolt,
    None,
    "sha256_mem_zolt",
    zolt_bench_properties(),
    { ZoltSharedState::new() },
    |size, shared: &ZoltSharedState| prepare_sha256(size, *shared),
    |_: &PreparedZoltSha256, _: &&ZoltSharedState| 0usize,
    |prepared: &PreparedZoltSha256, shared: &&ZoltSharedState| prove_sha256(prepared, shared),
    |prepared: &PreparedZoltSha256, proof: &ZoltProofResult, shared: &&ZoltSharedState| verify_sha256(prepared, proof, shared),
    |prepared: &PreparedZoltSha256, shared: &&ZoltSharedState| preprocessing_size(prepared, shared),
    |proof: &ZoltProofResult, shared: &&ZoltSharedState| proof_size(proof, shared)
);
