use cairo_m::{prepare, prove};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    sha256_mem(args.input_size);
}

fn sha256_mem(input_size: usize) {
    let (program, (entrypoint_name, runner_inputs)) = prepare(input_size);
    let _ = prove(&program, (&entrypoint_name, &runner_inputs));
}
