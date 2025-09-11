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

    let bench_harness = ProvekitSha256Benchmark::new(args.input_size);
    let _proof = bench_harness.run_prove();
}
