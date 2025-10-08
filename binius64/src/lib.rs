use anyhow::Result;
use binius_core::constraint_system::{ConstraintSystem, ValueVec};
use binius_frontend::CircuitBuilder;
use binius_prover::{
    KeyCollection, OptimalPackedB128, Prover,
    hash::{
        ParallelDigest,
        parallel_compression::{ParallelCompressionAdaptor, ParallelPseudoCompression},
    },
};
use binius_verifier::{
    Verifier,
    config::StdChallenger,
    hash::{PseudoCompressionFunction, StdCompression},
    transcript::{ProverTranscript, VerifierTranscript},
};
use sha2::digest::{Digest, FixedOutputReset, Output, core_api::BlockSizeUser};

use binius_examples::{
    ExampleCircuit, StdProver, StdVerifier,
    circuits::sha256::{Instance, Params, Sha256Example},
};

/// Setup the prover and verifier and use SHA256 for Merkle tree compression.
/// Providing the `key_collection` skips expensive key collection building.
fn setup_sha256(
    cs: ConstraintSystem,
    log_inv_rate: usize,
    key_collection: Option<KeyCollection>,
) -> Result<(StdVerifier, StdProver)> {
    let parallel_compression = ParallelCompressionAdaptor::new(StdCompression::default());
    let compression = parallel_compression.compression().clone();
    let verifier = Verifier::setup(cs, log_inv_rate, compression)?;
    let prover = if let Some(key_collection) = key_collection {
        Prover::setup_with_key_collection(verifier.clone(), parallel_compression, key_collection)?
    } else {
        Prover::setup(verifier.clone(), parallel_compression)?
    };
    Ok((verifier, prover))
}

// Use the default configs/params
pub fn prepare(input_size: usize) -> Result<(StdVerifier, StdProver, ValueVec, ConstraintSystem)> {
    // Extract common arguments
    let log_inv_rate = 1;
    // let compression = CompressionType::Sha256;

    // Parse Params and Instance from matches
    let params = Params {
        max_len_bytes: Some(input_size),
        exact_len: true,
    };
    let instance = Instance {
        message_len: Some(input_size),
        message_string: None,
    };

    // Build the circuit
    let mut builder = CircuitBuilder::new();
    let example = Sha256Example::build(params, &mut builder)?;
    let circuit = builder.build();

    // Set up prover and verifier
    let cs = circuit.constraint_system().clone();

    // Population of the input to the witness and then evaluating the circuit.
    let mut filler = circuit.new_witness_filler();
    example.populate_witness(instance, &mut filler)?; // input population
    circuit.populate_wire_witness(&mut filler)?; // circuit evaluation
    let witness = filler.into_value_vec();

    // Using SHA256 compression for Merkle tree
    let (verifier, prover) = setup_sha256(cs.clone(), log_inv_rate as usize, None)?;

    Ok((verifier, prover, witness, cs))
}

pub fn prove<D, C, PC>(
    prover: &Prover<OptimalPackedB128, PC, D>,
    witness: ValueVec,
) -> Result<Vec<u8>>
where
    D: ParallelDigest + Digest + BlockSizeUser,
    D::Digest: BlockSizeUser + FixedOutputReset,
    C: PseudoCompressionFunction<Output<D>, 2>,
    PC: ParallelPseudoCompression<Output<D::Digest>, 2>,
{
    let challenger = StdChallenger::default();

    let mut prover_transcript = ProverTranscript::new(challenger);
    prover.prove(witness.clone(), &mut prover_transcript)?;

    let proof = prover_transcript.finalize();

    Ok(proof)
}

pub fn verify<D, C, PC>(verifier: &Verifier<D, C>, witness: ValueVec, proof: &[u8]) -> Result<()>
where
    D: ParallelDigest + Digest + BlockSizeUser,
    D::Digest: BlockSizeUser + FixedOutputReset,
    C: PseudoCompressionFunction<Output<D>, 2>,
    PC: ParallelPseudoCompression<Output<D::Digest>, 2>,
{
    let challenger = StdChallenger::default();
    let mut verifier_transcript = VerifierTranscript::new(challenger, proof.to_vec());
    verifier.verify(witness.public(), &mut verifier_transcript)?;
    verifier_transcript.finalize()?;

    Ok(())
}
