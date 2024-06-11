use std::collections::BTreeMap;

use super::{Class, Field, Module, PoolEntry};
use crate::{function::Function, read_bytes::ReadBytes};

impl Module {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Module> {
    let magic = rd.read_u32()?;

    if magic != Self::MAGIC {
      return Err(std::io::Error::other("Is not a grape file"));
    }

    let name = rd.read_rc_str()?;

    let pool_count = rd.read_u16()?;
    let mut constants = Vec::with_capacity(pool_count as usize);
    for _ in 0..pool_count {
      let tag = rd.read_u8()?;
      match tag {
        PoolEntry::TAG_STRING => constants.push(PoolEntry::String(rd.read_string()?)),
        PoolEntry::TAG_INTEGER => constants.push(PoolEntry::Integer(rd.read_u32()? as i32)),
        PoolEntry::TAG_MODULE => constants.push(PoolEntry::Module(rd.read_string()?)),
        PoolEntry::TAG_FLOAT => constants.push(PoolEntry::Float(rd.read_f32()?)),
        PoolEntry::TAG_FUNCTION => constants.push(PoolEntry::Function(rd.read_string()?)),
        _ => unreachable!(),
      }
    }

    let functions_count = rd.read_u16()?;
    let mut functions = BTreeMap::new();
    for _ in 0..functions_count {
      let function = Function::read(rd)?;
      let name = function.name.clone();
      functions.insert(name, function);
    }

    let classes_count = rd.read_u16()?;
    let mut classes = BTreeMap::new();
    for _ in 0..classes_count {
      let class = Class::read(rd)?;
      let name = class.name.clone();
      classes.insert(name, class);
    }

    Ok(Self { name, constants, functions, classes })
  }
}

impl Class {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Self> {
    let name = rd.read_rc_str()?;

    let pool_count = rd.read_u16()?;
    let mut constants = Vec::with_capacity(pool_count as usize);
    for _ in 0..pool_count {
      let tag = rd.read_u8()?;
      match tag {
        PoolEntry::TAG_STRING => constants.push(PoolEntry::String(rd.read_string()?)),
        PoolEntry::TAG_INTEGER => constants.push(PoolEntry::Integer(rd.read_u32()? as i32)),
        PoolEntry::TAG_MODULE => constants.push(PoolEntry::Module(rd.read_string()?)),
        PoolEntry::TAG_FLOAT => constants.push(PoolEntry::Float(rd.read_f32()?)),
        PoolEntry::TAG_FUNCTION => constants.push(PoolEntry::Function(rd.read_string()?)),
        _ => unreachable!(),
      }
    }

    let fields_count = rd.read_u8()?;
    let mut fields = BTreeMap::new();
    for offset in 0..fields_count {
      let name = rd.read_rc_str()?;
      fields.insert(name, Field { vis: Field::PUBLIC, offset });
    }

    let methods_count = rd.read_u16()?;
    let mut methods = BTreeMap::new();
    for _ in 0..methods_count {
      let method = Function::read(rd)?;
      let name = method.name.clone();
      methods.insert(name, method);
    }

    Ok(Class { name, constants, fields, methods })
  }
}
