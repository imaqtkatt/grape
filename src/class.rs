pub mod builder;
pub mod read;
pub mod write;

use std::{collections::BTreeMap, rc::Rc};

use crate::{function::Function, pool_entry::PoolEntry};

pub use builder::ClassBuilder;

#[derive(Debug)]
pub struct Field {
  pub vis: u8,
  pub offset: u8,
}

impl Field {
  pub const PRIVATE: u8 = 0x0;
  pub const PUBLIC: u8 = 0x1;
}

#[derive(Debug)]
pub struct Class {
  /// The class name.
  pub name: Rc<str>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The class fields.
  pub fields: BTreeMap<Rc<str>, Field>,
  /// The class methods.
  pub methods: BTreeMap<Rc<str>, Function>,
}

impl Class {
  pub const TAG: u8 = 0x2;

  pub fn fetch_function_with_name_unchecked(&self, function_name: &str) -> &Function {
    &self.methods[function_name]
  }
}
