mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the Jolt private-transaction benchmark and returns timing in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Returns "prove_time_ms=<N>"
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
    use std::time::Instant;
    use utils::zkvm::load_compiled_program_from_path;

    let program = load_compiled_program_from_path::<RustRv64imacCustomized>(std::path::Path::new(
        &compiled_program_path,
    ));

    // prepare is setup (not timed), matching CI's iter_batched harness
    let prepared = prepare_private_tx(input_size as usize, &program);

    let start = Instant::now();
    prove_private_tx(&prepared, &());
    let prove_time_ms = start.elapsed().as_millis();

    println!("prove_time_ms: {}", prove_time_ms);

    format!("prove_time_ms={}", prove_time_ms)
}
