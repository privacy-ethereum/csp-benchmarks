use clap::Parser;
use ere_jolt::compiler::RustRv64imacCustomized;
use jolt::{prepare_constant_overhead, prove_targeted};
use utils::zkvm::CONSTANT_OVERHEAD_BENCH;
use utils::zkvm::helpers::load_compiled_program;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let program = load_compiled_program::<RustRv64imacCustomized>(CONSTANT_OVERHEAD_BENCH);
    let prepared = prepare_constant_overhead(args.input_size, &program);
    prove_targeted(&prepared, &());
}
