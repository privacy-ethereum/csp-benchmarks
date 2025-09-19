//! Guest programs module.

use crate::traits::ZkVMBuilder;
use ere_jolt::{EreJolt, JOLT_TARGET};
use ere_miden::{EreMiden, MIDEN_TARGET, error::MidenError};
use ere_risc0::{EreRisc0, RV32_IM_RISC0_ZKVM_ELF};
use ere_sp1::{EreSP1, RV32_IM_SUCCINCT_ZKVM_ELF};
use zkvm_interface::{Compiler, ProverResourceType, zkVMError};

pub mod sha256;

/// Loads a program into an EreRisc0 zkVM instance
pub struct Risc0Builder;
impl ZkVMBuilder<RV32_IM_RISC0_ZKVM_ELF, EreRisc0> for Risc0Builder {
    type Error = zkVMError;

    fn build(
        &self,
        program: <RV32_IM_RISC0_ZKVM_ELF as Compiler>::Program,
    ) -> Result<EreRisc0, zkVMError> {
        EreRisc0::new(program, ProverResourceType::Cpu)
    }
}

/// Loads a program into an EreSP1 zkVM instance
pub struct Sp1Builder;
impl ZkVMBuilder<RV32_IM_SUCCINCT_ZKVM_ELF, EreSP1> for Sp1Builder {
    type Error = zkVMError;

    fn build(
        &self,
        program: <RV32_IM_SUCCINCT_ZKVM_ELF as Compiler>::Program,
    ) -> Result<EreSP1, zkVMError> {
        Ok(EreSP1::new(program, ProverResourceType::Cpu))
    }
}

/// Loads a program into an EreJolt zkVM instance
pub struct JoltBuilder;
impl ZkVMBuilder<JOLT_TARGET, EreJolt> for JoltBuilder {
    type Error = zkVMError;

    fn build(&self, program: <JOLT_TARGET as Compiler>::Program) -> Result<EreJolt, zkVMError> {
        EreJolt::new(program, ProverResourceType::Cpu)
    }
}

/// Loads a program into an EreMiden zkVM instance
pub struct MidenBuilder;
impl ZkVMBuilder<MIDEN_TARGET, EreMiden> for MidenBuilder {
    type Error = MidenError;

    fn build(&self, program: <MIDEN_TARGET as Compiler>::Program) -> Result<EreMiden, MidenError> {
        EreMiden::new(program, ProverResourceType::Cpu)
    }
}
