use std::borrow::Cow;

pub mod bench;
pub mod metadata;
pub mod poseidon;

pub use utils::harness::{AuditStatus, BenchProperties};

pub const EXPANDER_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Libra"),
    field_curve: Cow::Borrowed("M31"),
    iop: Cow::Borrowed("GKR"),
    pcs: Some(Cow::Borrowed("Orion")),
    arithm: Cow::Borrowed("GKR"),
    is_zk: false,
    is_zkvm: false,
    security_bits: 128,
    is_pq: true,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};
