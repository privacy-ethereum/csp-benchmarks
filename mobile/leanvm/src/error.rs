#[derive(Debug, thiserror::Error)]
#[cfg_attr(feature = "uniffi", derive(uniffi::Error))]
pub enum MoproError {
    #[error("BenchmarkError: {0}")]
    BenchmarkError(String),
}
