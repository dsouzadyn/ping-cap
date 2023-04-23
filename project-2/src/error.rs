use failure::Fail;
use std::io;

#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO Error.
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    /// Removing non existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;
