use provekit_groth16_bench::{
    num_constraints, prepare_poseidon2, preprocessing_size, proof_size, prove, verify,
    PROVEKIT_GROTH16_PROPS,
};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Poseidon2,
    ProvingSystem::ProvekitGroth16,
    None,
    "poseidon2_mem_provekit_groth16",
    PROVEKIT_GROTH16_PROPS,
    prepare_poseidon2,
    |prepared| { num_constraints(prepared) },
    |prepared| { prove(prepared) },
    |prepared, proof| {
        verify(prepared, proof).unwrap();
    },
    |prepared| { preprocessing_size(prepared) },
    |proof| { proof_size(proof) }
);
