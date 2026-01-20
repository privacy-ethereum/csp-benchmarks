pub mod bench;
pub mod metadata;
pub mod poseidon;

pub use utils::harness::{AuditStatus, BenchProperties};

pub fn expander_bench_properties() -> BenchProperties {
    BenchProperties::new(
        "Libra",
        "M31",
        "GKR",
        Some("Orion"),
        "GKR",
        false,
        false,
        128,
        true,
        true,
        AuditStatus::NotAudited,
        None,
    )
}
