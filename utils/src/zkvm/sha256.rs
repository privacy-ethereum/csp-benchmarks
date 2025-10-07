use zkvm_interface::{Input, ProgramProvingReport, PublicValues, zkVM, zkVMError};

/// Benchmark name for SHA256 programs.
pub const SHA256_BENCH: &str = "sha256";

/// Result of executing `zkVM::prove` for a SHA-256 benchmark.
#[derive(Clone)]
pub struct ProofArtifacts {
    pub public_values: PublicValues,
    pub proof: Vec<u8>,
    pub report: ProgramProvingReport,
}

impl ProofArtifacts {
    pub fn new(public_values: PublicValues, proof: Vec<u8>, report: ProgramProvingReport) -> Self {
        Self {
            public_values,
            proof,
            report,
        }
    }

    pub fn proof_size(&self) -> usize {
        self.proof.len()
    }
}

/// Common preparation data for zkVM SHA-256 benchmarks.
pub struct PreparedSha256<V> {
    vm: V,
    input: Input,
    compiled_size: usize,
    expected_digest: Option<Vec<u8>>,
}

impl<V> PreparedSha256<V> {
    pub fn new(vm: V, input: Input, compiled_size: usize) -> Self {
        Self {
            vm,
            input,
            compiled_size,
            expected_digest: None,
        }
    }

    pub fn with_expected_digest(
        vm: V,
        input: Input,
        compiled_size: usize,
        expected_digest: Vec<u8>,
    ) -> Self {
        Self {
            vm,
            input,
            compiled_size,
            expected_digest: Some(expected_digest),
        }
    }

    pub fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    pub fn expected_digest(&self) -> Option<&[u8]> {
        self.expected_digest.as_deref()
    }

    pub fn vm(&self) -> &V {
        &self.vm
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}

impl<V> PreparedSha256<V>
where
    V: zkVM,
{
    pub fn prove(&self) -> Result<ProofArtifacts, zkVMError> {
        let (public_values, proof, report) = self.vm.prove(&self.input)?;
        Ok(ProofArtifacts::new(public_values, proof, report))
    }

    pub fn verify(&self, proof: &[u8]) -> Result<PublicValues, zkVMError> {
        self.vm.verify(proof)
    }

    pub fn verify_with_digest(&self, proof: &ProofArtifacts) -> Result<(), zkVMError> {
        let public_values = self.vm.verify(&proof.proof)?;

        if public_values != proof.public_values {
            return Err(zkVMError::other("public values mismatch"));
        }

        if let Some(expected) = &self.expected_digest
            && &public_values != expected
        {
            return Err(zkVMError::other("digest mismatch"));
        }

        Ok(())
    }

    pub fn execution_cycles(&self) -> Result<u64, zkVMError> {
        let (_, report) = self.vm.execute(&self.input)?;
        Ok(report.total_num_cycles)
    }
}

/// Builds default zkVM input from raw message bytes.
pub fn build_input(message_bytes: Vec<u8>) -> Input {
    let mut input = Input::new();
    input.write_bytes(message_bytes);
    input
}
