mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the Jolt private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Returns comma-separated summary fields, including all raw samples.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn jolt_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    // Jolt's prover uses deep recursion (sumcheck, polynomial commitments) that
    // overflows the 512 KB default iOS thread stack. Spawn a dedicated thread
    // with 64 MB of stack so the prover has enough room.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || jolt_prove_private_tx_inner(input_size, compiled_program_path))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn jolt_prove_private_tx_inner(input_size: u64, compiled_program_path: String) -> String {
    use ere_jolt::compiler::RustRv64imacCustomized;
    use jolt_bench::{prepare_private_tx, prove_private_tx};
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};
    use utils::zkvm::load_compiled_program_from_path;

    let program = load_compiled_program_from_path::<RustRv64imacCustomized>(std::path::Path::new(
        &compiled_program_path,
    ));

    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        let prepared = prepare_private_tx(input_size as usize, &program);

        let start = Instant::now();
        prove_private_tx(&prepared, &());
        let prove_time_ms = start.elapsed().as_millis();
        println!("sample_{}_prove_time_ms: {}", sample + 1, prove_time_ms);
        samples.push(prove_time_ms);

        if sample + 1 != MOBILE_SAMPLE_COUNT {
            std::thread::sleep(Duration::from_secs(MOBILE_BREAK_SECS));
        }
    }

    let summary = format_prove_ms_summary(&samples);
    println!("{}", summary);
    summary
}
