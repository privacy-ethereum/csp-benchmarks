use ark_serialize::CanonicalSerialize;
use noirc_abi::AbiVisibility;
use provekit_common::{HashConfig, NoirProof, Verifier, noir_proof_scheme::NoirProofScheme};
use provekit_prover::{Groth16CommitmentInfo, Groth16Prover, Prove, Prover};
use provekit_r1cs_compiler::NoirCompiler;
use provekit_verifier::Verify;
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use utils::generate_ecdsa_input;
use utils::harness::{AuditStatus, BenchProperties};

const WORKSPACE_ROOT: &str = "circuits";

pub const PROVEKIT_GROTH16_PROPS: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Groth16+BSB22"),
    field_curve: Cow::Borrowed("Bn254"),
    iop: Cow::Borrowed("Groth16"),
    pcs: None,
    arithm: Cow::Borrowed("R1CS"),
    is_zk: true,
    is_zkvm: false,
    security_bits: 128,
    is_pq: false,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

fn workspace_root() -> PathBuf {
    std::env::current_dir()
        .expect("Failed to get current directory")
        .join(WORKSPACE_ROOT)
}

fn compile_package(package: &str) -> (NoirProofScheme, PathBuf) {
    let workspace_root = workspace_root();
    let output = Command::new("nargo")
        .args([
            "compile",
            "--package",
            package,
            "--silence-warnings",
            "--skip-brillig-constraints-check",
        ])
        .current_dir(&workspace_root)
        .output()
        .expect("Failed to run nargo compile");
    if !output.status.success() {
        panic!(
            "Compilation failed for {package}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let circuit_path = workspace_root
        .join("target")
        .join(format!("{package}.json"));
    let proof_scheme = NoirCompiler::from_file(&circuit_path, HashConfig::default())
        .unwrap_or_else(|e| panic!("Failed to load proof scheme for {package}: {e}"));
    (proof_scheme, circuit_path)
}

fn package_dir(package: &str) -> PathBuf {
    let suffix = package.strip_prefix("csp_").unwrap_or(package);
    workspace_root().join(suffix)
}

fn write_prover_toml(package: &str, content: &str) -> PathBuf {
    let dir = package_dir(package);
    fs::create_dir_all(&dir).expect("Failed to create circuit dir");
    let toml_path = dir.join("Prover.toml");
    fs::write(&toml_path, content).expect("Failed to write Prover.toml");
    toml_path
}

fn join_u8(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(u8::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn join_field_strings(fields: &[String]) -> String {
    fields
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ")
}

pub struct GrothPrepared {
    pub prover: Prover,
    pub verifier: Verifier,
    pub toml_path: PathBuf,
    pub pk_size: usize,
    pub num_constraints: usize,
}

fn build_groth16_state(scheme: NoirProofScheme) -> (Prover, Verifier, usize) {
    let hash_config = HashConfig::default();
    let NoirProofScheme::Noir(d) = scheme else {
        panic!("Groth16 backend requires the Noir compiler (not Mavros)");
    };

    let abi = d.witness_generator.abi.clone();
    let mut r1cs = d.r1cs;
    let program = d.program;
    let split_witness_builders = d.split_witness_builders;
    let witness_generator = d.witness_generator;
    let w1_size = d.whir_for_witness.w1_size;
    let challenge_offsets = d.whir_for_witness.challenge_offsets.clone();

    // The Noir compiler leaves `num_public_inputs` unset on the R1CS (WHIR
    // tracks public inputs out of band); Groth16 needs it to classify wires.
    let mut n_public: usize = abi
        .parameters
        .iter()
        .filter(|p| p.is_public())
        .map(|p| p.typ.field_count() as usize)
        .sum();
    if let Some(ret) = &abi.return_type {
        if matches!(ret.visibility, AbiVisibility::Public) {
            n_public += ret.abi_type.field_count() as usize;
        }
    }
    r1cs.num_public_inputs = n_public;
    let num_public = 1 + r1cs.num_public_inputs;

    let num_challenges = challenge_offsets.len();
    let private_w1_wires: Vec<usize> = (num_public..w1_size).collect();
    let public_committed: Vec<usize> = (1..num_public).collect();

    let (commitment_info, groth16_ci, num_challenges_per_commitment) =
        if num_challenges > 0 && !private_w1_wires.is_empty() {
            let mut sorted_challenge_indices: Vec<usize> = challenge_offsets
                .iter()
                .map(|&offset| w1_size + offset)
                .collect();
            sorted_challenge_indices.sort_unstable();

            let ci = Groth16CommitmentInfo {
                public_committed: public_committed.clone(),
                private_committed: private_w1_wires.clone(),
                challenge_indices: sorted_challenge_indices.clone(),
            };
            let g16_ci = vec![provekit_groth16::CommitmentInfo {
                public_and_commitment_committed: public_committed,
                private_committed: private_w1_wires.clone(),
                commitment_index: sorted_challenge_indices[0],
                nb_public_committed: r1cs.num_public_inputs,
            }];
            (vec![ci], g16_ci, vec![num_challenges])
        } else {
            (vec![], vec![], vec![])
        };

    let challenge_wire_indices: Vec<usize> = commitment_info
        .iter()
        .flat_map(|ci| ci.challenge_indices.iter().copied())
        .collect();

    let (pk, vk) = provekit_groth16::setup::setup(
        &r1cs,
        &groth16_ci,
        &num_challenges_per_commitment,
        &challenge_wire_indices,
    )
    .expect("Groth16 trusted setup failed");

    let pk_size = pk.serialized_size(ark_serialize::Compress::Yes);

    let mut vk_bytes = Vec::new();
    vk.serialize_uncompressed(&mut vk_bytes)
        .expect("serialize Groth16 verifying key");

    let prover = Prover::Groth16(Groth16Prover {
        program,
        r1cs: r1cs.clone(),
        split_witness_builders,
        witness_generator,
        groth16_pk: pk,
        commitment_info,
    });

    let verifier = Verifier {
        hash_config,
        r1cs,
        whir_for_witness: None,
        abi,
        groth16_vk: Some(vk_bytes),
    };

    (prover, verifier, pk_size)
}

fn prepared_from(package: &str, scheme: NoirProofScheme, toml: String) -> GrothPrepared {
    let toml_path = write_prover_toml(package, &toml);
    let (prover, verifier, pk_size) = build_groth16_state(scheme);
    let Prover::Groth16(g) = &prover else {
        unreachable!()
    };
    let num_constraints = g.r1cs.num_constraints();
    GrothPrepared {
        prover,
        verifier,
        toml_path,
        pk_size,
        num_constraints,
    }
}

pub fn prepare_sha256(input_size: usize) -> GrothPrepared {
    let package = format!("csp_sha256_{input_size}");
    let (scheme, _) = compile_package(&package);
    let (data, _digest) = utils::generate_sha256_input(input_size);
    let toml = format!("input = [{}]\ninput_len = {input_size}", join_u8(&data));
    prepared_from(&package, scheme, toml)
}

pub fn prepare_keccak(input_size: usize) -> GrothPrepared {
    let package = format!("csp_keccak_{input_size}");
    let (scheme, _) = compile_package(&package);
    let (data, digest) = utils::generate_keccak_input(input_size);
    let toml = format!(
        "msg = [{}]\nmessage_size = {input_size}\nresult = [{}]",
        join_u8(&data),
        join_u8(&digest),
    );
    prepared_from(&package, scheme, toml)
}

pub fn prepare_poseidon(input_size: usize) -> GrothPrepared {
    let package = format!("csp_poseidon_{input_size}");
    let (scheme, _) = compile_package(&package);
    let fields = utils::generate_poseidon_input_strings(input_size);
    let toml = format!("inputs = [{}]", join_field_strings(&fields));
    prepared_from(&package, scheme, toml)
}

pub fn prepare_poseidon2(input_size: usize) -> GrothPrepared {
    let package = format!("csp_poseidon2_{input_size}");
    let (scheme, _) = compile_package(&package);
    let fields = utils::generate_poseidon_input_strings(input_size);
    let toml = format!("inputs = [{}]", join_field_strings(&fields));
    prepared_from(&package, scheme, toml)
}

pub fn prepare_ecdsa(_: usize) -> GrothPrepared {
    let package = "csp_ecdsa_p256";
    let (scheme, _) = compile_package(package);
    let (digest, (pub_key_x, pub_key_y), signature) = generate_ecdsa_input();
    let toml = format!(
        "hashed_message = [{}]\npub_key_x = [{}]\npub_key_y = [{}]\nsignature = [{}]",
        join_u8(&digest),
        join_u8(&pub_key_x),
        join_u8(&pub_key_y),
        join_u8(&signature),
    );
    prepared_from(package, scheme, toml)
}

pub fn prove(prepared: &GrothPrepared) -> NoirProof {
    prepared
        .prover
        .clone()
        .prove_with_toml(&prepared.toml_path)
        .expect("Groth16 proof generation failed")
}

pub fn verify(prepared: &GrothPrepared, proof: &NoirProof) -> Result<(), &'static str> {
    let mut verifier = prepared.verifier.clone();
    verifier.verify(proof).map_err(|_| "Proof is not valid")
}

pub fn preprocessing_size(prepared: &GrothPrepared) -> usize {
    prepared.pk_size
}

pub fn proof_size(proof: &NoirProof) -> usize {
    match proof {
        NoirProof::Groth16 { groth16_proof, .. } => groth16_proof.len(),
        _ => unreachable!("provekit-groth16-bench only produces NoirProof::Groth16"),
    }
}

pub fn num_constraints(prepared: &GrothPrepared) -> usize {
    prepared.num_constraints
}
