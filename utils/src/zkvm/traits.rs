use ere_zkvm_interface::{Compiler, Input, zkVM};

/// Program to be benchmarked.
pub trait Program {
    const NAME: &'static str;
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
