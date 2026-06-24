mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the LeanVM private-transaction benchmark and returns prove time in milliseconds.
/// Matches CI: prepare is setup (not timed), only prove is measured.
/// Returns "prove_time_ms=<N>"
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_private_tx(input_size: u64) -> String {
    // LeanVM's WHIR prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || leanvm_prove_private_tx_inner(input_size))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn leanvm_prove_private_tx_inner(input_size: u64) -> String {
    use leanvm_bench::{compile_private_tx, prepare_private_tx, prove_private_tx};
    use std::time::Instant;

    // compile_private_tx uses include_str! — no CARGO_MANIFEST_DIR needed
    let bytecode = compile_private_tx();
    // prepare is setup (not timed), matching CI's iter_batched harness
    let prepared = prepare_private_tx(input_size as usize, &bytecode);

    let start = Instant::now();
    prove_private_tx(&prepared, &());
    let prove_time_ms = start.elapsed().as_millis();

    println!("prove_time_ms: {}", prove_time_ms);

    format!("prove_time_ms={}", prove_time_ms)
}
