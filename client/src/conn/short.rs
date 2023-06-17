use std::borrow::Cow;
use std::io;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;
use std::net::ToSocketAddrs;

use serde::Serialize;

use crate::protocol::short::*;
use crate::protocol::Body;
use crate::socket::short::*;

pub struct ShortConn {
    request: Request,
    socket: TcpStream,
}

pub struct ShortConnBuilder {
    method: Cow<'static, str>,
    data: Option<Vec<u8>>,
}

impl ShortConn {
    #[inline]
    pub fn builder(method: impl Into<Cow<'static, str>>) -> ShortConnBuilder {
        ShortConnBuilder {
            method: method.into(),
            data: None,
        }
    }

    pub fn send(&mut self) -> Result<Response, io::Error> {
        write_request(BufWriter::new(&mut self.socket), &self.request).unwrap();
        let resp = read_response(BufReader::new(&mut self.socket)).unwrap();

        Ok(resp)
    }

    #[inline]
    pub fn into_socket(self) -> TcpStream {
        self.socket
    }
}

impl ShortConnBuilder {
    #[inline]
    pub fn data<T: Serialize>(self, data: T) -> Self {
        Self {
            data: Some(rmp_serde::to_vec(&data).unwrap()),
            ..self
        }
    }

    pub fn try_build(self, addr: impl ToSocketAddrs) -> Result<ShortConn, io::Error> {
        let socket = TcpStream::connect(addr)?;

        let body = self.data.map(Body::from).unwrap_or_else(Body::empty);
        let request = Request::new(self.method, body);

        Ok(ShortConn { request, socket })
    }
}
