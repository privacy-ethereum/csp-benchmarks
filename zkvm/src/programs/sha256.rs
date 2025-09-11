//! SHA256 benchmark module.

use crate::traits::{BenchmarkConfig, DataGenerator, InputBuilder, Program};
use ere_jolt::EreJolt;
use ere_risc0::EreRisc0;
use ere_sp1::EreSP1;
use rand::{RngCore, SeedableRng, rngs::StdRng};
use zkvm_interface::Input;

/// SHA256
pub struct Sha256;
impl Program for Sha256 {
    const NAME: &'static str = "sha256";
}

/// SHA256 benchmark configuration
pub struct Sha256Config {
    pub input_size: usize,
}

impl Sha256Config {
    pub fn new(input_size: usize) -> Self {
        Self { input_size }
    }
}

impl BenchmarkConfig for Sha256Config {}

/// SHA256 input data generator
pub struct Sha256Generator;

impl DataGenerator<Sha256Config> for Sha256Generator {
    type Data = Vec<u8>;

    fn generate(&self, config: &Sha256Config) -> (Self::Data, usize) {
        let mut rng = StdRng::seed_from_u64(1337);
        let mut data = vec![0; config.input_size];

        rng.fill_bytes(&mut data);

        (data, config.input_size)
    }
}

// Default input builder
macro_rules! impl_byte_vec_input_builder {
    ($zkvm:ty) => {
        impl InputBuilder<Sha256> for $zkvm {
            type Data = Vec<u8>;

            fn build_input(data: Self::Data) -> Input {
                let mut input = Input::new();
                input.write(data);
                input
            }
        }
    };
}

impl_byte_vec_input_builder!(EreRisc0);
impl_byte_vec_input_builder!(EreSP1);
impl_byte_vec_input_builder!(EreJolt);
