use std::error;
use std::fmt;
use std::io;

/// Error codes that the library might return.
#[derive(Debug)]
pub enum Error {
    /// An `io::Error`.
    Io(io::Error),

    /// A `c_ares::Error`.
    Ares(c_ares::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => err.fmt(f),
            Error::Ares(ref err) => err.fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Ares(ref err) => Some(err),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<c_ares::Error> for Error {
    fn from(err: c_ares::Error) -> Error {
        Error::Ares(err)
    }
}
