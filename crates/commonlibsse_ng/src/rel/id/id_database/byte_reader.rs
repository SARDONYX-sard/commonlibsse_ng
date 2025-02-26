use std::io::{self, Read};

pub fn read_u8<R: Read>(reader: &mut R) -> io::Result<u8> {
    let mut buf = [0; 1];
    reader.read_exact(&mut buf)?;
    Ok(buf[0])
}

pub fn read_le_u16<R: Read>(reader: &mut R) -> io::Result<u16> {
    let mut buf = [0; 2];
    reader.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

pub fn read_le_u32<R: Read>(reader: &mut R) -> io::Result<u32> {
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

pub fn read_le_u64<R: Read>(reader: &mut R) -> io::Result<u64> {
    let mut buf = [0; 8];
    reader.read_exact(&mut buf)?;
    Ok(u64::from_le_bytes(buf))
}
