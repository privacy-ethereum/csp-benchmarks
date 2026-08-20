use anyhow::Result;
use clap::Parser;
use flock::{prepare_blake3, prove_blake3};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prepared = prepare_blake3(args.input_size);
    let _ = prove_blake3(&prepared);
    Ok(())
}
