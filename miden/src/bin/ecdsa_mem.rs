use clap::Parser;
use ere_miden::compiler::MidenAsm;
use miden::{prepare_ecdsa, prove_ecdsa};
use utils::zkvm::ECDSA_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<MidenAsm>(ECDSA_BENCH);

    let prepared = prepare_ecdsa(args.input_size.unwrap_or(1), &program).expect("prepare_ecdsa");
    let _proof = prove_ecdsa(&prepared, &program);
}
