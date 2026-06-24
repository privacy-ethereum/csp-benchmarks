mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

#[derive(Clone, Copy)]
enum StarkVMobileBench {
    PrivateTx,
    ConstantOverhead,
    MerkleFake,
    HashSha256,
    MerkleSha256,
    HashKeccak,
    MerkleKeccak,
}

impl StarkVMobileBench {
    fn name(self) -> &'static str {
        match self {
            Self::PrivateTx => "private_tx",
            Self::ConstantOverhead => "constant_overhead",
            Self::MerkleFake => "merkle_fake",
            Self::HashSha256 => "hash_sha256",
            Self::MerkleSha256 => "merkle_sha256",
            Self::HashKeccak => "hash_keccak",
            Self::MerkleKeccak => "merkle_keccak",
        }
    }
}

/// Runs the Stark-V private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::PrivateTx,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_constant_overhead(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::ConstantOverhead,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_merkle_fake(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::MerkleFake,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_hash_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::HashSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_merkle_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::MerkleSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_hash_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::HashKeccak,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_merkle_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_stark_v_on_prover_thread(
        input_size,
        compiled_program_path,
        StarkVMobileBench::MerkleKeccak,
    )
}

fn run_stark_v_on_prover_thread(
    input_size: u64,
    compiled_program_path: String,
    bench: StarkVMobileBench,
) -> String {
    // Stark-V prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || stark_v_prove_inner(input_size, compiled_program_path, bench))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn stark_v_prove_inner(
    input_size: u64,
    compiled_program_path: String,
    bench: StarkVMobileBench,
) -> String {
    use stark_v_bench::{
        load_compiled_from_path, prepare_constant_overhead, prepare_hash_keccak,
        prepare_hash_sha256, prepare_merkle_fake, prepare_merkle_keccak, prepare_merkle_sha256,
        prepare_private_tx, prove_bench,
    };
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};

    let program = load_compiled_from_path(std::path::Path::new(&compiled_program_path));
    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        let prepared = match bench {
            StarkVMobileBench::PrivateTx => prepare_private_tx(input_size as usize, &program),
            StarkVMobileBench::ConstantOverhead => {
                prepare_constant_overhead(input_size as usize, &program)
            }
            StarkVMobileBench::MerkleFake => prepare_merkle_fake(input_size as usize, &program),
            StarkVMobileBench::HashSha256 => prepare_hash_sha256(input_size as usize, &program),
            StarkVMobileBench::MerkleSha256 => prepare_merkle_sha256(input_size as usize, &program),
            StarkVMobileBench::HashKeccak => prepare_hash_keccak(input_size as usize, &program),
            StarkVMobileBench::MerkleKeccak => prepare_merkle_keccak(input_size as usize, &program),
        };

        let start = Instant::now();
        prove_bench(&prepared, &program);
        let prove_time_ms = start.elapsed().as_millis();
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
