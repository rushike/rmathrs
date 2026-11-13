//! Error types for rmathrs

#[derive(Debug, thiserror::Error)]
pub enum RMathError {
    #[error("Invalid number format: {0}")]
    InvalidNumberFormat(String),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Not implemented: {0}")]
    NotImplemented(String),

    #[error("Number cast error: {0}")]
    NumberCast(String),
}

