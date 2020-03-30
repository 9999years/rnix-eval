use thiserror::Error;

use crate::pos::OwnedPos;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum NixError {
    #[error("undefined variable '{0}' at '{1}'")]
    UndefinedVar(String, OwnedPos),
    #[error("variable '{0}' was not evaluated")]
    VarLookupUnevaluated(String),
    #[error("type error: {0}")]
    Type(String),
}

pub type NixResult<T> = Result<T, NixError>;
