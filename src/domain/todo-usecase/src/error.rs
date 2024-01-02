use std::fmt::Debug;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input: {0:?}")]
    InvalidInput(Box<dyn Debug>),
    #[error("unknown: {0:?}")]
    Unknown(Box<dyn Debug>),
}
