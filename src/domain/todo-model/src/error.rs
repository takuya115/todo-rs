use thiserror::Error;

pub type Result<T> = std::result::Result<T, ModelError>;

#[derive(Debug, Error)]
pub enum ModelError {
    #[error("validation:{0}")]
    Validation(String),
}
