use noir::proof_size;
use noir::{prepare_sha256, preprocessing_size, prove, verify};
use utils::harness::BenchTarget;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Provekit,
    None,
    "sha256_mem",
    |input_size| { prepare_sha256(input_size) },
    |(input_size, toml_path, circuit_path)| { prove(*input_size, toml_path, circuit_path) },
    |(_, _, _), (proof_path, vk_path)| {
        verify(&proof_path, &vk_path).unwrap();
    },
    |(_, _, circuit_path)| { preprocessing_size(&circuit_path) },
    |(proof_path, _vk_path)| { proof_size(proof_path) }
);
