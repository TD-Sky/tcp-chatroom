use crate::error::ParseMethodError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
pub struct Horz {
    method: Method,
    data: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Serialize, Deserialize)]
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

impl From<Method> for Horz {
    fn from(method: Method) -> Self {
        Self { method, data: None }
    }
}

impl<T> From<(Method, T)> for Horz
where
    T: Serialize,
{
    fn from((method, data): (Method, T)) -> Self {
        Self {
            method,
            data: Some(rmp_serde::to_vec(&data).unwrap()),
        }
    }
}

impl Horz {
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn bytes(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }

    #[inline]
    pub fn data<T>(&self) -> Option<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.bytes()
            .and_then(|bs| rmp_serde::from_slice(bs).unwrap())
    }

    #[inline]
    pub fn set_method(&mut self, method: Method) {
        self.method = method;
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
