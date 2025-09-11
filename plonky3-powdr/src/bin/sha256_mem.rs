use clap::Parser;
use sha::bench::{prepare_pipeline, prove};

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

// TODO: variable input size
fn sha256_mem(_input_size: usize) {
    let mut pipeline = prepare_pipeline();

    prove(&mut pipeline);
}
