use plonky2::{
    field::goldilocks_field::GoldilocksField,
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder,
        circuit_data::{CircuitConfig, CircuitData, ProverCircuitData, VerifierCircuitData},
        config::{GenericConfig, PoseidonGoldilocksConfig},
        proof::ProofWithPublicInputs,
    },
};

use crate::circuit::{array_to_bits, make_circuits};

const D: usize = 2;
type C = PoseidonGoldilocksConfig;
type F = <C as GenericConfig<D>>::F;

pub fn verify(data: &VerifierCircuitData<F, C, D>, proof: ProofWithPublicInputs<F, C, D>) {
    data.verify(proof).unwrap()
}

pub fn prove(
    data: &ProverCircuitData<F, C, D>,
    pw: PartialWitness<F>,
) -> ProofWithPublicInputs<GoldilocksField, C, D> {
    data.prove(pw).unwrap()
}

pub fn sha256_prepare(input_size: usize) -> (CircuitData<F, C, D>, PartialWitness<F>) {
    let (msg, hash) = utils::generate_sha256_input(input_size);

    let msg_bits = array_to_bits(&msg);
    let len = msg.len() * 8;
    println!("block count: {}", (len + 65).div_ceil(512));
    const D: usize = 2;
    type C = PoseidonGoldilocksConfig;
    type F = <C as GenericConfig<D>>::F;
    let mut builder = CircuitBuilder::<F, D>::new(CircuitConfig::standard_recursion_config());
    let targets = make_circuits(&mut builder, len as u64);
    let mut pw = PartialWitness::new();

    for (i, msg_bit) in msg_bits.iter().enumerate().take(len) {
        pw.set_bool_target(targets.message[i], *msg_bit).unwrap();
    }

    let expected_res = array_to_bits(hash.as_slice());
    for (i, expected_res_bit) in expected_res.iter().enumerate() {
        if *expected_res_bit {
            builder.assert_one(targets.digest[i].target);
        } else {
            builder.assert_zero(targets.digest[i].target);
        }
    }

    println!(
        "Constructing inner proof with {} gates",
        builder.num_gates()
    );
    (builder.build::<C>(), pw)
}
