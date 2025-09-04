use provekit::ProvekitSha256Benchmark;

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(2048);
    let _proof = bench_harness.run_prove();
}
