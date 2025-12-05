use anyhow::Result;
use binius_prover::hash::parallel_compression::ParallelCompressionAdaptor;
use binius_verifier::hash::{StdCompression, StdDigest};
use binius64::{
    circuits::{
        KeccakCircuit,
        keccak::KeccakParams,
    },
    prepare, prove,
};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    keccak_mem(args.input_size).expect("Failed to run prove process");
}

fn keccak_mem(input_size: usize) -> Result<()> {
    let (_verifier, prover, _cs, keccak_circuit, compiled_circuit, input_size) =
        prepare::<KeccakCircuit>(
            input_size,
            KeccakParams {
                max_len_bytes: Some(input_size),
            },
        )?;
    let _ = prove::<
        StdDigest,
        StdCompression,
        ParallelCompressionAdaptor<StdCompression>,
        KeccakCircuit,
    >(
        &prover,
        &compiled_circuit,
        &keccak_circuit,
        input_size,
    )?;
    Ok(())
}
