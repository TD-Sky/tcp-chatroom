#[derive(Debug, PartialEq, Eq)]
pub struct ParseMethodError(pub(crate) String);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStatusError(pub(crate) String);

#[derive(Debug, PartialEq, Eq)]
pub struct ParseJwtTokenError;
