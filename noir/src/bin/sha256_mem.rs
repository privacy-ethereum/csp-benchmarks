use clap::Parser;
use noir::{prepare_sha256, prove};

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
    let (input_size, toml_path, circuit_path) = prepare_sha256(input_size);
    let _ = prove(input_size, &toml_path, &circuit_path);
}
