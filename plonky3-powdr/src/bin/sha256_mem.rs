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

    match args.input_size {
        2048 => sha256_2048_powdr_mem(),
        _ => panic!("Unsupported input size"),
    }
}

fn sha256_2048_powdr_mem() {
    let mut pipeline = prepare_pipeline();

    prove(&mut pipeline);
}
