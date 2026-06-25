use clap::Parser;
use leanvm::{compile_merkle_blake3, prepare_merkle_blake3, prove_lean_bench};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "INPUT_SIZE")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let bytecodes = compile_merkle_blake3();
    let prepared = prepare_merkle_blake3(args.input_size, &bytecodes);
    let _proof = prove_lean_bench(&prepared, &());
}
