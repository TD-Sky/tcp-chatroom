use protocol::{error::ParseStatusError, short::response::Status};
use std::{io, num::ParseIntError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("response status: {0}")]
    Service(Status),
    #[error("failed when parsing metadata")]
    ParseMetadata,
    #[error("socket reading/writing error: {0}")]
    Io(io::ErrorKind),
}

impl From<ParseIntError> for Error {
    fn from(_: ParseIntError) -> Self {
        Self::ParseMetadata
    }
}

impl From<ParseStatusError> for Error {
    fn from(_: ParseStatusError) -> Self {
        Self::ParseMetadata
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self::Io(e.kind())
    }
}
