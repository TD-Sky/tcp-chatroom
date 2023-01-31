#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("parsing method {0:?} failed")]
pub struct ParseMethodError(pub(crate) String);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("parsing status {0:?} failed")]
pub struct ParseStatusError(pub(crate) String);

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("parsing token failed")]
pub struct ParseJwtTokenError;
