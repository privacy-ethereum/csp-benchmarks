use clap::Parser;
use zolt_bench::{prepare_sha256, prove_sha256, ZoltSharedState};

#[derive(Parser)]
struct Args {
    #[arg(long = "input-size")]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    let shared = ZoltSharedState::new();
    let prepared = prepare_sha256(args.input_size, shared);
    let _proof = prove_sha256(&prepared, &shared);
}
