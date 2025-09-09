use ere_miden::{EreMiden, MidenTarget};
use rand::RngCore;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use utils::bench::{Metrics, benchmark};
use utils::metadata::SHA2_INPUTS;
use zkvm_interface::{Compiler, InputItem, ProverResourceType, zkVM};
const CSV_OUTPUT: &str = "miden_sha256.csv";

/// Miden SHA-256 benchmark harness.
struct Sha256Benchmark {
    program: <MidenTarget as Compiler>::Program,
    inputs: HashMap<usize, (usize, Vec<u8>)>,
}

impl Sha256Benchmark {
    /// Compiles the guest program and generates inputs.
    fn new(sizes: &[usize]) -> Self {
        let guest_path = Path::new("../guests/masm/sha256");
        let program = MidenTarget
            .compile(guest_path)
            .expect("Failed to compile guest program");

        let inputs = sizes
            .iter()
            .map(|&size| {
                let mut data = vec![0u8; size];
                rand::thread_rng().fill_bytes(&mut data);
                (size, (size, data))
            })
            .collect();

        Self { program, inputs }
    }

    /// Runs a single benchmark iteration.
    fn run(&self, input_size: usize) -> Metrics {
        let (size, test_data) = self.inputs.get(&input_size).unwrap();
        let zkvm = EreMiden::new(self.program.clone(), ProverResourceType::Cpu);

        // Input size is placed in the stack on the ere side
        let input = vec![InputItem::Bytes(test_data.clone())].into();

        // Execute, prove, and verify, measuring performance at each step.
        let execution_report = zkvm.execute(&input).unwrap();

        let prove_start = Instant::now();
        let (proof, _) = zkvm.prove(&input).unwrap();
        let proof_duration = prove_start.elapsed();

        let verify_start = Instant::now();
        zkvm.verify(&proof).unwrap();
        let verify_duration = verify_start.elapsed();

        let mut metrics = Metrics::new(
            "miden".to_string(),
            "".to_string(),
            true,
            "sha256".to_string(),
            *size,
        );
        metrics.proof_duration = proof_duration;
        metrics.verify_duration = verify_duration;
        metrics.cycles = execution_report.total_num_cycles;
        metrics.proof_size = proof.len();
        metrics
    }
}

/// Runs the Miden SHA-256 benchmark.
pub fn bench_sha256() {
    let bench_harness = Sha256Benchmark::new(&SHA2_INPUTS);
    benchmark(
        |size| bench_harness.run(size),
        SHA2_INPUTS.as_slice(),
        CSV_OUTPUT,
    );
}
