use std::{fs, path::PathBuf};

use bincode::Options;
use clap::Parser;
use ere_openvm::{OPENVM_TARGET, OpenVMProgram};
use openvm::{prepare_sha256, prove_sha256};
use utils::zkvm::{CompiledProgram, SHA256_BENCH};
use zkvm_interface::Compiler;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let compiled_program_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("guest")
        .join(SHA256_BENCH)
        .join("target")
        .join("sha256.bin");
    let program_bin = fs::read(&compiled_program_path).unwrap();
    let program: <OPENVM_TARGET as Compiler>::Program =
        bincode::options().deserialize(&program_bin).unwrap();
    let byte_size = program_bin.len();
    let program = CompiledProgram { program, byte_size };

    let prepared = prepare_sha256(args.input_size, &program);
    prove_sha256(&prepared, &());
}
