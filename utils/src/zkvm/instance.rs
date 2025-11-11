use bincode::Options;
use ere_zkvm_interface::{Compiler, ProgramProvingReport, Proof, PublicValues};
use std::path::Path;

/// Holds a compiled program together with its serialized size.
pub struct CompiledProgram<C: Compiler> {
    pub program: C::Program,
    pub byte_size: usize,
}

/// Result of executing `zkVM::prove` for a benchmark.
#[derive(Clone)]
pub struct ProofArtifacts {
    pub public_values: PublicValues,
    pub proof: Proof,
    pub report: ProgramProvingReport,
}

impl ProofArtifacts {
    pub fn new(public_values: PublicValues, proof: Proof, report: ProgramProvingReport) -> Self {
        Self {
            public_values,
            proof,
            report,
        }
    }

    pub fn proof_size(&self) -> usize {
        self.proof.as_bytes().len()
    }
}

/// Compiles a guest program located at `guest_dir` and tracks its serialized size.
pub fn compile_guest_program<C: Compiler>(
    compiler: &C,
    guest_dir: &Path,
) -> Result<CompiledProgram<C>, C::Error> {
    println!("Compiling guest program at {:?}", guest_dir);
    let program = compiler.compile(guest_dir)?;
    let cfg = bincode::options();
    let byte_size = cfg
        .serialize(&program)
        .map(|bytes| bytes.len())
        .unwrap_or_default();
    Ok(CompiledProgram { program, byte_size })
}
