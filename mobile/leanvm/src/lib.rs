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
    use std::time::Instant;
    use leanvm_bench::{compile_private_tx, prepare_private_tx, prove_private_tx};

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
