use cairo_m_common::{InputValue, Program};
use cairo_m_compiler::{CompilerOptions, compile_cairo};
use cairo_m_prover::{
    Proof, adapter::import_from_runner_output, prover::prove_cairo_m,
    prover_config::REGULAR_96_BITS, verifier::verify_cairo_m,
};
use cairo_m_runner::run_cairo_program;
use std::fs;
use stwo_prover::core::vcs::blake2_merkle::{Blake2sMerkleChannel, Blake2sMerkleHasher};
use utils::generate_sha256_input;

/// Compile the Cairo-M SHA256 program from source.
pub fn compile_program() -> Program {
    let source_path = "programs/sha256.cm".to_string();
    let source_text = fs::read_to_string(&source_path).expect("Failed to read sha256.cm");
    let options = CompilerOptions {
        verbose: false,
        optimization_level: Default::default(),
    };
    let output =
        compile_cairo(source_text, source_path, options).expect("Failed to compile sha256.cm");
    (*output.program).clone()
}

/// Prepares a message for the Cairo-M SHA256 function by padding it and
/// converting it to u32 words.
fn prepare_sha256_input(msg: &[u8]) -> Vec<u32> {
    // Perform standard SHA-256 padding
    let mut padded_bytes = msg.to_vec();
    padded_bytes.push(0x80);

    // Pad to 56 bytes (448 bits) within the last chunk
    while padded_bytes.len() % 64 != 56 {
        padded_bytes.push(0x00);
    }

    // Append message length as 64-bit big-endian
    let bit_len = (msg.len() as u64) * 8;
    padded_bytes.extend_from_slice(&bit_len.to_be_bytes());

    // Convert bytes to u32 words (big-endian)
    padded_bytes
        .chunks_exact(4)
        .map(|chunk| u32::from_be_bytes(chunk.try_into().expect("Chunk size mismatch")))
        .collect()
}

/// Prepare the input for the Cairo-M SHA256 program.
/// Takes a pre-compiled program and input size, returns the program with its inputs.
pub fn prepare(
    input_size: usize,
    compiled_program: &Program,
) -> (Program, (String, Vec<InputValue>)) {
    // Generate input using sha2_input
    let (input_bytes, _digest) = generate_sha256_input(2048);

    // Prepare the input with proper SHA-256 padding
    let padded_words = prepare_sha256_input(&input_bytes);

    // Prepare the entry point and input params based on input_size
    let entrypoint_name = "sha256_hash".to_string();
    let input_values: Vec<InputValue> = padded_words
        .iter()
        .map(|&word| InputValue::Number(word as i64))
        .collect();
    // padding adds extra 64 bytes to the input message bytes
    let num_chunks = input_size / 64_usize + 1;
    let runner_inputs = vec![
        InputValue::List(input_values),
        InputValue::Number(num_chunks as i64),
    ];

    (compiled_program.clone(), (entrypoint_name, runner_inputs))
}

pub fn prove(
    compiled_program: &Program,
    inputs: (&str, &[InputValue]),
) -> Proof<Blake2sMerkleHasher> {
    let (entrypoint_name, runner_inputs) = inputs;

    // Run/Execute the program
    let runner_output = run_cairo_program(
        compiled_program,
        entrypoint_name,
        runner_inputs,
        Default::default(),
    )
    .expect("failed to run cairo program");

    // Proof Generation
    let segment = runner_output
        .vm
        .segments
        .clone()
        .into_iter()
        .next()
        .unwrap();

    let mut prover_input =
        import_from_runner_output(segment, runner_output.public_address_ranges.clone())
            .expect("Failed to import runner output for proof generation");

    let pcs_config = REGULAR_96_BITS;

    prove_cairo_m::<Blake2sMerkleChannel>(&mut prover_input, Some(pcs_config))
        .expect("failed to generate proof")
}

pub fn verify(proof: &Proof<Blake2sMerkleHasher>) {
    let pcs_config = REGULAR_96_BITS;

    verify_cairo_m::<Blake2sMerkleChannel>(proof.clone(), Some(pcs_config))
        .expect("failed to verify proof");
}
