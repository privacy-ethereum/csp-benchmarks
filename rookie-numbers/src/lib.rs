use std::borrow::Cow;
use utils::harness::{AuditStatus, BenchProperties};

// Re-export types from sha256 crate
pub use sha256::{FriConfig, PcsConfig, MAX_PREPROCESSED_LOG_SIZE};

/// Benchmark properties for Rookie Numbers prover
pub const ROOKIE_NUMBERS_BENCH_PROPERTIES: BenchProperties = BenchProperties {
    proving_system: Cow::Borrowed("Rookie Numbers"),
    field_curve: Cow::Borrowed("M31"),
    iop: Cow::Borrowed("FRI"),
    pcs: Some(Cow::Borrowed("Circle-PCS")),
    arithm: Cow::Borrowed("AIR"),
    is_zk: false,
    is_zkvm: false,
    security_bits: 96,
    is_pq: true,
    is_maintained: true,
    is_audited: AuditStatus::NotAudited,
    isa: None,
};

pub fn secure_pcs_config() -> PcsConfig {
    PcsConfig {
        pow_bits: 26,
        fri_config: FriConfig {
            log_last_layer_degree_bound: 0,
            log_blowup_factor: 1,
            n_queries: 70,
        },
    }
}
