use circom::prepare;
use circom_prover::prover::CircomProof;
use utils::harness::BenchTarget;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Circom,
    None,
    "sha256_mem",
    |input_size| { prepare(input_size) },
    |(witness_fn, input_str, zkey_path)| {
        circom::prove(witness_fn.clone(), input_str.clone(), zkey_path.clone())
    },
    |(_witness_fn, _input_str, zkey_path), proof| { circom::verify(proof.clone(), zkey_path.clone(),) },
    |(_witness_fn, _input_str, _zkey_path)| { 0 },
    |_proof: &CircomProof| 0
);
