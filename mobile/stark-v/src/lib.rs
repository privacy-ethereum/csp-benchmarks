mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the Stark-V private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Matches CI: prepare is setup (not timed), only prove is measured.
/// Returns comma-separated summary fields, including all raw samples.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn stark_v_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    // Stark-V prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || stark_v_prove_private_tx_inner(input_size, compiled_program_path))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn stark_v_prove_private_tx_inner(input_size: u64, compiled_program_path: String) -> String {
    use stark_v_bench::{load_compiled_from_path, prepare_private_tx, prove_bench};
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};

    let program = load_compiled_from_path(std::path::Path::new(&compiled_program_path));
    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        let prepared = prepare_private_tx(input_size as usize, &program);

        let start = Instant::now();
        prove_bench(&prepared, &program);
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
