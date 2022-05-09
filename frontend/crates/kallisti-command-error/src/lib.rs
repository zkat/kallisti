use miette::Diagnostic;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error, Diagnostic, Serialize, Deserialize)]
pub enum KallistiCommandError {
    #[error("{0}")]
    GenericError(String),

    #[error("{message}")]
    UserIdParseError { message: String, id: String },

    #[error("{0}")]
    MatrixClientCreationError(String),

    #[error("{0}")]
    MatrixLoginError(String),
}
