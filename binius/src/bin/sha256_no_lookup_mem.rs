use binius::bench::{prove, sha256_no_lookup_prepare};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    match args.input_size {
        2048 => sha256_2048_binius_no_lookup_mem(),
        _ => panic!("Unsupported input size"),
    }
}

fn sha256_2048_binius_no_lookup_mem() {
    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) = sha256_no_lookup_prepare(&allocator);

    let (_, _, _proof) = prove(constraint_system, args, witness, backend);
}
