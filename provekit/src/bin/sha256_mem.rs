use clap::Parser;
use provekit::ProvekitSha256Benchmark;

#[derive(Parser, Debug)]
struct Args {
    /// Input size parameter
    #[arg(long)]
    input_size: usize,
}

fn main() {
    let args = Args::parse();

    match args.input_size {
        2048 => sha256_2048_provekit_mem(),
        _ => panic!("Unsupported input size"),
    }
}

fn sha256_2048_provekit_mem() {
    let bench_harness = ProvekitSha256Benchmark::new(2048);
    let _proof = bench_harness.run_prove();
}
