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
  /// The module name.
  pub name: Rc<str>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The module functions.
  pub functions: BTreeMap<Rc<str>, Function>,
  /// The module classes.
  pub classes: BTreeMap<Rc<str>, Class>,
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

#[derive(Debug)]
pub struct Field {
  pub vis: u8,
  pub offset: u8,
}

impl Field {
  pub const PRIVATE: u8 = 0x0;
  pub const PUBLIC: u8 = 0x1;
}

impl Module {
  pub fn fetch_function_with_name_unchecked(&self, function_name: &str) -> &Function {
    &self.functions[function_name]
  }
}

impl Class {
  pub fn fetch_function_with_name_unchecked(&self, function_name: &str) -> &Function {
    &self.methods[function_name]
  }
}

#[derive(Clone, Debug)]
pub enum PoolEntry {
  String(String),
  Integer(i32),
  Module(String),
  Float(f32),
  Function(String),
  Class(String),
  Field(String, u16),
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
    self.functions.get(name).ok_or(Error::FunctionNotFound(name.to_string()))
  }
}
