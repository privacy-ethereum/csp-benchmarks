mod error;
pub use error::MoproError;

// Initializes the shared UniFFI scaffolding and defines the `MoproError` enum.
#[cfg(not(target_arch = "wasm32"))]
mopro_ffi::app!();

#[derive(Clone, Copy)]
enum Risc0MobileBench {
    PrivateTx,
    ConstantOverhead,
    MerkleFake,
    HashSha256,
    MerkleSha256,
    HashKeccak,
    MerkleKeccak,
}

impl Risc0MobileBench {
    fn name(self) -> &'static str {
        match self {
            Self::PrivateTx => "private_tx",
            Self::ConstantOverhead => "constant_overhead",
            Self::MerkleFake => "merkle_fake",
            Self::HashSha256 => "hash_sha256",
            Self::MerkleSha256 => "merkle_sha256",
            Self::HashKeccak => "hash_keccak",
            Self::MerkleKeccak => "merkle_keccak",
        }
    }
}

/// Runs the RISC0 private-transaction benchmark and returns 10 prove samples in milliseconds.
/// `compiled_program_path` must point to the pre-compiled guest binary (private_tx.bin).
#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_private_tx(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::PrivateTx,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_constant_overhead(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::ConstantOverhead,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_merkle_fake(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::MerkleFake,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_hash_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::HashSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_merkle_sha256(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::MerkleSha256,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_hash_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::HashKeccak,
    )
}

#[cfg_attr(feature = "uniffi", uniffi::export)]
pub fn risc0_prove_merkle_keccak(input_size: u64, compiled_program_path: String) -> String {
    run_risc0_on_prover_thread(
        input_size,
        compiled_program_path,
        Risc0MobileBench::MerkleKeccak,
    )
}

fn run_risc0_on_prover_thread(
    input_size: u64,
    compiled_program_path: String,
    bench: Risc0MobileBench,
) -> String {
    // RISC0 prover uses deep recursion; 64 MB stack prevents overflow on iOS.
    std::thread::Builder::new()
        .stack_size(64 * 1024 * 1024)
        .spawn(move || risc0_prove_inner(input_size, compiled_program_path, bench))
        .expect("failed to spawn prover thread")
        .join()
        .expect("prover thread panicked")
}

fn risc0_prove_inner(
    input_size: u64,
    compiled_program_path: String,
    bench: Risc0MobileBench,
) -> String {
    use ere_risc0::compiler::RustRv32imaCustomized;
    use risc0_bench::prove_private_tx;
    use std::time::{Duration, Instant};
    use utils::mobile_stats::{format_prove_ms_summary, MOBILE_BREAK_SECS, MOBILE_SAMPLE_COUNT};
    use utils::targeted::ByteHashKind;
    use utils::zkvm::{load_compiled_program_from_path, PreparedPrivateTx};

    let program = load_compiled_program_from_path::<RustRv32imaCustomized>(std::path::Path::new(
        &compiled_program_path,
    ));

    let mut samples = Vec::with_capacity(MOBILE_SAMPLE_COUNT);
    for sample in 0..MOBILE_SAMPLE_COUNT {
        let prove_time_ms = match bench {
            Risc0MobileBench::PrivateTx => {
                let vm = new_risc0_mobile_vm(&program);
                let (input_bytes, expected_public_values) =
                    utils::generate_private_tx_input(input_size as usize);
                let input = build_risc0_framed_input(input_bytes);
                let prepared = PreparedPrivateTx::with_expected_public_values(
                    vm,
                    input,
                    program.byte_size,
                    expected_public_values,
                );
                let start = Instant::now();
                prove_private_tx(&prepared, &());
                start.elapsed().as_millis()
            }
            Risc0MobileBench::ConstantOverhead => prove_targeted_sample(
                &program,
                utils::targeted::generate_constant_overhead_input(),
            ),
            Risc0MobileBench::MerkleFake => prove_targeted_sample(
                &program,
                utils::targeted::generate_fake_merkle_input(input_size as usize),
            ),
            Risc0MobileBench::HashSha256 => prove_targeted_sample(
                &program,
                utils::targeted::generate_hash_count_input(
                    ByteHashKind::Sha256,
                    input_size as usize,
                ),
            ),
            Risc0MobileBench::MerkleSha256 => prove_targeted_sample(
                &program,
                utils::targeted::generate_real_merkle_input(
                    ByteHashKind::Sha256,
                    input_size as usize,
                ),
            ),
            Risc0MobileBench::HashKeccak => prove_targeted_sample(
                &program,
                utils::targeted::generate_hash_count_input(
                    ByteHashKind::Keccak,
                    input_size as usize,
                ),
            ),
            Risc0MobileBench::MerkleKeccak => prove_targeted_sample(
                &program,
                utils::targeted::generate_real_merkle_input(
                    ByteHashKind::Keccak,
                    input_size as usize,
                ),
            ),
        };

        println!(
            "{}_sample_{}_prove_time_ms: {}",
            bench.name(),
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
}

fn new_risc0_mobile_vm(
    program: &utils::zkvm::CompiledProgram<ere_risc0::compiler::RustRv32imaCustomized>,
) -> ere_risc0::EreRisc0 {
    // ProverResource::Gpu uses the Metal in-process prover (no external r0vm process).
    // CPU mode spawns r0vm as a subprocess, which is unavailable on iOS.
    ere_risc0::EreRisc0::new(
        program.program.clone(),
        ere_zkvm_interface::zkvm::ProverResource::Gpu,
    )
    .expect("failed to build risc0 Metal prover")
}

fn prove_targeted_sample(
    program: &utils::zkvm::CompiledProgram<ere_risc0::compiler::RustRv32imaCustomized>,
    generated: (Vec<u8>, Vec<u8>),
) -> u128 {
    let vm = new_risc0_mobile_vm(program);
    let (input_bytes, expected_public_values) = generated;
    let input = build_risc0_framed_input(input_bytes);
    let prepared = utils::zkvm::PreparedTargeted::with_expected_digest(
        vm,
        input,
        program.byte_size,
        expected_public_values,
    );
    let start = std::time::Instant::now();
    risc0_bench::prove_targeted(&prepared, &());
    start.elapsed().as_millis()
}

fn build_risc0_framed_input(data: Vec<u8>) -> ere_zkvm_interface::Input {
    let len = data.len() as u32;
    let mut framed = Vec::with_capacity(4 + data.len());
    framed.extend_from_slice(&len.to_le_bytes());
    framed.extend(data);
    ere_zkvm_interface::Input::new().with_stdin(framed)
}
