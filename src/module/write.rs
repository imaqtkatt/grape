use crate::write_bytes::WriteBytes;

use super::{Module, PoolEntry};

impl Module {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_u32(Self::MAGIC)?;
    wr.write_str(&self.name)?;

    wr.write_u16(self.constants.len() as u16)?;
    for element in self.constants.iter() {
      match element {
        PoolEntry::String(s) => {
          wr.write_u8(PoolEntry::TAG_STRING)?;
          wr.write_str(s)?;
        }
        PoolEntry::Integer(i) => {
          wr.write_u8(PoolEntry::TAG_INTEGER)?;
          wr.write_all(&i.to_be_bytes())?;
        }
        PoolEntry::Module(m) => {
          wr.write_u8(PoolEntry::TAG_MODULE)?;
          wr.write_str(m)?;
        }
        PoolEntry::Float(f) => {
          wr.write_u8(PoolEntry::TAG_FLOAT)?;
          wr.write_all(&f.to_be_bytes())?;
        }
      }
    }

    wr.write_u16(self.functions.len() as u16)?;

    for function in self.functions.iter() {
      function.write(wr)?;
    }

    Ok(())
  }
}
