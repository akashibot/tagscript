#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Workload exceeded maximum allowed. The total characters attempted were {0}")]
    WorkloadExceeded(String),

    #[error("Failed to parse the payload")]
    ParseFloatError(#[from] std::num::ParseFloatError),

    #[error("Failed to parse the payload")]
    ParseIntError(#[from] std::num::ParseIntError),
}
