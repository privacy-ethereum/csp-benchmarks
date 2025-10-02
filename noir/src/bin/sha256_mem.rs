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

    let (toml_path, circuit_path) = prepare_sha256(args.input_size);
    let _proof = prove(&toml_path, &circuit_path);
}
