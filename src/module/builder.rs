use crate::function::Function;

use super::{Module, PoolEntry};

#[derive(Default)]
pub struct ModuleBuilder {
  name: String,
  constants: Vec<PoolEntry>,
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
    self.functions.push(function);
    self
  }

  pub fn build(self) -> Module {
    let functions = self.functions.into_iter().map(|f| (f.name.clone(), f)).collect();
    Module {
      name: std::rc::Rc::from(self.name),
      constants: self.constants,
      functions,
      classes: Default::default(),
    }
  }
}
