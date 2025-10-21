use cairo_m::{prepare, prove, verify};
use utils::harness::ProvingSystem;

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::CairoM,
    None,
    "sha256_mem_cairo_m",
    |input_size| { prepare(input_size) },
    |(program, input_size)| { prove(program, *input_size) },
    |_, proof| { verify(proof) },
    |(compiled_program, _)| { compiled_program.len() },
    |proof| proof.stark_proof.size_estimate()
);
