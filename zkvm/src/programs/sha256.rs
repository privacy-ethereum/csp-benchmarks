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

impl DataGenerator<Sha256, Sha256Config> for Sha256Generator {
    type Data = Vec<u8>;

    fn generate(&self, config: &Sha256Config) -> (Self::Data, usize) {
        let mut rng = StdRng::seed_from_u64(1337);
        let mut data = vec![0; config.input_size];

        rng.fill_bytes(&mut data);

        (data, config.input_size)
    }
}

impl InputBuilder<Sha256> for EreRisc0 {
    type Data = Vec<u8>;

    fn build_input(data: Self::Data) -> Input {
        let static_data: &'static [u8] = Box::leak(data.into_boxed_slice());
        let mut input = Input::new();
        input.write(static_data);
        input
    }
}

impl InputBuilder<Sha256> for EreSP1 {
    type Data = Vec<u8>;

    fn build_input(data: Self::Data) -> Input {
        let mut input = Input::new();
        input.write(data);
        input
    }
}

impl InputBuilder<Sha256> for EreJolt {
    type Data = Vec<u8>;

    fn build_input(data: Self::Data) -> Input {
        let mut input = Input::new();
        input.write(data);
        input
    }
}
