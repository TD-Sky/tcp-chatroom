mod error;
pub use self::error::Error;
/// 向socket写入请求；从socket读出响应
mod io;

use protocol::short::{
    request::{self, Method},
    response::Status,
    Request, Response,
};
use serde::Serialize;
use std::{
    io::{BufReader, BufWriter},
    net::{TcpStream, ToSocketAddrs},
};

pub struct Duang {
    socket: TcpStream,
    request: Request,
}

pub struct DuangBuilder {
    address: String,
    method: Method,
    token: Option<String>,
    data: Option<Vec<u8>>,
}

impl Duang {
    pub fn builder(address: impl Into<String>) -> DuangBuilder {
        DuangBuilder {
            address: address.into(),
            method: Method::Login,
            token: None,
            data: None,
        }
    }

    pub fn send(&mut self) -> Result<Response, Error> {
        io::write_request(BufWriter::new(&mut self.socket), &self.request)?;
        let resp = io::read_response(BufReader::new(&mut self.socket))?;

        let status = resp.status();
        match status {
            Status::Ok => Ok(resp),
            _ => Err(Error::Service(status)),
        }
    }

    #[inline]
    pub fn into_socket(self) -> TcpStream {
        self.socket
    }
}

impl<A> TryFrom<(A, Request)> for Duang
where
    A: ToSocketAddrs,
{
    type Error = Error;

    fn try_from((addr, request): (A, Request)) -> Result<Self, Self::Error> {
        let duang = Self {
            socket: TcpStream::connect(addr)?,
            request,
        };
        Ok(duang)
    }
}

impl DuangBuilder {
    #[inline]
    pub fn method(self, method: request::Method) -> Self {
        Self { method, ..self }
    }

    #[inline]
    pub fn token(self, token: impl Into<String>) -> Self {
        Self {
            token: Some(token.into()),
            ..self
        }
    }

    #[inline]
    pub fn data<T>(self, data: T) -> Self
    where
        T: Serialize,
    {
        Self {
            data: Some(rmp_serde::to_vec(&data).unwrap()),
            ..self
        }
    }

    pub fn try_build(self) -> Result<Duang, Error> {
        let Self {
            address,
            method,
            token,
            data,
        } = self;

        let socket = TcpStream::connect(address)?;

        let mut request = Request::from(method);
        if let Some(token) = token {
            request.set_token(token);
        }
        if let Some(data) = data {
            request.set_bytes(data);
        }

        Ok(Duang { socket, request })
    }
}
