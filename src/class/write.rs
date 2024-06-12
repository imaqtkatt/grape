use super::Class;
use crate::write_bytes::WriteBytes;

impl Class {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_str(&self.name)?;

    wr.write_u8(self.fields.len() as u8)?;
    for field in self.fields.iter() {
      wr.write_str(field.0)?;
    }

    wr.write_u16(self.constants.len() as u16)?;
    for constant in self.constants.iter() {
      constant.write(wr)?;
    }

    wr.write_u16(self.methods.len() as u16)?;
    for function in self.methods.values() {
      function.write(wr)?;
    }

    Ok(())
  }
}
