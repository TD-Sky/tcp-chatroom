use crate::protocol::Body;

#[derive(Debug)]
pub struct Response {
    status: Status,
    body: Body,
}

#[non_exhaustive]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    Ok,
}

impl Response {
    pub fn new(status: Status, body: Body) -> Self {
        Self { status, body }
    }

    #[inline]
    pub fn status(&self) -> &Status {
        &self.status
    }

    #[inline]
    pub fn body(&self) -> &Body {
        &self.body
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

    #[derive(Debug)]
    pub struct UnknownStatusCode(pub(super) u8);

    impl fmt::Display for UnknownStatusCode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "unknown status code: {:?}", self.0)
        }
    }

    impl std::error::Error for UnknownStatusCode {}
}
