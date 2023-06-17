use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;

#[inline]
pub fn split(socket: TcpStream) -> (ReadHalf, WriteHalf) {
    let socket = Arc::new(socket);
    (ReadHalf(socket.clone()), WriteHalf(socket))
}

pub struct ReadHalf(Arc<TcpStream>);

pub struct WriteHalf(Arc<TcpStream>);

impl Read for ReadHalf {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.as_ref().read(buf)
    }
}

impl Write for WriteHalf {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.as_ref().write(buf)
    }

    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        self.0.as_ref().flush()
    }
}
