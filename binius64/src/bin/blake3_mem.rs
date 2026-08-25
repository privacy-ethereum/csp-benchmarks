use anyhow::Result;
use binius64::circuits::blake3::Blake3Params;
use binius64::{circuits::Blake3Circuit, prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size in bytes.
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();
    blake3_mem(args.input_size).expect("Failed to run BLAKE3 prove process");
}

fn blake3_mem(input_size: usize) -> Result<()> {
    let (_verifier, prover, _cs, circuit, compiled_circuit, input_size) = prepare::<Blake3Circuit>(
        input_size,
        Blake3Params {
            max_len_bytes: Some(input_size),
        },
    )?;
    let _ = prove::<Blake3Circuit>(&prover, &compiled_circuit, &circuit, input_size)?;
    Ok(())
}
