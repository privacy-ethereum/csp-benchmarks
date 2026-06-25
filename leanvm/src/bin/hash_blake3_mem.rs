use clap::Parser;
use leanvm::{compile_hash_blake3, prepare_hash_blake3, prove_lean_bench};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "INPUT_SIZE")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let bytecodes = compile_hash_blake3();
    let prepared = prepare_hash_blake3(args.input_size, &bytecodes);
    let _proof = prove_lean_bench(&prepared, &());
}
