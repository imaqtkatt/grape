use std::io::Result;

pub trait WriteBytes {
  fn write_u8(&mut self, value: u8) -> Result<()>;

  fn write_u16(&mut self, value: u16) -> Result<()>;

  fn write_u32(&mut self, value: u32) -> Result<()>;

  fn write_str(&mut self, s: &str) -> Result<()>;
}

impl<W> WriteBytes for W
where
  W: std::io::Write,
{
  fn write_u8(&mut self, value: u8) -> Result<()> {
    self.write_all(&[value])
  }

  fn write_u16(&mut self, value: u16) -> Result<()> {
    self.write_all(&value.to_be_bytes())
  }

  fn write_u32(&mut self, value: u32) -> Result<()> {
    self.write_all(&value.to_be_bytes())
  }

  fn write_str(&mut self, s: &str) -> Result<()> {
    self.write_all(&(s.len() as u16).to_be_bytes())?;
    self.write_all(s.as_bytes())
  }
}
