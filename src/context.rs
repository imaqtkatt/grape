use std::{collections::BTreeMap, rc::Rc};

use crate::{
  class::Class,
  module::Module,
  runtime::{Error, Result},
};

pub struct Context<'c> {
  pub(crate) modules: BTreeMap<Rc<str>, &'c Module>,
  pub(crate) classes: BTreeMap<Rc<str>, &'c Class>,
}

impl<'c> Context<'c> {
  pub fn fetch_module(&mut self, module_name: &str) -> Result<&'c Module> {
    self.modules.get(module_name).copied().ok_or(Error::ModuleNotFound(module_name.to_string()))
  }

  pub fn fetch_class(&self, class_name: &str) -> Result<&'c Class> {
    self.classes.get(class_name).copied().ok_or(Error::ClassNotFound(class_name.to_string()))
  }
}
