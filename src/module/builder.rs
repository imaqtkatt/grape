use std::{collections::BTreeMap, rc::Rc};

use crate::function::Function;

use super::{Module, PoolEntry};

#[derive(Default)]
pub struct ModuleBuilder {
  name: String,
  constants: Vec<PoolEntry>,
  functions_map: BTreeMap<Rc<str>, u16>,
  functions: Vec<Function>,
}

impl ModuleBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self.constants.push(PoolEntry::Module(name.to_string()));
    self
  }

  pub fn with_constant(mut self, entry: PoolEntry) -> Self {
    self.constants.push(entry);
    self
  }

  pub fn with_function(mut self, function: Function) -> Self {
    let id = self.functions.len() as u16;
    self.functions_map.insert(function.name.clone(), id);
    self.functions.push(function);
    self
  }

  pub fn build(self) -> Module {
    Module {
      id: u16::MAX,
      name: Rc::from(self.name),
      constants: self.constants,
      functions_map: self.functions_map,
      functions: self.functions,
    }
  }
}
