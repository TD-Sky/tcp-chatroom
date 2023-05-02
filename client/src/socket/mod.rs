use std::io;
use std::io::Read;
use std::io::Write;

use crate::protocol::response::Status;
use crate::protocol::{Request, Response};

pub fn write_request(mut writer: impl Write, req: Request) -> io::Result<()> {
    let method = req.method();
    write_usize_le(&mut writer, method.len()).unwrap();
    writer.write_all(method.as_bytes()).unwrap();

    if let Some(body) = req.body() {
        write_usize_le(&mut writer, body.len()).unwrap();
        writer.write_all(body).unwrap();
    } else {
        write_usize_le(&mut writer, 0).unwrap();
    }

    Ok(())
}

pub fn read_response(mut reader: impl Read) -> io::Result<Response> {
    let status = read_u8(&mut reader).unwrap();
    let status = Status::try_from(status).unwrap();

    let blen = read_usize_le(&mut reader).unwrap();
    let body = if blen != 0 {
        let mut body = vec![0; blen];
        reader.read_exact(body.as_mut_slice()).unwrap();
        Some(body)
    } else {
        None
    };

    let resp = Response::new(status, body);

    Ok(resp)
}

fn read_u8<T: Read>(reader: &mut T) -> io::Result<u8> {
    let mut buf = [0];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_usize_le<T: Read>(reader: &mut T) -> io::Result<usize> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf) as usize)
}

fn write_usize_le<T: Write>(writer: &mut T, n: usize) -> io::Result<()> {
    writer.write_all(&(n as u64).to_le_bytes())
}
