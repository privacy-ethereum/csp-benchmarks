use cairo_m::{prepare, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    |input_size| { prepare(input_size) },
    |(runner_output, _)| { prove(runner_output) },
    |_, proof| { verify(proof) },
    |(_, compiled_program)| { compiled_program.len() },
    |proof| proof.stark_proof.size_estimate()
);
