use std::collections::BTreeMap;

use super::{Class, Field};
use crate::{function::Function, pool_entry::PoolEntry, read_bytes::ReadBytes};

impl Class {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Self> {
    let name = rd.read_rc_str()?;

    let fields_count = rd.read_u8()?;
    let mut fields = BTreeMap::new();
    for offset in 0..fields_count {
      let name = rd.read_rc_str()?;
      fields.insert(name, Field { vis: Field::PUBLIC, offset });
    }

    let pool_count = rd.read_u16()?;
    let constants = (0..pool_count).map(|_| PoolEntry::read(rd)).collect::<Result<_, _>>()?;

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
