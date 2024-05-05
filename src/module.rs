pub mod builder;
pub mod read;
pub mod std_out;
pub mod write;

use crate::function::Function;
use crate::runtime::{Error, Result};
use std::rc::Rc;

/// Bytecode Module representation.
///
/// ```
/// {
///   magic_number: u32,
///   module_name_length: u16,
///   module_name: str<module_name_length>,
///   pool_count: u16,
///   constants: Vec<PoolEntry, pool_count>,
///   functions_count: u16,
///   functions: Vec<Function, functions_count>,
/// }
/// ```
#[derive(Debug)]
pub struct Module {
  /// The module name.
  pub name: Box<str>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The module functions.
  pub functions: Vec<Rc<Function>>,
}

#[derive(Clone, Debug)]
pub enum PoolEntry {
  String(String),
  Integer(i32),
  Module(String),
}

impl PoolEntry {
  pub const TAG_STRING: u8 = 0x1;
  pub const TAG_INTEGER: u8 = 0x2;
  pub const TAG_MODULE: u8 = 0x3;
}

impl Module {
  pub const MAGIC: u32 = 0x75_76_61_73;

  pub fn fetch_function_with_name(&self, name: &str) -> Result<Rc<Function>> {
    self
      .functions
      .iter()
      .find(|f| f.name.as_ref() == name)
      .ok_or(Error::FunctionNotFound(name.to_string()))
      .cloned()
  }

  pub fn fetch_function_with_identifier(&self, identifier: usize) -> Rc<Function> {
    unsafe { self.functions.get_unchecked(identifier).clone() }
  }
}
