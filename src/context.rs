use std::{
  collections::{btree_map::Entry, BTreeMap},
  fs::File,
  rc::Rc,
};

use crate::{
  module::Module,
  module_path,
  runtime::{Error, Result},
};

pub struct Context {
  pub modules: BTreeMap<Box<str>, Rc<Module>>,
}

impl Context {
  pub const fn new() -> Self {
    Self { modules: BTreeMap::new() }
  }

  #[inline(always)]
  pub fn add_module(&mut self, module: Module) -> Result<Rc<Module>> {
    match self.modules.entry(module.name.clone()) {
      Entry::Occupied(o) => Err(Error::ModuleAlreadyExists(o.get().name.to_string())),
      Entry::Vacant(v) => Ok(v.insert(Rc::new(module)).to_owned()),
    }
  }

  pub fn fetch_module(&mut self, module_name: &str) -> Result<Rc<Module>> {
    match self.modules.get(module_name) {
      Some(module) => Ok(module.clone()),
      None => {
        let mut file = File::open(module_path::from(module_name))
          .map_err(|_| Error::ModuleNotFound(module_name.to_string()))?;
        let module = Module::read(&mut file).map_err(Error::other)?;
        self.add_module(module)
      }
    }
  }
}

impl Default for Context {
  fn default() -> Self {
    Self::new()
  }
}
