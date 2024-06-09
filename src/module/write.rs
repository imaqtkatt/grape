use crate::write_bytes::WriteBytes;

use super::{Module, PoolEntry};

impl Module {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_u32(Self::MAGIC)?;
    wr.write_str(&self.name)?;

    wr.write_u16(self.constants.len() as u16)?;
    for element in self.constants.iter() {
      match element {
        PoolEntry::Integer(i) => {
          wr.write_u8(PoolEntry::TAG_INTEGER)?;
          wr.write_all(&i.to_be_bytes())?;
        }
        PoolEntry::Float(f) => {
          wr.write_u8(PoolEntry::TAG_FLOAT)?;
          wr.write_all(&f.to_be_bytes())?;
        }
        PoolEntry::String(s)
        | PoolEntry::Module(s)
        | PoolEntry::Function(s)
        | PoolEntry::Class(s) => {
          match element {
            PoolEntry::String(..) => wr.write_u8(PoolEntry::TAG_STRING)?,
            PoolEntry::Module(..) => wr.write_u8(PoolEntry::TAG_MODULE)?,
            PoolEntry::Function(..) => wr.write_u8(PoolEntry::TAG_FUNCTION)?,
            PoolEntry::Class(..) => todo!(),
            _ => unreachable!(),
          }
          wr.write_str(s)?;
        }
        PoolEntry::Field(_field_name, _class_index) => todo!(),
      }
    }

    wr.write_u16(self.functions.len() as u16)?;

    for function in self.functions.values() {
      function.write(wr)?;
    }

    Ok(())
  }
}
