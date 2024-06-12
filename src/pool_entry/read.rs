use crate::read_bytes;

use super::PoolEntry;
use read_bytes::ReadBytes;

impl PoolEntry {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Self> {
    let tag = rd.read_u8()?;
    let result = match tag {
      PoolEntry::TAG_STRING => PoolEntry::String(rd.read_string()?),
      PoolEntry::TAG_INTEGER => PoolEntry::Integer(rd.read_u32()? as i32),
      PoolEntry::TAG_MODULE => PoolEntry::Module(rd.read_string()?),
      PoolEntry::TAG_FLOAT => PoolEntry::Float(rd.read_f32()?),
      PoolEntry::TAG_FUNCTION => PoolEntry::Function(rd.read_string()?),
      _ => unreachable!(),
    };
    Ok(result)
  }
}
