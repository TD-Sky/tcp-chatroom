use serde::Deserialize;

#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Option<Vec<u8>>,
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug)]
pub enum Status {
    Ok,
}

impl Response {
    pub fn new(status: Status, body: Option<Vec<u8>>) -> Self {
        Self { status, body }
    }

    pub fn body<'a, T: Deserialize<'a>>(&'a self) -> Result<T, Error> {
        let body = self.body.as_deref().ok_or(Error::NoBody)?;
        rmp_serde::from_slice(body).map_err(Error::Decode)
    }
}

impl TryFrom<u8> for Status {
    type Error = UnknownStatusCode;

    fn try_from(code: u8) -> Result<Self, Self::Error> {
        let status = match code {
            0 => Self::Ok,
            _ => return Err(UnknownStatusCode(code)),
        };

        Ok(status)
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
    pub struct UnknownStatusCode(pub(super) u8);

    impl Display for UnknownStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "unknown status code: {:?}", self.0)
        }
    }

    impl std::error::Error for UnknownStatusCode {}
}
