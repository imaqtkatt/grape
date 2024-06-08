pub mod builder;
pub mod file;
pub mod read;
pub mod std_out;
pub mod tcp;
pub mod write;

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::function::Function;
use crate::runtime::{Error, Result};

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
  pub id: u16,
  /// The module name.
  pub name: Rc<str>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The module functions.
  pub functions_map: BTreeMap<Rc<str>, u16>,
  pub functions: Vec<Function>,
}

#[derive(Clone, Debug)]
pub enum PoolEntry {
  String(String),
  Integer(i32),
  Module(String),
  Float(f32),
  Function(String),
}

impl PoolEntry {
  pub const TAG_STRING: u8 = 0x1;
  pub const TAG_INTEGER: u8 = 0x2;
  pub const TAG_MODULE: u8 = 0x3;
  pub const TAG_FLOAT: u8 = 0x4;
  pub const TAG_FUNCTION: u8 = 0x5;
}

impl Module {
  pub const MAGIC: u32 = 0x75_76_61_73;

  pub fn fetch_function_with_name(&self, name: &str) -> Result<&Function> {
    let idx = self.functions_map.get(name).ok_or(Error::FunctionNotFound(name.to_string()))?;
    Ok(&self.functions[*idx as usize])
  }

  pub fn fetch_function_with_name_unchecked(&self, name: &str) -> &Function {
    let idx = self.functions_map[name];
    &self.functions[idx as usize]
  }
}
