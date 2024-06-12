use std::collections::BTreeMap;

use super::{Class, Module};
use crate::PoolEntry;
use crate::{function::Function, read_bytes::ReadBytes};

impl Module {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Module> {
    let magic = rd.read_u32()?;

    if magic != Self::MAGIC {
      return Err(std::io::Error::other("Is not a grape file"));
    }

    let name = rd.read_rc_str()?;

    let pool_count = rd.read_u16()?;
    let constants = (0..pool_count).map(|_| PoolEntry::read(rd)).collect::<Result<_, _>>()?;

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
