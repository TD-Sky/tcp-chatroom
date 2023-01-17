use crate::error::ParseStatusError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Response {
    status: Status,
    data: Option<Vec<u8>>,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Status {
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

impl Response {
    pub fn new(status: Status, data: Vec<u8>) -> Self {
        Self {
            status,
            data: Some(data),
        }
    }

    pub fn status(&self) -> Status {
        self.status
    }

    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data);
    }
}

impl From<Status> for Response {
    fn from(status: Status) -> Self {
        Response { status, data: None }
    }
}
