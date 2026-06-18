use std::mem::size_of;
use std::sync::Once;

use lean_backend::{ArenaVec, PrimeCharacteristicRing, PrimeField32};
use lean_compiler::{ProgramSource, compile_program};
use lean_prover::{
    default_whir_config,
    prove_execution::{ExecutionProof, prove_execution},
    verify_execution::verify_execution,
};
use lean_vm::{Bytecode, ExecutionWitness, F, Hints, PUBLIC_INPUT_LEN, try_execute_bytecode};
use utils::harness::{AuditStatus, BenchProperties};
use utils::private_tx::{
    PRIVATE_TX_INPUT_COUNT, PRIVATE_TX_PUBLIC_OUTPUT_BYTES, PrivateTxInput, evaluate_private_tx,
    generate_private_tx_case, note_commitment,
};

pub const PRIVATE_TX_DEPTH: usize = 32;
const PRIVATE_TX_FIELD_OUTPUTS: usize = 4;
const PRIVATE_TX_BRANCH_CELLS: usize = 1 + PRIVATE_TX_DEPTH + PRIVATE_TX_DEPTH;
const PRIVATE_TX_WITNESS_CELLS: usize = 8 + PRIVATE_TX_INPUT_COUNT * PRIVATE_TX_BRANCH_CELLS;
const PRIVATE_TX_PROGRAM: &str = include_str!("../guest/private_tx/main.py");

static PROVER_SETUP: Once = Once::new();

pub type PrivateTxProof = ExecutionProof;

pub struct PreparedPrivateTx<'a> {
    bytecode: &'a Bytecode,
    public_input: [F; PUBLIC_INPUT_LEN],
    witness: ExecutionWitness,
    expected_output: Vec<u8>,
}

impl<'a> PreparedPrivateTx<'a> {
    pub fn bytecode(&self) -> &Bytecode {
        self.bytecode
    }

    pub fn public_input(&self) -> &[F; PUBLIC_INPUT_LEN] {
        &self.public_input
    }

    pub fn witness(&self) -> &ExecutionWitness {
        &self.witness
    }

    pub fn expected_output(&self) -> &[u8] {
        &self.expected_output
    }
}

pub fn leanvm_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "leanVM",
        "KoalaBear",
        "SuperSpartan + LogUp",
        Some("WHIR"),
        "AIR",
        false,
        true,
        124,
        true,
        true,
        AuditStatus::NotAudited,
        Some("leanVM"),
    )
}

pub fn compile_private_tx() -> Bytecode {
    compile_program(&ProgramSource::Raw(PRIVATE_TX_PROGRAM.to_string()))
}

pub fn prepare_private_tx(depth: usize, bytecode: &Bytecode) -> PreparedPrivateTx<'_> {
    assert_eq!(
        depth, PRIVATE_TX_DEPTH,
        "leanVM private_tx only supports depth {PRIVATE_TX_DEPTH}"
    );
    let input = generate_private_tx_case(depth);
    prepare_private_tx_input(&input, bytecode).expect("generated private_tx input should prepare")
}

pub fn prove_private_tx<SharedState>(
    prepared: &PreparedPrivateTx<'_>,
    _: &SharedState,
) -> PrivateTxProof {
    setup_prover();
    prove_execution(
        prepared.bytecode,
        &prepared.public_input,
        &prepared.witness,
        &default_whir_config(1),
        false,
    )
    .expect("prove failed")
}

pub fn verify_private_tx<SharedState>(
    prepared: &PreparedPrivateTx<'_>,
    proof: &PrivateTxProof,
    _: &SharedState,
) {
    verify_execution(
        prepared.bytecode,
        &prepared.public_input,
        proof.proof.clone(),
    )
    .expect("verify failed");
    let actual_output = public_input_output_bytes(&prepared.public_input);
    assert_eq!(
        actual_output.as_slice(),
        prepared.expected_output(),
        "private_tx public output mismatch"
    );
}

pub fn preprocessing_size<SharedState>(prepared: &PreparedPrivateTx<'_>, _: &SharedState) -> usize {
    prepared.bytecode.instructions_multilinear().len() * size_of::<u32>()
}

pub fn proof_size<SharedState>(proof: &PrivateTxProof, _: &SharedState) -> usize {
    proof.proof.proof_size_fe() * size_of::<u32>()
}

pub fn execution_cycles(prepared: &PreparedPrivateTx<'_>) -> u64 {
    try_execute_bytecode(
        prepared.bytecode,
        &prepared.public_input,
        &prepared.witness,
        false,
    )
    .expect("execute failed")
    .n_cycles() as u64
}

pub fn num_constraints<SharedState>(_: &PreparedPrivateTx<'_>, _: &SharedState) -> usize {
    0
}

fn setup_prover() {
    PROVER_SETUP.call_once(|| {
        lean_backend::enable_arena();
        lean_backend::parallel::init();
    });
}

fn prepare_private_tx_input<'a>(
    input: &PrivateTxInput,
    bytecode: &'a Bytecode,
) -> anyhow::Result<PreparedPrivateTx<'a>> {
    let expected_output = evaluate_private_tx(input)?.to_vec();
    let prepared = prepare_private_tx_input_unchecked(input, bytecode)?;
    if prepared.expected_output != expected_output {
        anyhow::bail!("private_tx public input does not match expected output");
    }
    Ok(PreparedPrivateTx {
        expected_output,
        ..prepared
    })
}

fn prepare_private_tx_input_unchecked<'a>(
    input: &PrivateTxInput,
    bytecode: &'a Bytecode,
) -> anyhow::Result<PreparedPrivateTx<'a>> {
    let public_input = public_input_from_claims(input)?;
    let witness = build_witness(input, bytecode)?;
    let expected_output = public_input_output_bytes(&public_input);
    Ok(PreparedPrivateTx {
        bytecode,
        public_input,
        witness,
        expected_output,
    })
}

fn build_witness(input: &PrivateTxInput, bytecode: &Bytecode) -> anyhow::Result<ExecutionWitness> {
    if input.depth != PRIVATE_TX_DEPTH {
        anyhow::bail!("leanVM private_tx only supports depth {PRIVATE_TX_DEPTH}");
    }
    for branch in &input.input_branches {
        if branch.siblings.len() != PRIVATE_TX_DEPTH {
            anyhow::bail!("invalid private_tx branch length");
        }
    }

    let mut fields = Vec::with_capacity(PRIVATE_TX_WITNESS_CELLS);
    for note in &input.input_notes {
        fields.push(fe_from_u64(note.owner));
        fields.push(fe_from_i64(note.amount));
    }
    for note in &input.output_notes {
        fields.push(fe_from_u64(note.owner));
        fields.push(fe_from_i64(note.amount));
    }
    for branch in &input.input_branches {
        fields.push(fe_from_i64(branch.expected_root));
        for level in 0..PRIVATE_TX_DEPTH {
            fields.push(fe_from_u64((branch.path_index >> level) & 1));
        }
        for sibling in &branch.siblings {
            fields.push(fe_from_i64(*sibling));
        }
    }

    let mut hints = Hints::default();
    let entries = ArenaVec::from_iter([ArenaVec::from_slice(&fields)]);
    hints.insert(bytecode, "private_tx", entries);
    Ok(ExecutionWitness {
        hints,
        ..Default::default()
    })
}

fn public_input_from_claims(input: &PrivateTxInput) -> anyhow::Result<[F; PUBLIC_INPUT_LEN]> {
    let mut public_input = [F::ZERO; PUBLIC_INPUT_LEN];
    public_input[0] = fe_from_i64(input.input_branches[0].expected_root);
    public_input[1] = fe_from_i64(input.input_branches[1].expected_root);
    public_input[2] = fe_from_i64(note_commitment(&input.output_notes[0])?);
    public_input[3] = fe_from_i64(note_commitment(&input.output_notes[1])?);
    Ok(public_input)
}

fn public_input_output_bytes(public_input: &[F; PUBLIC_INPUT_LEN]) -> Vec<u8> {
    let mut output = [0u8; PRIVATE_TX_PUBLIC_OUTPUT_BYTES];
    for i in 0..PRIVATE_TX_FIELD_OUTPUTS {
        let value = i64::from(public_input[i].as_canonical_u32());
        output[i * 8..(i + 1) * 8].copy_from_slice(&value.to_le_bytes());
    }
    output.to_vec()
}

fn fe_from_u64(value: u64) -> F {
    F::from_u64(value)
}

fn fe_from_i64(value: i64) -> F {
    F::from_i64(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::private_tx::PrivateTxError;

    #[test]
    fn witness_public_input_matches_utils_output() {
        let bytecode = compile_private_tx();
        let input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        let prepared = prepare_private_tx_input(&input, &bytecode).unwrap();
        let expected = evaluate_private_tx(&input).unwrap().to_vec();

        assert_eq!(prepared.expected_output(), expected.as_slice());
        assert_eq!(
            public_input_output_bytes(prepared.public_input()).as_slice(),
            expected.as_slice()
        );
    }

    #[test]
    fn guest_executes_generated_input() {
        let bytecode = compile_private_tx();
        let prepared = prepare_private_tx(PRIVATE_TX_DEPTH, &bytecode);
        let result = try_execute_bytecode(
            prepared.bytecode(),
            prepared.public_input(),
            prepared.witness(),
            false,
        )
        .unwrap();

        assert!(result.n_cycles() > 0);
    }

    #[test]
    fn corrupted_branch_fails_execution() {
        let bytecode = compile_private_tx();
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.input_branches[0].expected_root += 1;
        let prepared = prepare_private_tx_input_unchecked(&input, &bytecode).unwrap();

        assert!(
            try_execute_bytecode(
                prepared.bytecode(),
                prepared.public_input(),
                prepared.witness(),
                false,
            )
            .is_err()
        );
    }

    #[test]
    fn balance_mismatch_fails_execution() {
        let bytecode = compile_private_tx();
        let mut input = generate_private_tx_case(PRIVATE_TX_DEPTH);
        input.output_notes[0].amount += 1;
        let prepared = prepare_private_tx_input_unchecked(&input, &bytecode).unwrap();

        assert_eq!(
            evaluate_private_tx(&input).unwrap_err(),
            PrivateTxError::BalanceMismatch
        );
        assert!(
            try_execute_bytecode(
                prepared.bytecode(),
                prepared.public_input(),
                prepared.witness(),
                false,
            )
            .is_err()
        );
    }

    #[test]
    fn unsupported_depth_is_rejected() {
        let bytecode = compile_private_tx();
        let input = generate_private_tx_case(16);

        assert!(prepare_private_tx_input(&input, &bytecode).is_err());
    }
}
