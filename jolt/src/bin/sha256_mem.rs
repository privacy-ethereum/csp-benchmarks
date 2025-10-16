use std::collections::HashMap;

use clap::Parser;
use jolt::{prepare_sha256, prove_sha256};
use utils::zkvm::{SHA256_BENCH, helpers::load_compiled_program};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let mut programs = HashMap::new();
    programs.insert(
        args.input_size,
        load_compiled_program(&format!("{}_{}", SHA256_BENCH, args.input_size)),
    );

    let prepared = prepare_sha256(args.input_size, &programs);
    prove_sha256(&prepared, &());
}
