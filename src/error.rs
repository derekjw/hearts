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
    Parsing { source: &'static str, string: String },
    #[doc(hidden)]
    __Nonexhaustive(Void)
}

#[doc(hidden)]
pub enum Void {}

impl fmt::Debug for Void {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Game(_) => "Server reported an error",
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::Json(ref e) => e.description(),
            Error::Parsing { source: _, string: _ } => "Error while parsing string",
            Error::__Nonexhaustive(ref void) =>  match *void {}
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
    pub fn parsing<A: Into<String>>(source: &'static str, string: A) -> Error {
        Error::Parsing {
            source: source,
            string: string.into(),
        }
    }

    pub fn game<A: Into<String>>(cause: Option<A>) -> Error {
        match cause {
            Some(string) => Error::Game(string.into()),
            None => Error::Game("Unknown server error".to_owned())
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

