use cairo_m::{prepare, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    utils::harness::BenchProperties::default(),
    |input_size| { prepare(input_size) },
    |(program, (entrypoint_name, runner_inputs))| {
        prove(program, (entrypoint_name, runner_inputs))
    },
    |_, proof| { verify(proof) },
    |(compiled_program, _)| { compiled_program.len() },
    |proof| proof.stark_proof.size_estimate()
);
