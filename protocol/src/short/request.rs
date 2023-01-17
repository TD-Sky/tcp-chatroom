use crate::error::ParseMethodError;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct Request {
    method: Method,
    token: Option<String>,
    data: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Method {
    Login,
    Groups,
    CreateGroup,
    JoinGroup,
    Persistence,
}

impl FromStr for Method {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;
        let method = match s {
            "login" => Login,
            "groups" => Groups,
            "create-group" => CreateGroup,
            "join-group" => JoinGroup,
            _ => return Err(ParseMethodError(s.to_owned())),
        };

        Ok(method)
    }
}

impl Request {
    pub fn new(method: Method, data: Vec<u8>) -> Self {
        Self {
            method,
            token: None,
            data,
        }
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_owned());
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
