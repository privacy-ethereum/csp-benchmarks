#[macro_use]
mod stubs;

mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();
// Skip wasm_setup!() to avoid extern crate alias conflict
// Instead, we import wasm_bindgen directly when needed
#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
use mopro_ffi::prelude::wasm_bindgen;

/// You can also customize the bindings by #[uniffi::export]
/// Reference: https://mozilla.github.io/uniffi-rs/latest/proc_macro/index.html
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn mopro_hello_world() -> String {
    "Hello, World!".to_string()
}

/// Runs the Jolt private-transaction benchmark and returns timing in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Returns a comma-separated string: "prepare_time_ms=<N>,prove_time_ms=<N>"
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
    use std::time::Instant;
    use ere_jolt::compiler::RustRv64imacCustomized;
    use jolt_bench::{prepare_private_tx, prove_private_tx};
    use utils::zkvm::load_compiled_program_from_path;

    let program = load_compiled_program_from_path::<RustRv64imacCustomized>(
        std::path::Path::new(&compiled_program_path),
    );

    // prepare is setup (not timed), matching CI's iter_batched harness
    let prepared = prepare_private_tx(input_size as usize, &program);

    let start = Instant::now();
    prove_private_tx(&prepared, &());
    let prove_time_ms = start.elapsed().as_millis();

    println!("prove_time_ms: {}", prove_time_ms);

    format!("prove_time_ms={}", prove_time_ms)
}

#[cfg_attr(
    all(feature = "wasm", target_arch = "wasm32"),
    wasm_bindgen(js_name = "moproWasmHelloWorld")
)]
pub fn mopro_wasm_hello_world() -> String {
    "Hello, World!".to_string()
}

#[cfg(test)]
mod uniffi_tests {
    #[test]
    fn test_mopro_hello_world() {
        assert_eq!(super::mopro_hello_world(), "Hello, World!");
    }
}


// CIRCOM_TEMPLATE
circom_stub!();

// HALO2_TEMPLATE
halo2_stub!();

// NOIR_TEMPLATE
noir_stub!();

// GNARK_TEMPLATE
gnark_stub!();
