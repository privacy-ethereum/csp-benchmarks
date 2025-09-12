use clap::Parser;
use plonky2_sha256::bench::{prove, sha256_prepare};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_no_lookup_mem(args.input_size);
}

fn sha256_no_lookup_mem(input_size: usize) {
    let (data, pw) = sha256_prepare(input_size);
    let _proof = prove(&data.prover_data(), pw);
}
