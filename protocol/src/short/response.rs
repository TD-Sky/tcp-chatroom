use crate::error::ParseStatusError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: Status,
    data: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize)]
pub enum Status {
    #[default]
    Ok,
    Unauthorized,
    UserNotFound,
    WrongPassword,
    GroupExisted,
}

impl FromStr for Status {
    type Err = ParseStatusError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let status = match s {
            "ok" => Status::Ok,
            "unauthorized" => Status::Unauthorized,
            "user-not-found" => Status::UserNotFound,
            "wrong-password" => Status::WrongPassword,
            "group-existed" => Status::GroupExisted,
            _ => return Err(ParseStatusError(s.to_owned())),
        };

        Ok(status)
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Status::Ok => "ok",
            Status::Unauthorized => "unauthorized",
            Status::UserNotFound => "user-not-found",
            Status::WrongPassword => "wrong-password",
            Status::GroupExisted => "group-existed",
        };

        write!(f, "{s}")
    }
}

impl From<Status> for Response {
    fn from(status: Status) -> Self {
        Response { status, data: None }
    }
}

impl Response {
    pub fn new<T>(data: T) -> Self
    where
        T: Serialize,
    {
        Self {
            status: Status::Ok,
            data: Some(rmp_serde::to_vec(&data).unwrap()),
        }
    }

    #[inline]
    pub fn status(&self) -> Status {
        self.status
    }

    #[inline]
    pub fn data<T>(&self) -> Option<T>
    where
        T: DeserializeOwned,
    {
        self.bytes()
            .and_then(|bs| rmp_serde::from_slice(bs).unwrap())
    }

    #[inline]
    pub fn bytes(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }

    #[inline]
    pub fn set_bytes(&mut self, bytes: Vec<u8>) {
        self.data = Some(bytes);
    }

    #[inline]
    pub fn set_data(&mut self, data: impl Serialize) {
        self.data = Some(rmp_serde::to_vec(&data).unwrap());
    }
}
