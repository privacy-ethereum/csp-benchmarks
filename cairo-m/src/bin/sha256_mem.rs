use cairo_m::{prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_mem(args.input_size);
}

fn sha256_mem(input_size: usize) {
    let runner_output = prepare(input_size);
    let _ = prove(&runner_output);
}
