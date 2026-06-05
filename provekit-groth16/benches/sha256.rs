use provekit_groth16_bench::{
    PROVEKIT_GROTH16_PROPS, num_constraints, prepare_sha256, preprocessing_size, proof_size, prove,
    verify,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::ProvekitGroth16,
    None,
    "sha256_mem_provekit_groth16",
    PROVEKIT_GROTH16_PROPS,
    |_| true,
    prepare_sha256,
    |prepared| { num_constraints(prepared) },
    |prepared| { prove(prepared) },
    |prepared, proof| {
        verify(prepared, proof).unwrap();
    },
    |prepared| { preprocessing_size(prepared) },
    |proof| { proof_size(proof) }
);
