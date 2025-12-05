// Copyright 2025 Irreducible Inc.
// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs
use anyhow::Result;

use binius_frontend::{CircuitBuilder, Wire, WitnessFiller};

use binius_circuits::sha256::Sha256;
use clap::Args;

use std::array;

use crate::utils::{
    CircuitTrait, DEFAULT_HASH_MESSAGE_BYTES, determine_hash_max_bytes_from_args, zero_pad_message,
};
use utils::generate_sha256_input;

pub type Sha256Params = <Sha256Circuit as CircuitTrait>::Params;
pub type Sha256Instance = <Sha256Circuit as CircuitTrait>::Instance;

pub struct Sha256Circuit {
    sha256_gadget: Sha256,
}

impl CircuitTrait for Sha256Circuit {
    type Params = Params;
    type Instance = usize;

    fn build(params: Params, builder: &mut CircuitBuilder) -> Result<Self> {
        let max_len_bytes = determine_hash_max_bytes_from_args(params.max_len_bytes)?;
        let max_len = max_len_bytes.div_ceil(8);
        let len_bytes = if params.exact_len {
            builder.add_constant_64(max_len_bytes as u64)
        } else {
            builder.add_witness()
        };
        let sha256_gadget = mk_circuit(builder, max_len, len_bytes);

        Ok(Self { sha256_gadget })
    }

    fn populate_witness(&self, message_len_bytes: usize, w: &mut WitnessFiller) -> Result<()> {
        // Step 1: Generate a deterministic message and its SHA-256 digest using shared utils
        let (message_bytes, digest_bytes) = generate_sha256_input(message_len_bytes);

        // Step 2: Zero-pad to maximum length
        let padded_message = zero_pad_message(message_bytes, self.sha256_gadget.max_len_bytes())?;

        // Step 3: Convert digest bytes to fixed-size array
        let digest: [u8; 32] = digest_bytes
            .as_slice()
            .try_into()
            .expect("SHA-256 digest must be 32 bytes");

        // Step 4: Populate witness values
        self.sha256_gadget
            .populate_len_bytes(w, padded_message.len());
        self.sha256_gadget.populate_message(w, &padded_message);
        self.sha256_gadget.populate_digest(w, digest);

        Ok(())
    }

    fn param_summary(params: &Self::Params) -> Option<String> {
        let base = format!(
            "{}b",
            params.max_len_bytes.unwrap_or(DEFAULT_HASH_MESSAGE_BYTES)
        );
        if params.exact_len {
            Some(format!("{}-exact", base))
        } else {
            Some(base)
        }
    }
}

fn mk_circuit(b: &mut CircuitBuilder, max_len: usize, len_bytes: Wire) -> Sha256 {
    let digest: [Wire; 4] = array::from_fn(|_| b.add_inout());
    let message = (0..max_len).map(|_| b.add_inout()).collect();
    Sha256::new(b, len_bytes, digest, message)
}

#[derive(Args, Debug, Clone)]
pub struct Params {
    /// Maximum message length in bytes that the circuit can handle.
    #[arg(long)]
    pub max_len_bytes: Option<usize>,

    /// Build circuit for exact message length (makes length a compile-time constant instead of
    /// runtime witness).
    #[arg(long, default_value_t = false)]
    pub exact_len: bool,
}
