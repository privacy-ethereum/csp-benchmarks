use clap::Parser;
use halo2_circuits::bench::{prove, sha256_prepare};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = sha256_prepare(args.input_size);
    let _proof = prove(&prepared);
}
