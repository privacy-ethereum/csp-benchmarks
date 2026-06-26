use anyhow::Result;
use clap::Parser;
use flock::{prepare_keccak, prove_keccak};

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input_size: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let prepared = prepare_keccak(args.input_size);
    let _ = prove_keccak(&prepared);
    Ok(())
}
