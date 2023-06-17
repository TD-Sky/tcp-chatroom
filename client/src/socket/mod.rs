pub mod persistent;
pub mod short;

mod utils {
    use std::io;
    use std::io::Read;
    use std::io::Write;

    pub trait ReadExt: Read {
        fn read_u8(&mut self) -> io::Result<u8> {
            let mut buf = [0];
            self.read_exact(&mut buf)?;
            Ok(buf[0])
        }

        fn read_u16(&mut self) -> io::Result<u16> {
            let mut buf = [0; 2];
            self.read_exact(&mut buf)?;
            Ok(u16::from_be_bytes(buf))
        }

        fn read_u64(&mut self) -> io::Result<u64> {
            let mut buf = [0; 8];
            self.read_exact(&mut buf)?;
            Ok(u64::from_be_bytes(buf))
        }
    }

    pub trait WriteExt: Write {
        #[inline]
        fn write_u16(&mut self, n: u16) -> io::Result<()> {
            self.write_all(&n.to_be_bytes())
        }

        #[inline]
        fn write_u64(&mut self, n: u64) -> io::Result<()> {
            self.write_all(&n.to_be_bytes())
        }
    }

    impl<R: Read> ReadExt for R {}

    impl<W: Write> WriteExt for W {}
}
