use anyhow::Result;
use protocol::{
    parser,
    persistent::{self, Horz},
};
use std::{
    io::{BufRead, BufReader, Read},
    net::TcpStream,
};
use tuirealm::{
    listener::{ListenerError, ListenerResult, Poll},
    Event,
};

pub struct HorzListener {
    reader: BufReader<TcpStream>,
}

impl HorzListener {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            reader: BufReader::new(socket),
        }
    }
}

impl Poll<Horz> for HorzListener {
    fn poll(&mut self) -> ListenerResult<Option<Event<Horz>>> {
        match self.read_horz() {
            Ok(incoming_horz) => Ok(Some(Event::User(incoming_horz))),
            Err(_) => Err(ListenerError::PollFailed),
        }
    }
}

impl HorzListener {
    fn read_horz(&mut self) -> Result<Horz> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let method: persistent::Method = line.parse()?;

        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        let length = parser::length(&line)?;

        let mut horz = Horz::from(method);
        if length != 0 {
            let mut bytes = Vec::with_capacity(length as usize);
            (&mut self.reader).take(length).read_to_end(&mut bytes)?;
            horz.set_bytes(bytes);
        }

        Ok(horz)
    }
}
