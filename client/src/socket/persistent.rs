use std::io;
use std::io::Read;
use std::io::Write;

use cursive::reexports::log::info;

use super::utils::*;
use crate::protocol::persistent::*;
use crate::protocol::Body;

pub fn read_message(mut reader: impl Read) -> io::Result<Message> {
    let mlen = reader.read_u64()? as usize;
    let mut method = vec![0; mlen];
    reader.read_exact(&mut method)?;
    let method: Method = String::from_utf8(method).unwrap().parse().unwrap();
    info!("read message method");

    let blen = reader.read_u16()?;
    let body = if blen != 0 {
        let mut body = vec![0; blen as usize];
        reader.read_exact(&mut body)?;
        Body::from(body)
    } else {
        Body::empty()
    };
    info!("read message body");

    Ok(Message::new(method, body))
}

pub fn write_message(mut writer: impl Write, msg: Message) -> io::Result<()> {
    let method = msg.method().to_string();
    writer.write_u64(method.len() as u64).unwrap();
    writer.write_all(method.as_bytes()).unwrap();

    let body = msg.body().as_bytes();
    writer.write_u16(body.len() as u16).unwrap();
    if !body.is_empty() {
        writer.write_all(body).unwrap();
    }

    writer.flush()
}
