use clap::Parser;
use provekit::{prepare_keccak, prove};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let (scheme, toml_path, _pre_size) = prepare_keccak(args.input_size);
    let _proof = prove(&scheme, &toml_path);
}
