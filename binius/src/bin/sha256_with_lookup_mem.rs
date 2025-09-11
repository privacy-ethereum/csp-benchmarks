use binius::bench::{prove, sha256_with_lookup_prepare};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_with_lookup_mem(args.input_size);
}

// TODO: variable input size
fn sha256_with_lookup_mem(_input_size: usize) {
    let allocator = bumpalo::Bump::new();

    let (constraint_system, args, witness, backend) = sha256_with_lookup_prepare(&allocator);

    let (_, _, _proof) = prove(constraint_system, args, witness, backend);
}
