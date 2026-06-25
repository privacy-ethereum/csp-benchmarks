mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

static BLAKE3_INLINE_INIT: std::sync::Once = std::sync::Once::new();

#[derive(Clone, Copy)]
enum JoltMobileBench {
    PrivateTx,
    ConstantOverhead,
    MerkleFake,
    HashSha256,
    MerkleSha256,
    HashKeccak,
    MerkleKeccak,
    HashBlake3,
    MerkleBlake3,
}

impl JoltMobileBench {
    fn name(self) -> &'static str {
        match self {
            Self::PrivateTx => "private_tx",
            Self::ConstantOverhead => "constant_overhead",
            Self::MerkleFake => "merkle_fake",
            Self::HashSha256 => "hash_sha256",
            Self::MerkleSha256 => "merkle_sha256",
            Self::HashKeccak => "hash_keccak",
            Self::MerkleKeccak => "merkle_keccak",
            Self::HashBlake3 => "hash_blake3",
            Self::MerkleBlake3 => "merkle_blake3",
        }
    }
}

/// Runs the Jolt private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::PrivateTx,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_constant_overhead(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::ConstantOverhead,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_merkle_fake(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::MerkleFake,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_hash_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::HashSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_merkle_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::MerkleSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_hash_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::HashKeccak,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_merkle_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::MerkleKeccak,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_hash_blake3(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::HashBlake3,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_merkle_blake3(input_size: u64, compiled_program_path: String) -> String {
    run_jolt_on_prover_thread(
        input_size,
        compiled_program_path,
        JoltMobileBench::MerkleBlake3,
    )
}

fn run_jolt_on_prover_thread(
    input_size: u64,
    compiled_program_path: String,
    bench: JoltMobileBench,
) -> String {
    // Jolt's prover uses deep recursion (sumcheck, polynomial commitments) that
    // overflows the 512 KB default iOS thread stack. Spawn a dedicated thread
    // with 64 MB of stack so the prover has enough room.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || jolt_prove_inner(input_size, compiled_program_path, bench))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn ensure_blake3_inlines_registered() {
    BLAKE3_INLINE_INIT.call_once(|| {
        if let Err(error) = jolt_inlines_blake3::init_inlines() {
            assert!(
                error.contains("already registered"),
                "failed to register Jolt BLAKE3 inlines: {error}"
            );
        }
    });
}

fn jolt_prove_inner(
    input_size: u64,
    compiled_program_path: String,
    bench: JoltMobileBench,
) -> String {
    use ere_jolt::compiler::RustRv64imacCustomized;
    use jolt_bench::{
        prepare_constant_overhead, prepare_hash_blake3, prepare_hash_keccak, prepare_hash_sha256,
        prepare_merkle_blake3, prepare_merkle_fake, prepare_merkle_keccak, prepare_merkle_sha256,
        prepare_private_tx, prove_private_tx, prove_targeted,
    };
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};
    use utils::zkvm::load_compiled_program_from_path;

    let program = load_compiled_program_from_path::<RustRv64imacCustomized>(std::path::Path::new(
        &compiled_program_path,
    ));

    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        let prove_time_ms = match bench {
            JoltMobileBench::PrivateTx => {
                let prepared = prepare_private_tx(input_size as usize, &program);
                let start = Instant::now();
                prove_private_tx(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::ConstantOverhead => {
                let prepared = prepare_constant_overhead(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::MerkleFake => {
                let prepared = prepare_merkle_fake(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::HashSha256 => {
                let prepared = prepare_hash_sha256(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::MerkleSha256 => {
                let prepared = prepare_merkle_sha256(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::HashKeccak => {
                let prepared = prepare_hash_keccak(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::MerkleKeccak => {
                let prepared = prepare_merkle_keccak(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::HashBlake3 => {
                ensure_blake3_inlines_registered();
                let prepared = prepare_hash_blake3(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
            JoltMobileBench::MerkleBlake3 => {
                ensure_blake3_inlines_registered();
                let prepared = prepare_merkle_blake3(input_size as usize, &program);
                let start = Instant::now();
                prove_targeted(&prepared, &());
                start.elapsed().as_millis()
            }
        };
        println!(
            "{}_sample_{}_prove_time_ms: {}",
            bench.name(),
            sample + 1,
            prove_time_ms
        );
        samples.push(prove_time_ms);

        if sample + 1 != MOBILE_SAMPLE_COUNT {
            std::thread::sleep(Duration::from_secs(MOBILE_BREAK_SECS));
        }
    }

    let summary = format_prove_ms_summary(&samples);
    println!("{}", summary);
    summary
}
