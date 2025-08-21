use provekit::ProvekitSha256Benchmark;

const INPUT_EXPONENTS: [u32; 1] = [11];

fn main() {
    let bench_harness = ProvekitSha256Benchmark::new(&INPUT_EXPONENTS);

    for &exp in INPUT_EXPONENTS.iter() {
        let _proof = bench_harness.run_prove(exp);
    }
}
