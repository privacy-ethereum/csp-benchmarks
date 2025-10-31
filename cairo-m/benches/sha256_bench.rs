use cairo_m::{prepare, prove, verify};
use cairo_m_common::{InputValue, Program};
use cairo_m_prover::{adapter::import_from_runner_output, public_data::PublicData};
use cairo_m_runner::run_cairo_program;
use utils::harness::{AuditStatus, BenchProperties, ProvingSystem};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    BenchProperties {
        proving_system: Some("Circle STARK".to_string()), // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#welcome-to-cairo-m
        field_curve: Some("M31".to_string()), // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#welcome-to-cairo-m
        iop: Some("Circle-FRI".to_string()), // https://eprint.iacr.org/2024/278.pdf
        pcs: Some("Circle-FRI".to_string()), // https://eprint.iacr.org/2024/278.pdf
        arithm: Some("AIR".to_string()),
        is_zk: Some(false),
        security_bits: Some(96), // https://github.com/kkrt-labs/cairo-m/blob/main/crates/prover/src/prover_config.rs#L13-L20
        is_pq: Some(true),       // hash-based PCS
        is_maintained: Some(true), // https://github.com/kkrt-labs
        is_audited: Some(AuditStatus::NotAudited), // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#about
        isa: Some("Cairo ISA".to_string()), // https://github.com/kkrt-labs/cairo-m/blob/main/docs/design.md
    },
    |input_size| { prepare(input_size) },
    |_| 0,
    |(program, (entrypoint_name, runner_inputs))| {
        prove(program, (entrypoint_name, runner_inputs))
    },
    |_, proof| { verify(proof) },
    |(compiled_program, _)| { compiled_program.len() },
    |proof| proof.stark_proof.size_estimate(),
    |(program, (entrypoint_name, runner_inputs)): &(Program, (String, Vec<InputValue>))| {
        // Run/Execute the program
        let runner_output = run_cairo_program(
            program,
            entrypoint_name.as_str(),
            runner_inputs.as_slice(),
            Default::default(),
        )
        .expect("failed to run cairo program");

        // Proof Generation
        let segment = runner_output
            .vm
            .segments
            .clone()
            .into_iter()
            .next()
            .unwrap();

        let prover_input =
            import_from_runner_output(segment, runner_output.public_address_ranges.clone())
                .expect("Failed to import runner output for proof generation");

        PublicData::new(&prover_input).clock.0 as u64
    }
);
