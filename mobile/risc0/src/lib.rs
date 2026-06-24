mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

/// Runs the RISC0 private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
/// Matches CI: prepare is setup (not timed), only prove is measured.
/// Returns comma-separated summary fields, including all raw samples.
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
    use ere_risc0::{compiler::RustRv32imaCustomized, EreRisc0};
    use ere_zkvm_interface::zkvm::ProverResource;
    use risc0_bench::prove_private_tx;
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};
    use utils::zkvm::{load_compiled_program_from_path, PreparedPrivateTx};

    let program = load_compiled_program_from_path::<RustRv32imaCustomized>(std::path::Path::new(
        &compiled_program_path,
    ));

    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        // ProverResource::Gpu uses the Metal in-process prover (no external r0vm process).
        // CPU mode spawns r0vm as a subprocess, which is unavailable on iOS.
        let vm = EreRisc0::new(program.program.clone(), ProverResource::Gpu)
            .expect("failed to build risc0 Metal prover");

        let (input_bytes, expected_public_values) =
            utils::generate_private_tx_input(input_size as usize);
        let len = input_bytes.len() as u32;
        let mut framed = Vec::with_capacity(4 + input_bytes.len());
        framed.extend_from_slice(&len.to_le_bytes());
        framed.extend(input_bytes);
        let input = ere_zkvm_interface::Input::new().with_stdin(framed);

        let prepared = PreparedPrivateTx::with_expected_public_values(
            vm,
            input,
            program.byte_size,
            expected_public_values,
        );

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
