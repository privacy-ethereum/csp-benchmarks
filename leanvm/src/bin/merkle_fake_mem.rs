use clap::Parser;
use leanvm::{compile_merkle_fake, prepare_merkle_fake, prove_lean_bench};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "INPUT_SIZE")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let bytecodes = compile_merkle_fake();
    let prepared = prepare_merkle_fake(args.input_size, &bytecodes);
    let _proof = prove_lean_bench(&prepared, &());
}
