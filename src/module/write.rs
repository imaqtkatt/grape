use super::Module;
use crate::write_bytes::WriteBytes;

impl Module {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_u32(Self::MAGIC)?;
    wr.write_str(&self.name)?;

    wr.write_u16(self.constants.len() as u16)?;
    for constant in self.constants.iter() {
      constant.write(wr)?;
    }

    wr.write_u16(self.functions.len() as u16)?;
    for function in self.functions.values() {
      function.write(wr)?;
    }

    wr.write_u16(self.classes.len() as u16)?;
    for class in self.classes.values() {
      class.write(wr)?;
    }

    Ok(())
  }
}
