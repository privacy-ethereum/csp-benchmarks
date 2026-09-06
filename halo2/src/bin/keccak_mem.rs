use clap::Parser;
use halo2_circuits::bench::{keccak_prepare, prove};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let prepared = keccak_prepare(args.input_size);
    let _proof = prove(&prepared);
}
