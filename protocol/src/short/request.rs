use crate::error::ParseMethodError;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

#[derive(Serialize, Deserialize)]
pub struct Request {
    method: Method,
    token: Option<String>,
    data: Option<Vec<u8>>,
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
            "persistence" => Persistence,
            _ => return Err(ParseMethodError(s.to_owned())),
        };

        Ok(method)
    }
}

impl Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Method::*;
        let s = match self {
            Login => "login",
            Groups => "groups",
            CreateGroup => "create-group",
            JoinGroup => "join-group",
            Persistence => "persistence",
        };

        write!(f, "{s}")
    }
}

impl Request {
    pub fn new(method: Method, token: impl Into<String>, data: Vec<u8>) -> Self {
        Self {
            method,
            token: Some(token.into()),
            data: Some(data),
        }
    }

    pub fn login(data: Vec<u8>) -> Self {
        Self {
            method: Method::Login,
            token: None,
            data: Some(data),
        }
    }

    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = Some(data);
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn data(&self) -> Option<&[u8]> {
        self.data.as_deref()
    }
}

impl From<Method> for Request {
    fn from(method: Method) -> Self {
        Self {
            method,
            token: None,
            data: None,
        }
    }
}

impl<T: Into<String>> From<(Method, T)> for Request {
    fn from((method, token): (Method, T)) -> Self {
        Self {
            method,
            token: Some(token.into()),
            data: None,
        }
    }
}
