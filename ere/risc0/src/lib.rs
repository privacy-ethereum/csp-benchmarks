use ere_risc0::{EreRisc0, RV32_IM_RISC0_ZKVM_ELF, Risc0Program};
use rand::RngCore;
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;
use utils::bench::{Metrics, benchmark};
use zkvm_interface::{Compiler, Input, ProverResourceType, zkVM};

const INPUT_EXP: [u8; 5] = [8, 9, 10, 11, 12];
const CSV_OUTPUT: &str = "risc0_sha256.csv";

/// Risc0 SHA-256 benchmark.
struct Sha256Benchmark {
    program: Risc0Program,
    inputs: HashMap<u8, (usize, &'static [u8])>,
}

impl Sha256Benchmark {
    /// Compiles the guest program and generates all necessary inputs.
    fn new(exponents: &[u8]) -> Self {
        let guest_relative = Path::new("../guests/sha256");
        let program = RV32_IM_RISC0_ZKVM_ELF
            .compile(guest_relative)
            .expect("Failed to compile guest program");

        let inputs = exponents
            .iter()
            .map(|&exp| {
                let size = 1usize << exp;
                let mut data = vec![0u8; size];
                rand::thread_rng().fill_bytes(&mut data);
                let static_data: &'static [u8] = Box::leak(data.into_boxed_slice());
                (exp, (size, static_data))
            })
            .collect();

        Self { program, inputs }
    }

    /// Runs a single benchmark iteration.
    fn run(&self, input_exp: u8) -> Metrics {
        let &(size, test_data) = self
            .inputs
            .get(&input_exp)
            .expect("Input not found for exponent");

        let zkvm = EreRisc0::new(self.program.clone(), ProverResourceType::Cpu);
        let mut input = Input::new();
        input.write(test_data);

        let execution_report = zkvm.execute(&input).unwrap();

        let prove_start = Instant::now();
        let (proof, _) = zkvm.prove(&input).unwrap();
        let proof_duration = prove_start.elapsed();

        let verify_start = Instant::now();
        zkvm.verify(&proof).unwrap();
        let verify_duration = verify_start.elapsed();

        let mut metrics = Metrics::new(size);
        metrics.proof_duration = proof_duration;
        metrics.verify_duration = verify_duration;
        metrics.cycles = execution_report.total_num_cycles;
        metrics.proof_size = proof.len();

        metrics
    }
}

/// Runs the Risc0 SHA-256 benchmark.
pub fn bench_sha256() {
    let bench_harness = Sha256Benchmark::new(&INPUT_EXP);
    benchmark(
        |exp| bench_harness.run(exp),
        INPUT_EXP.as_slice(),
        CSV_OUTPUT,
    );
}
