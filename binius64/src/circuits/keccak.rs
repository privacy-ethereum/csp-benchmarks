// Copyright 2025 Irreducible Inc.
// Reference https://github.com/binius-zk/binius64/blob/main/crates/examples/src/circuits/keccak.rs
use anyhow::Result;
use binius_circuits::keccak::{Keccak256, N_WORDS_PER_DIGEST};
use binius_frontend::{CircuitBuilder, Wire, WitnessFiller};
use clap::Args;

use crate::utils::{
    CircuitTrait, DEFAULT_HASH_MESSAGE_BYTES, determine_hash_max_bytes_from_args, zero_pad_message,
};
use utils::generate_keccak_input;

pub type KeccakParams = <KeccakCircuit as CircuitTrait>::Params;
pub type KeccakInstance = <KeccakCircuit as CircuitTrait>::Instance;

/// Keccak-256 hash circuit example
pub struct KeccakCircuit {
    keccak_hash: Keccak256,
    max_len_bytes: usize,
}

#[derive(Args, Debug, Clone)]
pub struct Params {
    /// Maximum message length in bytes that the circuit can handle
    #[arg(long)]
    pub max_len_bytes: Option<usize>,
}

impl CircuitTrait for KeccakCircuit {
    type Params = Params;
    type Instance = usize;

    fn build(params: Params, builder: &mut CircuitBuilder) -> Result<Self> {
        let max_len_bytes = determine_hash_max_bytes_from_args(params.max_len_bytes)?;

        let len_bytes = builder.add_witness();
        let digest: [Wire; N_WORDS_PER_DIGEST] = std::array::from_fn(|_| builder.add_inout());

        let n_words = max_len_bytes.div_ceil(8);
        let message = (0..n_words).map(|_| builder.add_inout()).collect();

        let keccak = Keccak256::new(builder, len_bytes, digest, message);

        Ok(Self {
            keccak_hash: keccak,
            max_len_bytes,
        })
    }

    fn populate_witness(&self, message_len_bytes: usize, w: &mut WitnessFiller) -> Result<()> {
        // Step 1: Generate a deterministic message and its Keccak-256 digest using shared utils
        let (message_bytes, digest_bytes) = generate_keccak_input(message_len_bytes);

        // Step 2: Zero-pad to maximum length
        let padded_message = zero_pad_message(message_bytes, self.max_len_bytes)?;

        // Step 3: Convert digest bytes to fixed-size array
        let digest: [u8; 32] = digest_bytes
            .as_slice()
            .try_into()
            .expect("Keccak-256 digest must be 32 bytes");

        // Step 4: Populate witness values
        self.keccak_hash.populate_len_bytes(w, padded_message.len());
        self.keccak_hash.populate_message(w, &padded_message);
        self.keccak_hash.populate_digest(w, digest);

        Ok(())
    }

    fn param_summary(params: &Self::Params) -> Option<String> {
        Some(format!(
            "{}b",
            params.max_len_bytes.unwrap_or(DEFAULT_HASH_MESSAGE_BYTES)
        ))
    }
}
