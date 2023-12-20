pub trait Endian {
    fn loadu8(buf: &[u8], from: usize) -> u8;
    fn writeu8<W>(w: &mut W, num: u8) -> io::Result<()>
    where
        W: io::Write;
}

pub struct BigEndian;
pub struct LittleEndian;

impl Endian for BigEndian {
    fn loadu8(buf: &[u8], from: usize) -> u8 {
        let mut num = [0u8; mem::size_of::<u8>()];
        num.copy_from_slice(&buf[from..from + mem::size_of::<u8>()]);
        u8::from_be_bytes(num)
    }
    fn writeu8<W>(w: &mut W, num: u8) -> io::Result<()>
    where
        W: io::Write,
    {
        let buf = num.to_be_bytes();
        w.write_all(&buf)
    }
}

impl Endian for LittleEndian {
    fn loadu8(buf: &[u8], from: usize) -> u8 {
        let mut num = [0u8; mem::size_of::<u8>()];
        num.copy_from_slice(&buf[from..from + mem::size_of::<u8>()]);
        u8::from_le_bytes(num)
    }
    fn writeu8<W>(w: &mut W, num: u8) -> io::Result<()>
    where
        W: io::Write,
    {
        let buf = num.to_le_bytes();
        w.write_all(&buf)
    }
}
