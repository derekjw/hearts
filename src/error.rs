use std::error::Error as StdError;
use std::fmt;

use std::io::Error as IoError;
use serde_json::error::Error as SerdeJsonError;
use hyper::error::Error as HyperError;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Game(String),
    Http(HyperError),
    Io(IoError),
    Json(SerdeJsonError),
    Parsing(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Game(ref s) => s,
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Parsing(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match *self {
            Error::Http(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Json(ref e) => Some(e),
            _ => None,
        }
    }
}

impl Error {
    pub fn parsing(source: &str, string: &str) -> Error {
        Error::Parsing(format!("Error while parsing \"{}\" as {}", string, source))
    }

    pub fn game<A: Into<String>>(cause: Option<A>) -> Error {
        match cause {
            Some(string) => Error::Game(string.into()),
            None => Error::Game("Unknown server error".to_owned()),
        }
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<HyperError> for Error {
    fn from(err: HyperError) -> Error {
        Error::Http(err)
    }
}

impl From<SerdeJsonError> for Error {
    fn from(err: SerdeJsonError) -> Error {
        Error::Json(err)
    }
}
