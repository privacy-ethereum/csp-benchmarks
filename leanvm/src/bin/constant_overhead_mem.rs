use clap::Parser;
use leanvm::{compile_constant_overhead, prepare_constant_overhead, prove_lean_bench};

#[derive(Parser)]
struct Args {
    #[arg(long, env = "INPUT_SIZE")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let bytecode = compile_constant_overhead();
    let prepared = prepare_constant_overhead(args.input_size, &bytecode);
    let _proof = prove_lean_bench(&prepared, &());
}
