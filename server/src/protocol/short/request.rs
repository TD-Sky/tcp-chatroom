use std::str::FromStr;

use crate::protocol::error::UnknownMethodError;
use crate::protocol::Backdoor;
use crate::protocol::Body;

#[derive(Debug)]
pub struct Request {
    method: Method,
    body: Body,
    pub backdoor: Option<Backdoor>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Ping,
    Persistent,
}

impl Request {
    #[inline]
    pub fn new(method: Method, body: Body) -> Self {
        Self {
            method,
            body,
            backdoor: (method == Method::Persistent).then(Backdoor::new),
        }
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
        let method = match s {
            "ping" => Self::Ping,
            "persistent" => Self::Persistent,
            _ => return Err(UnknownMethodError(s.to_owned())),
        };
        Ok(method)
    }
}
