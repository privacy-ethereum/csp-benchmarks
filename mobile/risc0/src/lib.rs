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

/// Runs the RISC0 private-transaction benchmark and returns prove time in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Matches CI: prepare is setup (not timed), only prove is measured.
/// Returns "prove_time_ms=<N>"
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    // RISC0 prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || risc0_prove_private_tx_inner(input_size, compiled_program_path))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn risc0_prove_private_tx_inner(input_size: u64, compiled_program_path: String) -> String {
    use std::time::Instant;
    use ere_risc0::{EreRisc0, compiler::RustRv32imaCustomized};
    use ere_zkvm_interface::zkvm::ProverResource;
    use risc0_bench::prove_private_tx;
    use utils::zkvm::{PreparedPrivateTx, load_compiled_program_from_path};

    let program = load_compiled_program_from_path::<RustRv32imaCustomized>(
        std::path::Path::new(&compiled_program_path),
    );

    // ProverResource::Gpu uses the Metal in-process prover (no external r0vm process).
    // CPU mode spawns r0vm as a subprocess, which is unavailable on iOS.
    let vm = EreRisc0::new(program.program.clone(), ProverResource::Gpu)
        .expect("failed to build risc0 Metal prover");

    let (input_bytes, expected_public_values) = utils::generate_private_tx_input(input_size as usize);
    // RISC0 expects a 4-byte little-endian length prefix before the payload.
    let len = input_bytes.len() as u32;
    let mut framed = Vec::with_capacity(4 + input_bytes.len());
    framed.extend_from_slice(&len.to_le_bytes());
    framed.extend(input_bytes);
    let input = ere_zkvm_interface::Input::new().with_stdin(framed);

    // prepare is setup (not timed), matching CI's iter_batched harness
    let prepared = PreparedPrivateTx::with_expected_public_values(
        vm,
        input,
        program.byte_size,
        expected_public_values,
    );

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
