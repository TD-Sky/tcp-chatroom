use std::fmt::Display;
use std::str::FromStr;

use super::error::UnknownMethodError;
use super::Body;

#[derive(Debug)]
pub struct Message {
    method: Method,
    body: Body,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Ping,
    Pong,
    Echo,
    Private,
}

impl Message {
    #[inline]
    pub fn new(method: Method, body: Body) -> Self {
        Self { method, body }
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn body(&self) -> &Body {
        &self.body
    }
}

impl FromStr for Method {
    type Err = UnknownMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "ping" => Self::Ping,
            "pong" => Self::Pong,
            "echo" => Self::Echo,
            "private-message" => Self::Private,
            _ => return Err(UnknownMethodError(s.to_owned())),
        })
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Ping => "ping",
            Self::Pong => "pong",
            Self::Echo => "echo",
            Self::Private => "private-message",
        };
        f.write_str(s)
    }
}
