#[derive(thiserror::Error, Debug, Clone, PartialEq)]
pub enum Error {
    #[error("invalid argument `{name}`: {reason}")]
    InvalidArgument { name: &'static str, reason: String },

    #[error("invalid operation: {0}")]
    InvalidOperation(String),

    #[error("value out of range: {0}")]
    OutOfRange(String),

    #[error("division by zero")]
    DivideByZero,
}

pub type Result<T> = std::result::Result<T, Error>;
