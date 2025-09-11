//! Traits module.

use zkvm_interface::{Compiler, Input, zkVM, zkVMError};

/// Program to be benchmarked
pub trait Program {
    const NAME: &'static str;
}

/// Benchmark configuration marker trait
pub trait BenchmarkConfig {}

/// Generate test data for a program with the given configuration
/// The data can be used across different VMs
pub trait DataGenerator<C: BenchmarkConfig> {
    type Data;

    fn generate(&self, config: &C) -> (Self::Data, usize);
}

/// Build zkVM specific input from generated data
pub trait InputBuilder<P: Program> {
    type Data;
    fn build_input(data: Self::Data) -> Input;
}

/// Build a zkVM instance with a loaded program
pub trait ZkVMBuilder<C: Compiler, V: zkVM> {
    fn build(&self, program: C::Program) -> Result<V, zkVMError>;
}
