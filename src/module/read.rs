use super::{Module, PoolEntry};
use crate::{function::Function, read_bytes::ReadBytes};

impl Module {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Module> {
    let magic = rd.read_u32()?;

    if magic != Self::MAGIC {
      return Err(std::io::Error::other("Is not a grape file"));
    }

    let name = rd.read_box_str()?;

    let pool_count = rd.read_u16()?;
    let mut constants = Vec::with_capacity(pool_count as usize);
    for _ in 0..pool_count {
      let tag = rd.read_u8()?;
      match tag {
        PoolEntry::TAG_STRING => constants.push(PoolEntry::String(rd.read_string()?)),
        PoolEntry::TAG_INTEGER => constants.push(PoolEntry::Integer(rd.read_u32()? as i32)),
        PoolEntry::TAG_MODULE => constants.push(PoolEntry::Module(rd.read_string()?)),
        _ => unreachable!(),
      }
    }

    let functions_count = rd.read_u16()?;
    let mut functions = Vec::with_capacity(functions_count as usize);
    for id in 0..functions_count {
      let mut function = Function::read(rd)?;
      function.identifier = id as usize;
      functions.push(function);
    }

    Ok(Self { name, constants, functions })
  }
}
