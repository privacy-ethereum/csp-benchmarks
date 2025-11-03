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
    BenchProperties::new(
        "Circle STARK", // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#welcome-to-cairo-m
        "M31", // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#welcome-to-cairo-m
        "Circle-FRI", // https://eprint.iacr.org/2024/278.pdf
        Some("Circle-FRI"), // https://eprint.iacr.org/2024/278.pdf
        "AIR",
        false,
        96, // https://github.com/kkrt-labs/cairo-m/blob/main/crates/prover/src/prover_config.rs#L13-L20
        true, // hash-based PCS
        true, // https://github.com/kkrt-labs
        AuditStatus::NotAudited, // https://github.com/kkrt-labs/cairo-m/?tab=readme-ov-file#about
        Some("Cairo ISA"), // https://github.com/kkrt-labs/cairo-m/blob/main/docs/design.md
    ),
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
