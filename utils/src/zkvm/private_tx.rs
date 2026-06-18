use crate::zkvm::instance::ProofArtifacts;
use crate::zkvm::traits::PreparedBenchmark;
use ere_zkvm_interface::{Input, Proof, ProofKind, PublicValues, zkVM};

pub const PRIVATE_TX_BENCH: &str = "private_tx";

pub struct PreparedPrivateTx<V> {
    vm: V,
    input: Input,
    compiled_size: usize,
    expected_public_values: Vec<u8>,
}

impl<V> PreparedPrivateTx<V> {
    pub fn with_expected_public_values(
        vm: V,
        input: Input,
        compiled_size: usize,
        expected_public_values: Vec<u8>,
    ) -> Self {
        Self {
            vm,
            input,
            compiled_size,
            expected_public_values,
        }
    }

    pub fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    pub fn vm(&self) -> &V {
        &self.vm
    }

    pub fn input(&self) -> &Input {
        &self.input
    }
}

impl<V> PreparedPrivateTx<V>
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

    pub fn verify_with_expected(&self, proof: &ProofArtifacts) -> Result<(), anyhow::Error> {
        let public_values = self.vm.verify(&proof.proof)?;

        if public_values != proof.public_values {
            return Err(anyhow::anyhow!("public values mismatch"));
        }

        if public_values != self.expected_public_values {
            return Err(anyhow::anyhow!("private_tx public output mismatch"));
        }

        Ok(())
    }

    pub fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        let (_, report) = self.vm.execute(&self.input)?;
        Ok(report.total_num_cycles)
    }
}

impl<V: zkVM> PreparedBenchmark for PreparedPrivateTx<V> {
    type VM = V;

    fn compiled_size(&self) -> usize {
        self.compiled_size
    }

    fn execution_cycles(&self) -> Result<u64, anyhow::Error> {
        PreparedPrivateTx::execution_cycles(self)
    }

    fn prove(&self) -> Result<ProofArtifacts, anyhow::Error> {
        PreparedPrivateTx::prove(self)
    }

    fn vm(&self) -> &Self::VM {
        &self.vm
    }

    fn input(&self) -> &Input {
        &self.input
    }
}

pub fn build_private_tx_input(input_bytes: Vec<u8>) -> Input {
    Input::new().with_stdin(input_bytes)
}
