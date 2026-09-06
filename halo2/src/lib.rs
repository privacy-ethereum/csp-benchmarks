use std::borrow::Cow;
use utils::harness::{AuditStatus, BenchProperties};

pub mod bench;
pub mod circuits;

pub const HALO2_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Halo2"),
    field_curve: Cow::Borrowed("Bn254"),
    iop: Cow::Borrowed("Plonkish"),
    pcs: Some(Cow::Borrowed("KZG")), // SHPLONK multi-open over BN254
    arithm: Cow::Borrowed("Plonkish"),
    //is_zk: flase for now. Can be proven later
    is_zk: false,
    is_zkvm: false,
    security_bits: 100, // BN254 pairing security after exTNFS estimates, see https://eips.ethereum.org/assets/eip-3068/2017-334.pdf
    is_pq: false,
    is_maintained: true,
    is_audited: AuditStatus::PartiallyAudited, // https://github.com/axiom-crypto/halo2-lib/tree/main/audits
    isa: None,
};
