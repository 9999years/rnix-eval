use thiserror::Error;

use crate::pos::OwnedPos;

#[derive(Debug, Error)]
pub enum NixError {
    #[error("undefined variable '{0}' at '{1}'")]
    UndefinedVar(String, OwnedPos),
}

pub type NixResult<T> = Result<T, NixError>;
