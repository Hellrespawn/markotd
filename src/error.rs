use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum MarkotdError {
    #[error("Unable to open {0}")]
    NotFound(PathBuf),
    #[error(transparent)]
    Io {
        #[from]
        source: std::io::Error,
    },
}
