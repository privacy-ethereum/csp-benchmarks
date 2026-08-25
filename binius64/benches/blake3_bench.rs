use binius_utils::serialization::SerializeBytes;
use binius64::circuits::blake3::Blake3Params;
use binius64::prepare;
use binius64::{BINIUS64_BENCH_PROPERTIES, circuits::Blake3Circuit};

use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Blake3,
    ProvingSystem::Binius64,
    None,
    "blake3_mem_binius64",
    BINIUS64_BENCH_PROPERTIES,
    |_| None,
    |input_size| {
        prepare::<Blake3Circuit>(
            input_size,
            Blake3Params {
                max_len_bytes: Some(input_size),
            },
        )
        .expect("Failed to prepare BLAKE3 circuit for prove/verify")
    },
    |(_, _, cs, _, _, _)| {
        cs.n_zero_constraints()
            + cs.n_and_constraints()
            + cs.n_imul_constraints()
            + cs.n_bmul_constraints()
    },
    |(_verifier, prover, _cs, circuit, compiled_circuit, input_size)| {
        binius64::prove::<Blake3Circuit>(prover, compiled_circuit, circuit, *input_size)
            .expect("Failed to prove BLAKE3 circuit")
    },
    |(verifier, _prover, _cs, _circuit, _compiled_circuit, _input_size), (proof, pub_witness)| {
        binius64::verify(verifier, pub_witness, proof).expect("Failed to verify BLAKE3 circuit")
    },
    |(_verifier, _prover, cs, _circuit, _compiled_circuit, _input_size)| {
        let mut buf: Vec<u8> = Vec::new();
        cs.serialize(&mut buf)
            .expect("Failed to serialize constraint system into byte array");
        buf.len()
    },
    |(proof, _pub_witness)| proof.len()
);
