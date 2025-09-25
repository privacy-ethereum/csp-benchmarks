//! zkVM instance module.

use crate::traits::ZkVMBuilder;
use criterion::Criterion;
use std::path::PathBuf;
use std::{fmt, marker::PhantomData};
use zkvm_interface::{
    Compiler, Input, ProgramExecutionReport, ProgramProvingReport, PublicValues, zkVM, zkVMError,
};

/// Supported VMs.
pub enum SupportedVms {
    Risc0,
    Sp1,
    Jolt,
    Miden,
}

impl From<&str> for SupportedVms {
    fn from(s: &str) -> Self {
        match s {
            "risc0" => SupportedVms::Risc0,
            "sp1" => SupportedVms::Sp1,
            "jolt" => SupportedVms::Jolt,
            "miden" => SupportedVms::Miden,
            _ => panic!("Unknown VM: {}", s),
        }
    }
}

impl fmt::Display for SupportedVms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SupportedVms::Risc0 => write!(f, "risc0"),
            SupportedVms::Sp1 => write!(f, "sp1"),
            SupportedVms::Jolt => write!(f, "jolt"),
            SupportedVms::Miden => write!(f, "miden"),
        }
    }
}

/// Benchmark instance struct
pub struct ZkVMInstance<C, V>
where
    C: Compiler,
    V: zkVM,
{
    vm: V,
    vm_name: String,
    bench_name: String,
    _phantom: PhantomData<C>,
}

impl<C, V> ZkVMInstance<C, V>
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
        let project_root: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let guest_path = project_root.join("guests").join(vm_name).join(bench_name);

        assert!(
            guest_path.exists(),
            "Guest path does not exist: {}",
            guest_path.display()
        );

        let program = compiler.compile(&guest_path)?;
        let vm = vm_builder.build(program).unwrap();

        Ok(Self {
            vm,
            vm_name: vm_name.to_string(),
            bench_name: bench_name.to_string(),
            _phantom: PhantomData,
        })
    }

    /// Benchmarks the prove and verify methods with Criterion.
    pub fn bench(
        &self,
        input: &Input,
    ) -> Result<(Vec<u8>, PublicValues, ProgramProvingReport), zkVMError> {
        let (_, proof, report) = self.vm.prove(input)?;
        let public_values = self.vm.verify(&proof)?;

        let mut c = Criterion::default();
        let mut group = c.benchmark_group(format!("{}::{}", self.vm_name, self.bench_name));
        group.sample_size(10);

        group.bench_function("prove", |b| {
            b.iter(|| {
                let _ = self
                    .vm
                    .prove(std::hint::black_box(input))
                    .expect("prove failed");
            });
        });

        group.bench_function("verify", |b| {
            b.iter(|| {
                let _ = self
                    .vm
                    .verify(std::hint::black_box(&proof))
                    .expect("verify failed");
            });
        });

        group.finish();
        c.final_summary();

        Ok((proof, public_values, report))
    }

    /// Executes the program with the given input.
    pub fn execute_only(
        &self,
        input: &Input,
    ) -> Result<(PublicValues, ProgramExecutionReport), zkVMError> {
        self.vm.execute(input)
    }

    /// Proves the program with the given input.
    pub fn prove_only(
        &self,
        input: &Input,
    ) -> Result<(Vec<u8>, Vec<u8>, ProgramProvingReport), zkVMError> {
        self.vm.prove(input)
    }
}
