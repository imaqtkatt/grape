use std::collections::HashMap;

use crate::{function::Function, module::Module};

#[derive(Default)]
pub struct Context {
  pub modules: HashMap<Box<str>, Module>,
}

impl Context {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_module(&mut self, module: Module) {
    let module_name = module.name.clone();
    match self.modules.entry(module_name) {
      std::collections::hash_map::Entry::Occupied(o) => {
        panic!("Module '{}' already exists.", o.key())
      }
      std::collections::hash_map::Entry::Vacant(v) => {
        v.insert(module);
      }
    }
  }

  pub fn fetch_module(&self, module_name: &str) -> Option<&Module> {
    self.modules.get(module_name)
  }

  pub fn fetch_function(&self, module: &str, function: &str) -> Option<&Function> {
    self
      .modules
      .get(module)
      .and_then(|module| module.fetch_function(function))
  }
}
