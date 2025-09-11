//! Benchmark module.

use crate::traits::{BenchmarkConfig, DataGenerator, InputBuilder, Program, ZkVMBuilder};
use std::{fs::canonicalize, marker::PhantomData, time::Instant};
use utils::bench::Metrics;
use zkvm_interface::{Compiler, PublicValues, zkVM, zkVMError};

/// Benchmark instance struct
pub struct Benchmark<C, V>
where
    C: Compiler,
    V: zkVM,
{
    vm: V,
    vm_name: String,
    bench_name: String,
    _phantom: PhantomData<C>,
}

impl<C, V> Benchmark<C, V>
where
    C: Compiler,
    V: zkVM,
{
    /// Create a new benchmark instance.
    pub fn new(
        compiler: &C,
        vm_name: &str,
        bench_name: &str,
        vm_builder: &impl ZkVMBuilder<C, V>,
    ) -> Result<Self, C::Error> {
        let guest_path = canonicalize("guests")
            .unwrap()
            .join(vm_name)
            .join(bench_name);
        let program = compiler.compile(&guest_path)?;
        let vm = vm_builder.build(program).unwrap();

        Ok(Self {
            vm,
            vm_name: vm_name.to_string(),
            bench_name: bench_name.to_string(),
            _phantom: PhantomData,
        })
    }

    /// Benchmark runner.
    pub fn run<P, B, G>(
        &self,
        generator: &G,
        config: &B,
    ) -> Result<(PublicValues, Metrics), zkVMError>
    where
        P: Program,
        B: BenchmarkConfig,
        G: DataGenerator<B, Data = <V as InputBuilder<P>>::Data>,
        V: InputBuilder<P>,
    {
        let (data, size) = generator.generate(config);
        let input = V::build_input(data);

        // Execute the program
        let (_, exec_report) = self.vm.execute(&input)?;

        // Prove
        let prove_start = Instant::now();
        let (_, proof, _) = self.vm.prove(&input)?;
        let proof_duration = prove_start.elapsed();

        // Verify
        let verify_start = Instant::now();
        let public_values = self.vm.verify(&proof)?;
        let verify_duration = verify_start.elapsed();

        let mut metrics = Metrics::new(
            self.vm_name.clone(),
            "".to_string(),
            true,
            self.bench_name.clone(),
            size,
        );

        metrics.proof_duration = proof_duration;
        metrics.verify_duration = verify_duration;
        metrics.cycles = exec_report.total_num_cycles;
        metrics.proof_size = proof.len();

        Ok((public_values, metrics))
    }
}
