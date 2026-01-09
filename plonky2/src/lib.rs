use std::borrow::Cow;
use utils::harness::{AuditStatus, BenchProperties};

pub mod bench;
pub mod circuit;

pub const PLONKY2_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Plonky2"), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    field_curve: Cow::Borrowed("Goldilocks"), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    iop: Cow::Borrowed("FRI"), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    pcs: Some(Cow::Borrowed("FRI")), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    arithm: Cow::Borrowed("Plonkish"), // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    is_zk: true, // https://github.com/0xPolygonZero/plonky2/blob/main/plonky2/plonky2.pdf
    security_bits: 100, // https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#security
    is_pq: true, // hash-based PCS
    is_maintained: false, // deprecated: https://github.com/0xPolygonZero/plonky2?tab=readme-ov-file#%EF%B8%8F-plonky2-deprecation-notice
    is_audited: AuditStatus::Audited, // https://github.com/0xPolygonZero/plonky2/tree/main/audits
    isa: None,
};
