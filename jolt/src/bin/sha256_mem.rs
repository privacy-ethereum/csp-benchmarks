use clap::Parser;
use ere_jolt::JOLT_TARGET;
use jolt::{prepare_sha256, prove_sha256};
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let bench_name = format!("sha256_{}", args.input_size);
    let program = load_compiled_program::<JOLT_TARGET>(&bench_name);
    let prepared = prepare_sha256(args.input_size, &program);
    prove_sha256(&prepared, &());
}
