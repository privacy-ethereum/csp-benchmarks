use ere_risc0::{EreRisc0, RV32_IM_RISC0_ZKVM_ELF};
use rand::{RngCore, SeedableRng, rngs::StdRng};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use utils::bench::{Metrics, benchmark};
use utils::metadata::SHA2_INPUTS;
use zkvm_interface::{Compiler, Input, ProverResourceType, zkVM};

const CSV_OUTPUT: &str = "risc0_sha256.csv";

/// Risc0 SHA-256 benchmark.
struct Sha256Benchmark {
    inputs: HashMap<usize, (usize, &'static [u8])>,
}

impl Sha256Benchmark {
    /// Compiles the guest program and generates inputs.
    fn new(sizes: &[usize]) -> Self {
        let inputs = sizes
            .iter()
            .map(|&size| {
                let mut rng = StdRng::seed_from_u64(1337);
                let mut data = vec![0u8; size];
                rng.fill_bytes(&mut data);
                let static_data: &'static [u8] = Box::leak(data.into_boxed_slice());
                (size, (size, static_data))
            })
            .collect();

        Self { inputs }
    }

    /// Runs a single benchmark iteration.
    fn run(&self, input_size: usize) -> Metrics {
        let guest_relative = Path::new("../guests/rust/sha256");
        let program = RV32_IM_RISC0_ZKVM_ELF
            .compile(guest_relative)
            .expect("Failed to compile guest program");

        let &(size, test_data) = self.inputs.get(&input_size).unwrap();

        let zkvm = EreRisc0::new(program.clone(), ProverResourceType::Cpu).unwrap();
        let mut input = Input::new();
        input.write(test_data);

        let execution_report = zkvm.execute(&input).unwrap();

        let prove_start = Instant::now();
        let (proof, _) = zkvm.prove(&input).unwrap();
        let proof_duration = prove_start.elapsed();

        let verify_start = Instant::now();
        zkvm.verify(&proof).unwrap();
        let verify_duration = verify_start.elapsed();

        let mut metrics = Metrics::new(
            "risc0".to_string(),
            "".to_string(),
            true,
            "sha256".to_string(),
            size,
        );
        metrics.proof_duration = proof_duration;
        metrics.verify_duration = verify_duration;
        metrics.cycles = execution_report.total_num_cycles;
        metrics.proof_size = proof.len();

        metrics
    }
}

/// Runs the Risc0 SHA-256 benchmark.
pub fn bench_sha256() {
    let bench_harness = Sha256Benchmark::new(&SHA2_INPUTS);
    benchmark(
        |size| bench_harness.run(size),
        SHA2_INPUTS.as_slice(),
        CSV_OUTPUT,
    );
}
