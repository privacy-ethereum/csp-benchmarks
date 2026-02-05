use spartan2::provider::T256HyraxEngine;
use std::borrow::Cow;

pub type E = T256HyraxEngine;
pub type Scalar = <E as spartan2::traits::Engine>::Scalar;

pub mod circuits;

use circuits::sha256_circuit::Sha256Circuit;
use spartan2::{spartan::SpartanSNARK, traits::snark::R1CSSNARKTrait};
use utils::generate_sha256_input;
use utils::harness::{AuditStatus, BenchProperties};

pub const SPARTAN2_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Spartan2"),
    field_curve: Cow::Borrowed("P256"),
    iop: Cow::Borrowed("Spartan"),
    pcs: Some(Cow::Borrowed("Hyrax")),
    arithm: Cow::Borrowed("R1CS"),
    is_zk: true, //https://github.com/microsoft/Spartan2/pull/73
    is_zkvm: false,
    security_bits: 128,
    is_pq: false,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

/// Prepared context for SHA256 benchmark
pub struct PreparedSha256 {
    circuit: Sha256Circuit,
    pk: <SpartanSNARK<E> as R1CSSNARKTrait<E>>::ProverKey,
    vk: <SpartanSNARK<E> as R1CSSNARKTrait<E>>::VerifierKey,
}

/// Prepare SHA256 circuit for benchmarking
pub fn prepare_sha256(input_size: usize) -> PreparedSha256 {
    // Generate SHA256 inputs
    let (preimage, _digest) = generate_sha256_input(input_size);

    // Create circuit
    let circuit = Sha256Circuit::new(preimage);

    // Setup keys
    let (pk, vk) = SpartanSNARK::<E>::setup(circuit.clone()).expect("setup failed");

    PreparedSha256 { circuit, pk, vk }
}

/// Generate proof for SHA256 circuit
pub fn prove_sha256(prepared: &PreparedSha256) -> SpartanSNARK<E> {
    // Prepare the SNARK
    let prep_snark = SpartanSNARK::<E>::prep_prove(&prepared.pk, prepared.circuit.clone(), true)
        .expect("prep_prove failed");

    // Generate proof
    SpartanSNARK::<E>::prove(&prepared.pk, prepared.circuit.clone(), &prep_snark, true)
        .expect("Failed to generate proof")
}

/// Verify proof for SHA256 circuit
pub fn verify_sha256(_prepared: &PreparedSha256, proof: &SpartanSNARK<E>) {
    proof.verify(&_prepared.vk).expect("Verification failed");
}

/// Get number of constraints
pub fn num_constraints(prepared: &PreparedSha256) -> usize {
    // Get number of constraints from the proving key's sizes
    // sizes() returns [num_cons_unpadded, num_shared_unpadded, num_precommitted_unpadded, num_rest_unpadded,
    //                  num_cons, num_shared, num_precommitted, num_rest, num_public, num_challenges]
    let sizes = prepared.pk.sizes();
    sizes[4] // num_cons (padded)
}

/// Get preprocessing size (proving key size)
pub fn preprocessing_size(prepared: &PreparedSha256) -> usize {
    bincode::serialize(&prepared.pk)
        .map(|bytes| bytes.len())
        .unwrap_or(0)
}

/// Get proof size
pub fn proof_size(proof: &SpartanSNARK<E>) -> usize {
    bincode::serialize(proof)
        .map(|bytes| bytes.len())
        .unwrap_or(0)
}
