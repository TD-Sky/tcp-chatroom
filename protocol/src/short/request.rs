use crate::error::ParseMethodError;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
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
    MyGroups,
    CreateGroup,
    JoinGroup,
    Persistence,
    IdNameMap,
}

impl FromStr for Method {
    type Err = ParseMethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;
        let method = match s {
            "login" => Login,
            "groups" => Groups,
            "my-groups" => MyGroups,
            "create-group" => CreateGroup,
            "join-group" => JoinGroup,
            "persistence" => Persistence,
            "id-name-map" => IdNameMap,
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
            MyGroups => "my-groups",
            CreateGroup => "create-group",
            JoinGroup => "join-group",
            Persistence => "persistence",
            IdNameMap => "id-name-map",
        };

        write!(f, "{s}")
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

impl<T> From<(Method, T)> for Request
where
    T: Into<String>,
{
    fn from((method, token): (Method, T)) -> Self {
        Self {
            method,
            token: Some(token.into()),
            data: None,
        }
    }
}

impl Request {
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    #[inline]
    pub fn bytes(&self) -> Option<&[u8]> {
        self.data.as_deref()
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
    pub fn set_token(&mut self, token: impl Into<String>) {
        self.token = Some(token.into());
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
