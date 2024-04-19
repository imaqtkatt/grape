use crate::function::Function;

use super::{Module, PoolEntry};

#[derive(Default)]
pub struct ModuleBuilder {
  name: String,
  names: Vec<String>,
  constants: Vec<PoolEntry>,
  functions: Vec<Function>,
}

impl ModuleBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self.names.push(name.to_string());
    self
  }

  pub fn with_constant_module_name(mut self, name: &str) -> Self {
    self.names.push(name.to_string());
    self
  }

  pub fn with_constant_module_names(mut self, names: Vec<String>) -> Self {
    self.names = names;
    self
  }

  pub fn with_constant(mut self, entry: PoolEntry) -> Self {
    self.constants.push(entry);
    self
  }

  pub fn with_function(mut self, function: Function) -> Self {
    self.functions.push(function);
    self
  }

  pub fn build(self) -> Module {
    Module {
      name: self.name.into_boxed_str(),
      names: self.names,
      constants: self.constants,
      functions: self.functions,
    }
  }
}