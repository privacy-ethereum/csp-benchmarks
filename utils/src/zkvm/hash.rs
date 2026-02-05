use crate::zkvm::instance::ProofArtifacts;
use crate::zkvm::traits::PreparedBenchmark;
use ere_zkvm_interface::{Input, Proof, ProofKind, PublicValues, zkVM};

/// Common preparation data for zkVM hash benchmarks.
pub struct PreparedHash<V> {
    vm: V,
    input: Input,
    compiled_size: usize,
    expected_digest: Option<Vec<u8>>,
}

impl<V> PreparedHash<V> {
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

impl<V> PreparedHash<V>
where
    V: zkVM,
{
    pub fn prove(&self) -> Result<ProofArtifacts, anyhow::Error> {
        let (public_values, proof, report) = self.vm.prove(&self.input, ProofKind::default())?;
        Ok(ProofArtifacts::new(public_values, proof, report))
    }

    pub fn verify(&self, proof: &Proof) -> Result<PublicValues, anyhow::Error> {
        self.vm.verify(proof)
    }

    pub fn verify_with_digest(&self, proof: &ProofArtifacts) -> Result<(), anyhow::Error> {
        let public_values = self.vm.verify(&proof.proof)?;

        if public_values != proof.public_values {
            return Err(anyhow::anyhow!("public values mismatch"));
        }

        match &self.expected_digest {
            None => {}
            Some(expected) => {
                if public_values != *expected {
                    return Err(anyhow::anyhow!("digest mismatch"));
                }
            }
        }

        Ok(())
    }

    pub fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        let (_, report) = self.vm.execute(&self.input)?;
        Ok(report.total_num_cycles)
    }
}

impl<V: zkVM> PreparedBenchmark for PreparedHash<V> {
    type VM = V;

    fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        PreparedHash::execution_cycles(self)
    }

    fn prove(&self) -> Result<ProofArtifacts, anyhow::Error> {
        PreparedHash::prove(self)
    }

    fn vm(&self) -> &Self::VM {
        &self.vm
    }

    fn input(&self) -> &Input {
        &self.input
    }
}

/// Builds default zkVM input from raw message bytes.
pub fn build_input(message_bytes: Vec<u8>) -> Input {
    Input::new().with_stdin(message_bytes)
}
