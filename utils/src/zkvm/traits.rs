use crate::zkvm::instance::ProofArtifacts;
use ere_zkvm_interface::{Compiler, Input, zkVM};

/// Program to be benchmarked.
pub trait Program {
    const NAME: &'static str;
}

/// Common interface for prepared benchmark instances.
pub trait PreparedBenchmark {
    type VM: zkVM;

    /// Get the compiled program size in bytes.
    fn compiled_size(&self) -> usize;

    /// Execute the program and return the total number of cycles.
    fn execution_cycles(&self) -> Result<u64, anyhow::Error>;

    /// Generate a proof for the prepared benchmark.
    fn prove(&self) -> Result<ProofArtifacts, anyhow::Error>;

    /// Get a reference to the underlying zkVM instance.
    fn vm(&self) -> &Self::VM;

    /// Get a reference to the input data.
    fn input(&self) -> &Input;
}

/// Marker trait for benchmark configuration types.
pub trait BenchmarkConfig {}

/// Generates reusable data for benchmarks.
pub trait DataGenerator<C: BenchmarkConfig> {
    type Data;

    fn generate(&self, config: &C) -> (Self::Data, usize);
}

/// Builds a zkVM specific input from generated data.
pub trait InputBuilder<P: Program> {
    type Data;

    fn build_input(data: Self::Data) -> Input;
}

/// Builds a zkVM instance with a compiled program.
pub trait ZkVMBuilder<C: Compiler, V: zkVM> {
    type Error: std::error::Error + Send + Sync + 'static;

    fn build(&self, program: C::Program) -> Result<V, Self::Error>;
}
