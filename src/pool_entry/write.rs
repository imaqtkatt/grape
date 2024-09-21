use super::PoolEntry;
use crate::write_bytes::WriteBytes;

impl PoolEntry {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    match self {
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
        match self {
          PoolEntry::String(..) => wr.write_u8(PoolEntry::TAG_STRING)?,
          PoolEntry::Module(..) => wr.write_u8(PoolEntry::TAG_MODULE)?,
          PoolEntry::Function(..) => wr.write_u8(PoolEntry::TAG_FUNCTION)?,
          PoolEntry::Class(..) => wr.write_u8(PoolEntry::TAG_CLASS)?,
          _ => unreachable!(),
        }
        wr.write_str(s)?;
      }
      PoolEntry::Field(field_name) => {
        wr.write_u8(PoolEntry::TAG_FIELD)?;
        wr.write_str(field_name)?;
      }
    }
    Ok(())
  }
}
