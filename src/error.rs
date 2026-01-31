#[derive(Debug, thiserror::Error)]
pub enum NseError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("unexpected API response")]
    InvalidResponse,
}
