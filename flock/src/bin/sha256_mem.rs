use anyhow::Result;
use clap::Parser;
use flock::{prepare_sha256, prove_sha256};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prepared = prepare_sha256(args.input_size);
    let _ = prove_sha256(&prepared);
    Ok(())
}
