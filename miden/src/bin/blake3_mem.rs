use clap::Parser;
use ere_miden::compiler::MidenAsm;
use miden::{blake3_program_name, prepare_blake3_with_program, prove_blake3};
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the BLAKE3 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<MidenAsm>(&blake3_program_name(args.input_size));
    let prepared = prepare_blake3_with_program(args.input_size, &program);
    let _proof = prove_blake3(&prepared, &program);
}
