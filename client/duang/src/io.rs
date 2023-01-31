use crate::Error;
use protocol::{
    parser,
    short::{response, Request, Response},
};
use std::io::{self, BufRead, Read, Write};

pub(super) fn write_request(mut writer: impl Write, req: &Request) -> io::Result<()> {
    writer.write_all(req.method().to_string().as_bytes())?;
    writer.write_all(b"\n")?;

    if let Some(token) = req.token() {
        writer.write_all(format!("Token = {token}\n").as_bytes())?;
    }

    if let Some(data) = req.bytes() {
        writer.write_all(format!("Length = {}\n", data.len()).as_bytes())?;
        writer.write_all(data)?;
    } else {
        writer.write_all(b"Length = 0\n")?;
    }

    Ok(())
}

pub(super) fn read_response(mut reader: impl BufRead) -> Result<Response, Error> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let status: response::Status = line.parse()?;

    let mut line = String::new();
    reader.read_line(&mut line)?;
    let length = parser::length(&line)?;

    let mut resp = Response::from(status);
    if length != 0 {
        let mut bytes = Vec::with_capacity(length as usize);
        (&mut reader).take(length).read_to_end(&mut bytes)?;
        resp.set_bytes(bytes);
    }

    Ok(resp)
}
