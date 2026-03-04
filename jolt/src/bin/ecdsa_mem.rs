use clap::Parser;
use jolt::JoltCompiler;
use jolt::{prepare_ecdsa, prove_ecdsa};
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size (unused for ECDSA)
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<JoltCompiler>(ECDSA_BENCH);
    let prepared = prepare_ecdsa(args.input_size, &program);
    prove_ecdsa(&prepared, &());
}
