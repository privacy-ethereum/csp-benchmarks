use anyhow::{Result, ensure};
use rand::{RngCore, SeedableRng, rngs::StdRng};

use binius_frontend::{CircuitBuilder, Wire, WitnessFiller};
use binius_prover::{
    OptimalPackedB128, Prover, hash::parallel_compression::ParallelCompressionAdaptor,
};
use binius_verifier::{
    Verifier,
    hash::{StdCompression, StdDigest},
};

use binius_circuits::sha256::Sha256;
use clap::Args;
use sha2::Digest;

use std::array;

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs
pub struct Sha256Circuit {
    sha256_gadget: Sha256,
}

impl CircuitTrait for Sha256Circuit {
    type Params = Params;
    type Instance = Instance;

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

    fn populate_witness(&self, instance: Instance, w: &mut WitnessFiller) -> Result<()> {
        // Step 1: Get raw message bytes
        let raw_message = generate_message_bytes(instance.message_string, instance.message_len);

        // Step 2: Zero-pad to maximum length
        let padded_message = zero_pad_message(raw_message, self.sha256_gadget.max_len_bytes())?;

        // Step 3: Compute digest using reference implementation
        let digest = sha2::Sha256::digest(&padded_message);

        // Step 4: Populate witness values
        self.sha256_gadget
            .populate_len_bytes(w, padded_message.len());
        self.sha256_gadget.populate_message(w, &padded_message);
        self.sha256_gadget.populate_digest(w, digest.into());

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

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs
fn mk_circuit(b: &mut CircuitBuilder, max_len: usize, len_bytes: Wire) -> Sha256 {
    let digest: [Wire; 4] = array::from_fn(|_| b.add_inout());
    let message = (0..max_len).map(|_| b.add_inout()).collect();
    Sha256::new(b, len_bytes, digest, message)
}

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs
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

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/sha256.rs
#[derive(Args, Debug, Clone)]
#[group(multiple = false)]
pub struct Instance {
    /// Length of the randomly generated message, in bytes (defaults to 1024).
    #[arg(long)]
    pub message_len: Option<usize>,

    /// UTF-8 string to hash (if not provided, random bytes are generated)
    #[arg(long)]
    pub message_string: Option<String>,
}

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/lib.rs
pub type StdVerifier = Verifier<StdDigest, StdCompression>;

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/lib.rs
pub type StdProver =
    Prover<OptimalPackedB128, ParallelCompressionAdaptor<StdCompression>, StdDigest>;

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/lib.rs
pub trait CircuitTrait: Sized {
    /// Circuit parameters that affect the structure of the circuit.
    /// These are typically compile-time constants or bounds.
    type Params: clap::Args;

    /// Instance data used to populate the witness.
    /// This represents the actual input values for a specific proof.
    type Instance: clap::Args;

    /// Build the circuit with the given parameters.
    ///
    /// This method should:
    /// - Add witnesses, constants, and constraints to the builder
    /// - Store any wire references needed for witness population
    /// - Return a Self instance that can later populate witness values
    fn build(params: Self::Params, builder: &mut CircuitBuilder) -> Result<Self>;

    /// Populate witness values for a specific instance.
    ///
    /// This method should:
    /// - Process the instance data (e.g., parse inputs, compute hashes)
    /// - Fill all witness values using the provided filler
    /// - Validate that instance data is compatible with circuit parameters
    fn populate_witness(&self, instance: Self::Instance, filler: &mut WitnessFiller) -> Result<()>;

    /// Generate a concise parameter summary for perfetto trace filenames.
    ///
    /// This method should return a short string (5-10 chars max) that captures
    /// the most important parameters for this circuit configuration.
    /// Used to differentiate traces with different parameter settings.
    ///
    /// Format suggestions:
    /// - Bytes: "2048b", "4096b"
    /// - Counts: "10p" (permutations), "5s" (signatures)
    ///
    /// Returns None if no meaningful parameters to include in filename.
    #[allow(dead_code)]
    fn param_summary(params: &Self::Params) -> Option<String> {
        let _ = params;
        None
    }
}

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/utils.rs
pub const DEFAULT_HASH_MESSAGE_BYTES: usize = 1024;

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/utils.rs
pub const DEFAULT_RANDOM_SEED: u64 = 42;

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/utils.rs
pub fn determine_hash_max_bytes_from_args(max_bytes_param: Option<usize>) -> Result<usize> {
    let max_bytes = max_bytes_param.unwrap_or_else(|| {
        let args: Vec<String> = std::env::args().collect();
        let mut message_len = None;
        let mut message_string = None;

        for i in 0..args.len() {
            if args[i] == "--message-len" && i + 1 < args.len() {
                message_len = args[i + 1].parse::<usize>().ok();
            } else if args[i] == "--message-string" && i + 1 < args.len() {
                message_string = Some(args[i + 1].clone());
            }
        }

        if let Some(msg_string) = message_string {
            msg_string.len()
        } else {
            message_len.unwrap_or(DEFAULT_HASH_MESSAGE_BYTES)
        }
    });

    ensure!(max_bytes > 0, "Message length must be positive");
    Ok(max_bytes)
}

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/utils.rs
pub fn generate_message_bytes(
    message_string: Option<String>,
    message_len: Option<usize>,
) -> Vec<u8> {
    if let Some(message_string) = message_string {
        message_string.as_bytes().to_vec()
    } else {
        let mut rng = StdRng::seed_from_u64(DEFAULT_RANDOM_SEED);
        let len = message_len.unwrap_or(DEFAULT_HASH_MESSAGE_BYTES);
        let mut message_bytes = vec![0u8; len];
        rng.fill_bytes(&mut message_bytes);
        message_bytes
    }
}

// Reference: https://github.com/IrreducibleOSS/binius64/blob/main/prover/examples/src/circuits/utils.rs
pub fn zero_pad_message(message_bytes: Vec<u8>, max_len: usize) -> Result<Vec<u8>> {
    ensure!(
        message_bytes.len() <= max_len,
        "Message length ({}) exceeds maximum ({})",
        message_bytes.len(),
        max_len
    );

    let mut padded = message_bytes;
    padded.resize(max_len, 0);
    Ok(padded)
}
