// `zkevm_hashes` re-exports `halo2_proofs` privately (`use halo2_base::halo2_proofs;`),
// so halo2 types have to be imported from `halo2_base` instead.
use halo2_base::{
    SKIP_FIRST_PASS,
    halo2_proofs::{
        circuit::{Layouter, SimpleFloorPlanner},
        halo2curves::bn256::Fr,
        plonk::{Circuit, ConstraintSystem, Error},
    },
};
use zkevm_hashes::{
    keccak::vanilla::{
        KeccakCircuitConfig, KeccakConfigParams, keccak_packed_multi::get_keccak_capacity,
        witness::multi_keccak,
    },
    sha256::vanilla::{columns::Sha256CircuitConfig, util::get_sha2_capacity},
};

/// SHA-256 over a single message, sized to `num_rows` usable rows.
#[derive(Clone, Default)]
pub struct Sha256BitCircuit {
    inputs: Vec<Vec<u8>>,
    num_rows: usize,
}

impl Sha256BitCircuit {
    pub fn new(num_rows: usize, inputs: Vec<Vec<u8>>) -> Self {
        Self { inputs, num_rows }
    }
}

impl Circuit<Fr> for Sha256BitCircuit {
    type Config = Sha256CircuitConfig<Fr>;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = ();

    fn without_witnesses(&self) -> Self {
        Self {
            inputs: vec![],
            num_rows: self.num_rows,
        }
    }

    fn configure(meta: &mut ConstraintSystem<Fr>) -> Self::Config {
        Sha256CircuitConfig::new(meta)
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        layouter.assign_region(
            || "sha256 circuit",
            |mut region| {
                config.multi_sha256(
                    &mut region,
                    self.inputs.clone(),
                    Some(get_sha2_capacity(self.num_rows)),
                );
                Ok(())
            },
        )
    }
}

/// Keccak-256 over a single message, sized to `num_rows` usable rows.
#[derive(Clone, Default)]
pub struct KeccakCircuit {
    config: KeccakConfigParams,
    inputs: Vec<Vec<u8>>,
    num_rows: usize,
}

impl KeccakCircuit {
    pub fn new(config: KeccakConfigParams, num_rows: usize, inputs: Vec<Vec<u8>>) -> Self {
        Self {
            config,
            inputs,
            num_rows,
        }
    }
}

impl Circuit<Fr> for KeccakCircuit {
    type Config = KeccakCircuitConfig<Fr>;
    type FloorPlanner = SimpleFloorPlanner;
    type Params = KeccakConfigParams;

    fn params(&self) -> Self::Params {
        self.config
    }

    fn without_witnesses(&self) -> Self {
        Self {
            config: self.config,
            inputs: vec![],
            num_rows: self.num_rows,
        }
    }

    fn configure_with_params(
        meta: &mut ConstraintSystem<Fr>,
        params: Self::Params,
    ) -> Self::Config {
        // The keccak config only allocates SecondPhase advice columns; halo2 requires
        // at least one FirstPhase column to exist, so add an empty one.
        meta.advice_column();
        KeccakCircuitConfig::new(meta, params)
    }

    fn configure(_: &mut ConstraintSystem<Fr>) -> Self::Config {
        unreachable!("keccak circuit is configured via `configure_with_params`")
    }

    fn synthesize(
        &self,
        config: Self::Config,
        mut layouter: impl Layouter<Fr>,
    ) -> Result<(), Error> {
        let params = config.parameters;
        config.load_aux_tables(&mut layouter, params.k)?;
        let mut first_pass = SKIP_FIRST_PASS;
        layouter.assign_region(
            || "keccak circuit",
            |mut region| {
                if first_pass {
                    first_pass = false;
                    return Ok(());
                }
                let (witness, _) = multi_keccak(
                    &self.inputs,
                    Some(get_keccak_capacity(self.num_rows, params.rows_per_round)),
                    params,
                );
                config.assign(&mut region, &witness);
                Ok(())
            },
        )
    }
}
