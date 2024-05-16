use std::{io::Result, rc::Rc};

pub trait ReadBytes {
  fn read_u8(&mut self) -> Result<u8>;

  fn read_u16(&mut self) -> Result<u16>;

  fn read_u32(&mut self) -> Result<u32>;

  fn read_f32(&mut self) -> Result<f32>;

  fn read_box_str(&mut self) -> Result<Box<str>>;

  fn read_rc_str(&mut self) -> Result<Rc<str>>;

  fn read_string(&mut self) -> Result<String>;
}

impl<R> ReadBytes for R
where
  R: std::io::Read,
{
  fn read_u8(&mut self) -> Result<u8> {
    let mut buf = [0; 1];
    self.read_exact(&mut buf)?;
    Ok(u8::from_be_bytes(buf))
  }

  fn read_u16(&mut self) -> Result<u16> {
    let mut buf = [0; 2];
    self.read_exact(&mut buf)?;
    Ok(u16::from_be_bytes(buf))
  }

  fn read_u32(&mut self) -> Result<u32> {
    let mut buf = [0; 4];
    self.read_exact(&mut buf)?;
    Ok(u32::from_be_bytes(buf))
  }

  fn read_f32(&mut self) -> Result<f32> {
    let mut buf = [0; 4];
    self.read_exact(&mut buf)?;
    Ok(f32::from_be_bytes(buf))
  }

  fn read_box_str(&mut self) -> Result<Box<str>> {
    let length = self.read_u16()?;

    let mut str_buf = vec![0; length as usize];
    self.read_exact(&mut str_buf)?;

    let str = std::str::from_utf8(&str_buf).map_err(std::io::Error::other)?;

    Ok(Box::from(str))
  }

  fn read_rc_str(&mut self) -> Result<Rc<str>> {
    let length = self.read_u16()?;

    let mut str_buf = vec![0; length as usize];
    self.read_exact(&mut str_buf)?;

    let str = std::str::from_utf8(&str_buf).map_err(std::io::Error::other)?;

    Ok(Rc::from(str))
  }

  fn read_string(&mut self) -> Result<String> {
    let length = self.read_u16()?;

    let mut str_buf = vec![0; length as usize];
    self.read_exact(&mut str_buf)?;

    String::from_utf8(str_buf).map_err(std::io::Error::other)
  }
}
