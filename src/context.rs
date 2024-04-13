use std::{collections::HashMap, fs::File, rc::Rc};

use crate::{
  function::Function,
  module::Module,
  module_path,
  runtime_error::{self, RtError},
};

#[derive(Default)]
pub struct Context {
  pub modules: HashMap<Box<str>, Rc<Module>>,
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
        v.insert(Rc::new(module));
      }
    }
  }

  pub fn fetch_module(&mut self, module_name: &str) -> runtime_error::Result<Rc<Module>> {
    match self.modules.get(module_name) {
      Some(module) => Ok(module.clone()),
      None => {
        // TODO: fixme
        let mut file = File::open(module_path::from(module_name))
          .map_err(|_| RtError::ModuleNotFound(module_name.to_string()))?;
        let module = Module::read(&mut file).map_err(runtime_error::other)?;
        Ok(
          self
            .modules
            .insert(Box::from(module_name), Rc::new(module))
            .unwrap(),
        )
      }
    }
  }

  pub fn fetch_function(&self, _module: &str, _function: &str) -> runtime_error::Result<&Function> {
    // self
    //   .modules
    //   .get(module)
    //   .and_then(|module| module.fetch_function(function))
    todo!()
  }
}
