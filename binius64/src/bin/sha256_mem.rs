use anyhow::Result;
use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::{prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_mem(args.input_size).expect("Failed to run prove process");
}

fn sha256_mem(input_size: usize) -> Result<()> {
    let (_verifier, prover, _cs, sha256_circuit, compiled_circuit, input_size) =
        prepare(input_size)?;
    let _ = prove::<StdDigest, StdCompression, ParallelCompressionAdaptor<StdCompression>>(
        &prover,
        &compiled_circuit,
        &sha256_circuit,
        input_size,
    )?;
    Ok(())
}
