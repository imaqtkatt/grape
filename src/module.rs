pub mod builder;
pub mod file;
pub mod read;
pub mod std_out;
pub mod tcp;
pub mod write;

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::class::Class;
use crate::function::Function;
use crate::pool_entry::PoolEntry;
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
  /// The module name.
  pub name: Rc<str>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The module functions.
  pub functions: BTreeMap<Rc<str>, Function>,
  /// The module classes.
  pub classes: BTreeMap<Rc<str>, Class>,
}

impl Module {
  pub fn fetch_function_with_name_unchecked(&self, function_name: &str) -> &Function {
    &self.functions[function_name]
  }
}

impl Module {
  pub const MAGIC: u32 = 0x75_76_61_73;

  pub fn fetch_function_with_name(&self, name: &str) -> Result<&Function> {
    self.functions.get(name).ok_or(Error::FunctionNotFound(name.to_string()))
  }
}
