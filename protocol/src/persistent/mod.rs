use crate::error::ParseMethodError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Horz {
    method: Method,
    data: Option<Vec<u8>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Method {
    Ping,
    Pong,
    Close,
    PublicMessage,
    PrivateMessage,
    GroupMessage,
    Echo,
}

impl FromStr for Method {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;
        let method = match s {
            "ping" => Ping,
            "pong" => Pong,
            "close" => Close,
            "public-message" => PublicMessage,
            "private-message" => PrivateMessage,
            "group-message" => GroupMessage,
            "echo" => Echo,
            _ => return Err(ParseMethodError(s.to_owned())),
        };

        Ok(method)
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Method::*;
        let s = match self {
            Ping => "ping",
            Pong => "pong",
            Close => "close",
            PublicMessage => "public-message",
            PrivateMessage => "private-message",
            GroupMessage => "group-message",
            Echo => "echo",
        };

        write!(f, "{s}")
    }
}

impl Horz {
    pub fn new(method: Method, data: Vec<u8>) -> Self {
        Self {
            method,
            data: Some(data),
        }
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }

    pub fn set_method(&mut self, method: Method) {
        self.method = method;
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data);
    }
}

impl From<Method> for Horz {
    fn from(method: Method) -> Self {
        Self { method, data: None }
    }
}
