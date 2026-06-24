mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the Stark-V private-transaction benchmark and returns prove time in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Matches CI: prepare is setup (not timed), only prove is measured.
/// Returns "prove_time_ms=<N>"
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
    use std::time::Instant;

    let program = load_compiled_from_path(std::path::Path::new(&compiled_program_path));
    // prepare is setup (not timed), matching CI's iter_batched harness
    let prepared = prepare_private_tx(input_size as usize, &program);

    let start = Instant::now();
    prove_bench(&prepared, &program);
    let prove_time_ms = start.elapsed().as_millis();

    println!("prove_time_ms: {}", prove_time_ms);

    format!("prove_time_ms={}", prove_time_ms)
}
