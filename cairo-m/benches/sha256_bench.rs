use cairo_m::{prepare, prove, verify};
use cairo_m_common::{InputValue, Program};
use cairo_m_prover::{adapter::import_from_runner_output, public_data::PublicData};
use cairo_m_runner::run_cairo_program;
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    utils::harness::BenchProperties::default(),
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
