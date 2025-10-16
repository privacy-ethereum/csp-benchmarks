use clap::Parser;
use ere_sp1::RV32_IM_SUCCINCT_ZKVM_ELF;
use sp1::{prepare_sha256, prove_sha256};
use utils::zkvm::SHA256_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the SHA256 benchmark
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled_program::<RV32_IM_SUCCINCT_ZKVM_ELF>(SHA256_BENCH);

    let prepared = prepare_sha256(args.input_size, &program);
    prove_sha256(&prepared, &());
}
