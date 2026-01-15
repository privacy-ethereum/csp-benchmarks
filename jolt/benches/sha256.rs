use ere_jolt::compiler::RustRv32imaCustomized;
use jolt::{
    execution_cycles, prepare_sha256, preprocessing_size, proof_size, prove_sha256, verify_sha256,
};
use std::collections::HashMap;
use utils::{
    harness::ProvingSystem,
    zkvm::{SHA256_BENCH, helpers::load_or_compile_program},
};

utils::define_benchmark_harness!(
    BenchTarget::Sha256,
    ProvingSystem::Jolt,
    None,
    "sha256_mem_jolt",
    utils::harness::BenchProperties {
        is_zkvm: true,
        ..Default::default()
    },
    {
        let mut programs = HashMap::new();
        for input_size in utils::metadata::selected_byte_inputs() {
            programs.insert(
                input_size,
                load_or_compile_program(
                    &RustRv32imaCustomized,
                    &format!("{}_{}", SHA256_BENCH, input_size),
                ),
            );
        }
        programs
    },
    prepare_sha256,
    |_, _| 0,
    prove_sha256,
    verify_sha256,
    preprocessing_size,
    proof_size,
    execution_cycles
);
