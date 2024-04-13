use std::fmt::Debug;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0:?}")]
    InvalidInput(Box<dyn Debug>),
    #[error("unexpected: {0:?}")]
    Unexpected(Box<dyn Debug>),
}

impl Error {
    pub fn invalid_input<E: Debug + 'static>(err: E) -> Self {
        Self::InvalidInput(Box::new(err))
    }
}
