use clap::Parser;
use plonky2_sha256::bench::{prove, sha256_no_lookup_prepare};

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    match args.input_size {
        2048 => sha256_2048_plonky2_no_lookup_mem(),
        _ => panic!("Unsupported input size"),
    }
}

fn sha256_2048_plonky2_no_lookup_mem() {
    let (data, pw) = sha256_no_lookup_prepare();
    let _proof = prove(&data.prover_data(), pw);
}
