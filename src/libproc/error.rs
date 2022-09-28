use errno::Errno;
use thiserror::Error;
use std::convert::From;
use std::string::FromUtf8Error;

/// libproc error type
#[derive(Debug, Error, Eq, PartialEq)]
pub enum LibProcError {
    /// OS error
    #[error("OS Error `{0:?}`")]
    OSError(Errno),
    /// unsupported proctype specified
    #[error("Unsupported ProcType specified")]
    UnsupportedProcType,
    /// can't read procfs
    #[error("Can't read '/proc': `{0}`")]
    CantReadProcFS(String),
    /// generic error
    #[error("`{0}`")]
    GenericError(String),
    /// not implemented features
    #[error("`{0}`")]
    NotImplemented(&'static str),
    /// UTF-8 error
    #[error("Invalid UTF-8 sequence: `{0}`")]
    InvalidUTF8(FromUtf8Error),
    #[error("Could not find the field named '`{0}`' in the /proc FS file name '`{1}`'")]
    /// Could not find procfs field
    CouldNotFoundProc(String, String),
}

/// LibProcError from Errno
impl From<Errno> for LibProcError {
    fn from(errno: Errno) -> Self {
        LibProcError::OSError(errno)
    }
}

/// convert FromUtf8Error to inner error type
impl From<FromUtf8Error> for LibProcError {
    fn from(e: FromUtf8Error) -> Self {
        LibProcError::InvalidUTF8(e)
    }
}
