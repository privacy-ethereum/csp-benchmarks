use ::circuit::Circuit;
use bin::executor::dump_proof_and_claimed_v;
use bin::executor::load_proof_and_claimed_v;
use bin::executor::prove;
use bin::executor::verify;
use circuit_std_rs::sha256::m31::sha256_var_bytes;
use expander_compiler::frontend::*;
use gkr::Prover;
use gkr::Verifier;
use gkr_engine::GKRScheme;
use gkr_engine::M31x1Config;
use gkr_engine::MPIConfig;
use gkr_engine::MPIEngine;
use gkr_engine::MPISharedMemory;
use gkr_engine::{FieldEngine, GKREngine};
use gkr_hashers::SHA256hasher;
use mersenne31::M31;
use poly_commit::OrionPCSForGKR;
use rand::RngCore;
use serdes::ExpSerde;
use sha2::{Digest, Sha256};
use transcript::BytesHashTranscript;

// ref: https://github.com/PolyhedraZK/ExpanderCompilerCollection/blob/master/circuit-std-rs/tests/sha256_gf2.rs#L89-L137
const INPUT_LEN: usize = 1024;
const OUTPUT_LEN: usize = 32; // FIXED 32

declare_circuit!(SHA256Circuit {
    input: [Variable; INPUT_LEN],
    output: [Variable; OUTPUT_LEN],
});

pub fn check_sha256_var<C: Config, B: RootAPI<C>>(
    api: &mut B,
    #[allow(clippy::ptr_arg)] data: &Vec<Variable>, //  Expander API is enforcing it to be a Vec
) -> Vec<Variable> {
    let msg_len = data.len() - OUTPUT_LEN;
    let expected = data[msg_len..].to_vec();
    let computed = sha256_var_bytes(api, &data[..msg_len]);

    for i in 0..OUTPUT_LEN {
        api.assert_is_equal(computed[i], expected[i]);
    }
    computed
}

#[derive(Default, Debug, Clone, PartialOrd, Ord, Hash, PartialEq, Eq, Copy)]
pub struct M31SingleConfig<'a> {
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GKREngine for M31SingleConfig<'a> {
    type FieldConfig = M31x1Config;
    type MPIConfig = MPIConfig<'a>;
    type TranscriptConfig = BytesHashTranscript<SHA256hasher>;
    type PCSConfig = OrionPCSForGKR<M31x1Config, M31>;
    const SCHEME: GKRScheme = GKRScheme::Vanilla;
}

pub type M31SnglConfig = M31SingleConfig<'static>;

impl Config for M31SnglConfig {
    const CONFIG_ID: usize = 1;
}

impl Define<M31SnglConfig> for SHA256Circuit<Variable> {
    fn define<Builder: RootAPI<M31SnglConfig>>(&self, api: &mut Builder) {
        let mut data = self.input.to_vec();
        data.extend(self.output.to_vec());
        api.memorized_simple_call(check_sha256_var, &data);
    }
}

fn main() {
    let compile_result = compile(&SHA256Circuit::default(), CompileOptions::default()).unwrap();
    let mut rng = rand::thread_rng();
    let data = [rng.next_u32() as u8; INPUT_LEN];
    let mut hash = Sha256::new();
    hash.update(data);
    let output = hash.finalize();
    let mut assignment = SHA256Circuit::default();
    for (i, input_byte) in data.iter().enumerate().take(INPUT_LEN) {
        assignment.input[i] = M31::from(*input_byte as u32);
    }
    for (i, output_byte) in output.iter().enumerate().take(OUTPUT_LEN) {
        assignment.output[i] = M31::from(*output_byte as u32);
    }
    let witness = compile_result
        .witness_solver
        .solve_witness_with_hints(&assignment, &EmptyHintCaller)
        .unwrap();
    let output = compile_result.layered_circuit.run(&witness);
    assert_eq!(output, vec![true]);

    // create "circuit.txt"
    let file = std::fs::File::create("build/circuit.txt").unwrap();
    let writer = std::io::BufWriter::new(file);
    compile_result
        .layered_circuit
        .serialize_into(writer)
        .unwrap();

    // create "witness.txt"
    let file = std::fs::File::create("build/witness.txt").unwrap();
    let writer = std::io::BufWriter::new(file);
    witness.serialize_into(writer).unwrap();

    let universe = MPIConfig::init().unwrap();
    let world = universe.world();
    let mpi_config = MPIConfig::prover_new(Some(&universe), Some(&world));

    let circuit_file = "build/circuit.txt";
    let witness_file = "build/witness.txt";
    let output_proof_file = "build/proof.txt";

    let (mut circuit, mut window) =
        Circuit::<M31x1Config>::prover_load_circuit::<M31SnglConfig>(circuit_file, &mpi_config);
    let prover = Prover::<M31SnglConfig>::new(mpi_config.clone());

    circuit.prover_load_witness_file(witness_file, &mpi_config);
    let (claimed_v, proof) = prove::<M31SnglConfig>(&mut circuit, mpi_config.clone());

    if prover.mpi_config.is_root() {
        let bytes =
            dump_proof_and_claimed_v(&proof, &claimed_v).expect("Unable to serialize proof.");
        std::fs::write(output_proof_file, bytes).expect("Unable to write proof to file.");
    }
    circuit.discard_control_of_shared_mem();
    mpi_config.free_shared_mem(&mut window);

    let verifier = Verifier::<M31SnglConfig>::new(mpi_config);

    println!("loading circuit file");

    let mut circuit = Circuit::<M31x1Config>::verifier_load_circuit::<M31SnglConfig>(circuit_file);

    println!("loading witness file");

    circuit.verifier_load_witness_file(witness_file, &verifier.mpi_config);

    println!("loading proof file");

    let bytes = std::fs::read(output_proof_file).expect("Unable to read proof from file.");
    let (proof, claimed_v) =
        load_proof_and_claimed_v::<<M31x1Config as FieldEngine>::ChallengeField>(&bytes)
            .expect("Unable to deserialize proof.");

    println!("verifying proof");

    assert!(verify::<M31SnglConfig>(
        &mut circuit,
        verifier.mpi_config,
        &proof,
        &claimed_v
    ));

    println!("success");
}
