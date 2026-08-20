use clap::Parser;
use stark_v::{load_compiled, prepare_blake3, prove_bench};

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes for the BLAKE3 benchmark.
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    let program = load_compiled("blake3");
    let prepared = prepare_blake3(args.input_size, &program);
    prove_bench(&prepared, &program);
}
