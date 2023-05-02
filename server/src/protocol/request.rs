use serde::Deserialize;
use std::str::FromStr;

#[derive(Debug)]
pub struct Request {
    method: Method,
    body: Option<Vec<u8>>,
}

#[derive(Debug)]
pub enum Method {
    Ping,
}

impl Request {
    #[inline]
    pub fn new(method: Method, body: Option<Vec<u8>>) -> Self {
        Self { method, body }
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn body<'a, T: Deserialize<'a>>(&'a self) -> Result<T, Error> {
        let body = self.body.as_deref().ok_or(Error::NoBody)?;
        rmp_serde::from_slice(body).map_err(Error::Decode)
    }
}

impl FromStr for Method {
    type Err = UnknownProtocolError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let method = match s {
            "ping" => Self::Ping,
            _ => return Err(UnknownProtocolError(s.to_owned())),
        };
        Ok(method)
    }
}

pub use error::*;
mod error {
    use std::fmt;
    use std::fmt::Display;

    #[derive(Debug)]
    pub enum Error {
        NoBody,
        Decode(rmp_serde::decode::Error),
    }

    impl std::error::Error for Error {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            use Error::*;
            match self {
                NoBody => None,
                Decode(e) => Some(e),
            }
        }
    }

    impl Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Error::*;
            match self {
                NoBody => write!(f, "request has no body"),
                Decode(e) => write!(f, "{e}"),
            }
        }
    }

    impl From<rmp_serde::decode::Error> for Error {
        fn from(e: rmp_serde::decode::Error) -> Self {
            Self::Decode(e)
        }
    }

    #[derive(Debug)]
    pub struct UnknownProtocolError(pub(super) String);

    impl std::error::Error for UnknownProtocolError {}

    impl Display for UnknownProtocolError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "unknown protocol: {:?}", self.0)
        }
    }
}
