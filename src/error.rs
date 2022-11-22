use std::io::ErrorKind;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MarkotdError {
    #[error("Can't find command '{0}'")]
    NotFound(&'static str),

    #[error(transparent)]
    FromUtf8 {
        #[from]
        source: FromUtf8Error,
    },

    #[error("String is not valid UTF-8")]
    Other {
        #[from]
        source: std::io::Error,
    },
}

impl MarkotdError {
    pub(crate) fn map_command_error(
        command: &'static str,
        error: std::io::Error,
    ) -> MarkotdError {
        if error.kind() == ErrorKind::NotFound {
            MarkotdError::NotFound(command)
        } else {
            MarkotdError::from(error)
        }
    }
}
