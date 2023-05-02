use super::Response;
use crate::socket::read_response;
use crate::socket::write_request;

use std::borrow::Cow;
use std::io;
use std::io::BufReader;
use std::io::BufWriter;
use std::net::TcpStream;
use std::net::ToSocketAddrs;

#[derive(Debug)]
pub struct Request {
    method: Cow<'static, str>,
    body: Option<Vec<u8>>,
}

impl Request {
    #[inline]
    pub fn method(&self) -> &str {
        self.method.as_ref()
    }

    #[inline]
    pub fn body(&self) -> Option<&[u8]> {
        self.body.as_deref()
    }

    #[inline]
    pub fn from_method(method: impl Into<Cow<'static, str>>) -> Self {
        Self {
            method: method.into(),
            body: None,
        }
    }

    pub fn send(self, addr: impl ToSocketAddrs) -> Result<Response, io::Error> {
        let mut socket = TcpStream::connect(addr)?;

        write_request(BufWriter::new(&mut socket), self).unwrap();
        let resp = read_response(BufReader::new(&mut socket)).unwrap();

        Ok(resp)
    }
}
