mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

#[derive(Clone, Copy)]
enum LeanvmMobileBench {
    PrivateTx,
    ConstantOverhead,
    MerkleFake,
    HashPoseidon16,
    MerklePoseidon16,
}

/// Runs the LeanVM private-transaction benchmark and returns 10 prove samples in milliseconds.
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_private_tx(input_size: u64) -> String {
    run_leanvm_on_prover_thread(input_size, LeanvmMobileBench::PrivateTx)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_constant_overhead(input_size: u64) -> String {
    run_leanvm_on_prover_thread(input_size, LeanvmMobileBench::ConstantOverhead)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_merkle_fake(input_size: u64) -> String {
    run_leanvm_on_prover_thread(input_size, LeanvmMobileBench::MerkleFake)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_hash_poseidon16(input_size: u64) -> String {
    run_leanvm_on_prover_thread(input_size, LeanvmMobileBench::HashPoseidon16)
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn leanvm_prove_merkle_poseidon16(input_size: u64) -> String {
    run_leanvm_on_prover_thread(input_size, LeanvmMobileBench::MerklePoseidon16)
}

fn run_leanvm_on_prover_thread(input_size: u64, bench: LeanvmMobileBench) -> String {
    // LeanVM's WHIR prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || leanvm_prove_inner(input_size, bench))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn leanvm_prove_inner(input_size: u64, bench: LeanvmMobileBench) -> String {
    use leanvm_bench::{
        compile_constant_overhead, compile_hash_poseidon16, compile_merkle_fake,
        compile_merkle_poseidon16, compile_private_tx, prepare_constant_overhead,
        prepare_hash_poseidon16, prepare_merkle_fake, prepare_merkle_poseidon16,
        prepare_private_tx, prove_lean_bench,
    };
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};

    macro_rules! run_samples {
        ($bench_name:expr, $prepare:expr) => {{
            let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
            for sample in 0..MOBILE_SAMPLE_COUNT {
                let prepared = $prepare;
                let start = Instant::now();
                prove_lean_bench(&prepared, &());
                let prove_time_ms = start.elapsed().as_millis();
                println!(
                    "{}_sample_{}_prove_time_ms: {}",
                    $bench_name,
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
        }};
    }

    match bench {
        LeanvmMobileBench::PrivateTx => {
            // compile_private_tx uses include_str!, so it does not need CARGO_MANIFEST_DIR.
            let bytecode = compile_private_tx();
            run_samples!(
                "private_tx",
                prepare_private_tx(input_size as usize, &bytecode)
            )
        }
        LeanvmMobileBench::ConstantOverhead => {
            let bytecode = compile_constant_overhead();
            run_samples!(
                "constant_overhead",
                prepare_constant_overhead(input_size as usize, &bytecode)
            )
        }
        LeanvmMobileBench::MerkleFake => {
            let bytecodes = compile_merkle_fake();
            run_samples!(
                "merkle_fake",
                prepare_merkle_fake(input_size as usize, &bytecodes)
            )
        }
        LeanvmMobileBench::HashPoseidon16 => {
            let bytecodes = compile_hash_poseidon16();
            run_samples!(
                "hash_poseidon16",
                prepare_hash_poseidon16(input_size as usize, &bytecodes)
            )
        }
        LeanvmMobileBench::MerklePoseidon16 => {
            let bytecodes = compile_merkle_poseidon16();
            run_samples!(
                "merkle_poseidon16",
                prepare_merkle_poseidon16(input_size as usize, &bytecodes)
            )
        }
    }
}
