use sp1_build::include_elf;
use sp1_sdk::{
    Elf, ProvingKey, SP1ProofWithPublicValues,
    blocking::{ProveRequest, Prover, ProverClient, SP1Stdin},
};
use utils::harness::{AuditStatus, BenchProperties};

pub const PRIVATE_TX_DEPTH: usize = evm_sp1_private_tx::PRIVATE_TX_DEPTH;

const ELF: Elf = include_elf!("evm-sp1-private-tx");

type EnvProver = sp1_sdk::blocking::EnvProver;
type EnvProvingKey = <EnvProver as Prover>::ProvingKey;

pub type PrivateTxProof = SP1ProofWithPublicValues;

pub struct EvmSp1State {
    prover: EnvProver,
    pk: EnvProvingKey,
    elf_size: usize,
    private_tx_cycles: u64,
}

pub struct PreparedPrivateTx {
    input: Vec<u8>,
    expected_public_values: Vec<u8>,
    elf_size: usize,
    cycles: u64,
}

impl PreparedPrivateTx {
    fn stdin(&self) -> SP1Stdin {
        let mut stdin = SP1Stdin::new();
        stdin.write(&self.input);
        stdin
    }
}

pub fn setup_private_tx() -> EvmSp1State {
    let prover = ProverClient::from_env();
    let pk = prover.setup(ELF.clone()).expect("SP1 setup failed");
    let (input, _) = utils::generate_private_tx_input(PRIVATE_TX_DEPTH);
    let private_tx_cycles = execute_cycles(&prover, &input);

    EvmSp1State {
        prover,
        pk,
        elf_size: ELF.len(),
        private_tx_cycles,
    }
}

pub fn prepare_private_tx(depth: usize, state: &EvmSp1State) -> PreparedPrivateTx {
    assert_eq!(
        depth, PRIVATE_TX_DEPTH,
        "evm-sp1 private_tx only supports depth {PRIVATE_TX_DEPTH}"
    );

    let (input, expected_public_values) = utils::generate_private_tx_input(depth);
    let evm_output = evm_sp1_private_tx::execute_private_tx(&input)
        .expect("generated private_tx EVM input failed");
    assert_eq!(
        evm_output.as_slice(),
        expected_public_values.as_slice(),
        "private_tx EVM output does not match utility evaluator"
    );

    PreparedPrivateTx {
        input,
        expected_public_values,
        elf_size: state.elf_size,
        cycles: state.private_tx_cycles,
    }
}

pub fn prove_private_tx(prepared: &PreparedPrivateTx, state: &&EvmSp1State) -> PrivateTxProof {
    state
        .prover
        .prove(&state.pk, prepared.stdin())
        .compressed()
        .run()
        .expect("SP1 private_tx proving failed")
}

pub fn verify_private_tx(
    prepared: &PreparedPrivateTx,
    proof: &PrivateTxProof,
    state: &&EvmSp1State,
) {
    assert_eq!(
        proof.public_values.as_slice(),
        prepared.expected_public_values.as_slice(),
        "private_tx public output mismatch"
    );
    state
        .prover
        .verify(proof, state.pk.verifying_key(), None)
        .expect("SP1 private_tx verification failed");
}

pub fn preprocessing_size<SharedState>(prepared: &PreparedPrivateTx, _: &SharedState) -> usize {
    prepared.elf_size
}

pub fn proof_size<SharedState>(proof: &PrivateTxProof, _: &SharedState) -> usize {
    bincode::serialize(proof)
        .expect("failed to serialize SP1 proof")
        .len()
}

pub fn execution_cycles(prepared: &PreparedPrivateTx) -> u64 {
    prepared.cycles
}

pub fn num_constraints<SharedState>(_: &PreparedPrivateTx, _: &SharedState) -> usize {
    0
}

pub fn uses_precompile(_: usize) -> bool {
    false
}

pub fn evm_sp1_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "SP1",
        "BabyBear",
        "STARK",
        Some("FRI"),
        "AIR",
        false,
        true,
        100,
        true,
        true,
        AuditStatus::NotAudited,
        Some("EVM via revm on SP1"),
    )
}

fn execute_cycles(prover: &EnvProver, input: &[u8]) -> u64 {
    let mut stdin = SP1Stdin::new();
    stdin.write(&input.to_vec());
    let (_, report) = prover
        .execute(ELF.clone(), stdin)
        .run()
        .expect("SP1 private_tx execution failed");
    report.total_instruction_count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::private_tx::{
        encode_private_tx_input, evaluate_private_tx, generate_private_tx_case,
    };

    #[test]
    fn depth_32_evm_execution_matches_utility_evaluator() {
        let input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        let encoded = encode_private_tx_input(&input);
        let evm_output = evm_sp1_private_tx::execute_private_tx(&encoded).unwrap();
        let expected = evaluate_private_tx(&input).unwrap();
        assert_eq!(evm_output, expected);
    }

    #[test]
    fn corrupted_branch_root_fails_evm_execution() {
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.input_branches[0].expected_root += 1;
        let encoded = encode_private_tx_input(&input);
        assert_eq!(
            evm_sp1_private_tx::execute_private_tx(&encoded).unwrap_err(),
            evm_sp1_private_tx::PrivateTxEvmError::EvmReverted
        );
    }

    #[test]
    fn balance_mismatch_fails_evm_execution() {
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.output_notes[0].amount += 1;
        let encoded = encode_private_tx_input(&input);
        assert_eq!(
            evm_sp1_private_tx::execute_private_tx(&encoded).unwrap_err(),
            evm_sp1_private_tx::PrivateTxEvmError::EvmReverted
        );
    }

    #[test]
    fn unsupported_depth_is_rejected_by_adapter() {
        let input = generate_private_tx_case(16);
        let encoded = encode_private_tx_input(&input);
        assert_eq!(
            evm_sp1_private_tx::execute_private_tx(&encoded).unwrap_err(),
            evm_sp1_private_tx::PrivateTxEvmError::InvalidInputLength
        );
    }

    #[test]
    #[ignore = "executes the SP1 guest"]
    fn sp1_guest_execute_matches_utility_evaluator() {
        let (input, expected_public_values) = utils::generate_private_tx_input(PRIVATE_TX_DEPTH);
        let mut stdin = SP1Stdin::new();
        stdin.write(&input);

        let prover = ProverClient::from_env();
        let (public_values, _) = prover
            .execute(ELF.clone(), stdin)
            .run()
            .expect("SP1 guest execution failed");

        assert_eq!(public_values.as_slice(), expected_public_values.as_slice());
    }
}
