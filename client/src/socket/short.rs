use std::io;
use std::io::Read;
use std::io::Write;

use super::utils::*;
use crate::protocol::short::*;
use crate::protocol::Body;

pub fn write_request(mut writer: impl Write, req: &Request) -> io::Result<()> {
    let method = req.method();
    writer.write_u64(method.len() as u64).unwrap();
    writer.write_all(method.as_bytes()).unwrap();

    let body = req.body().as_bytes();
    writer.write_u16(body.len() as u16).unwrap();
    if !body.is_empty() {
        writer.write_all(body).unwrap();
    }

    writer.flush().unwrap();

    Ok(())
}

pub fn read_response(mut reader: impl Read) -> io::Result<Response> {
    let status = reader.read_u8().unwrap();
    let status = Status::try_from(status).unwrap();

    let blen = reader.read_u64().unwrap() as usize;
    let body = if blen != 0 {
        let mut body = vec![0; blen];
        reader.read_exact(body.as_mut_slice()).unwrap();
        Body::from(body)
    } else {
        Body::empty()
    };

    let resp = Response::new(status, body);

    Ok(resp)
}
