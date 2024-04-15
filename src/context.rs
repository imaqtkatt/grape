use std::{
  collections::{hash_map::Entry, HashMap},
  fs::File,
  rc::Rc,
};

use crate::{
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

  pub fn add_module(&mut self, module: Module) -> runtime_error::Result<Rc<Module>> {
    let module_name = module.name.clone();
    match self.modules.entry(module_name.clone()) {
      Entry::Occupied(_) => Err(RtError::ModuleAlreadyExists(module_name.to_string())),
      Entry::Vacant(v) => Ok(v.insert(Rc::new(module)).to_owned()),
    }
  }

  pub fn fetch_module(&mut self, module_name: &str) -> runtime_error::Result<Rc<Module>> {
    match self.modules.get(module_name) {
      Some(module) => Ok(module.clone()),
      None => {
        let mut file = File::open(module_path::from(module_name))
          .map_err(|_| RtError::ModuleNotFound(module_name.to_string()))?;
        let module = Module::read(&mut file).map_err(runtime_error::other)?;
        self.add_module(module)
      }
    }
  }
}
