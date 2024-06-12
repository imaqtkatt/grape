use std::{
  collections::{btree_map::Entry, BTreeMap, BTreeSet},
  fs::File,
  rc::Rc,
};

use crate::{
  class::Class,
  module::Module,
  module_path,
  runtime::{Error, Result},
};

#[derive(Default)]
pub struct ContextArena {
  modules: typed_arena::Arena<Module>,
  classes: typed_arena::Arena<Class>,
}

pub struct Context<'c> {
  arena: &'c ContextArena,
  pub(crate) modules: BTreeMap<Rc<str>, &'c Module>,
  pub(crate) classes: BTreeMap<Rc<str>, &'c Class>,
}

impl<'c> Context<'c> {
  pub fn new(arena: &'c ContextArena) -> Self {
    let std_out: &'c Module = arena.modules.alloc(crate::module::std_out::module());
    let file: &'c Module = arena.modules.alloc(crate::module::file::module());
    let tcp: &'c Module = arena.modules.alloc(crate::module::tcp::module());
    let mut modules = BTreeMap::new();
    modules.insert(Rc::from("std:out"), std_out);
    modules.insert(Rc::from("file"), file);
    modules.insert(Rc::from("tcp"), tcp);
    Self { arena, modules, classes: Default::default() }
  }

  pub fn add_module(&mut self, module: Module) -> Result<&'c Module> {
    match self.modules.entry(module.name.clone()) {
      Entry::Vacant(v) => Ok(v.insert(self.arena.modules.alloc(module))),
      Entry::Occupied(o) => Err(Error::ModuleAlreadyExists(o.get().name.to_string())),
    }
  }

  pub fn add_class(&mut self, class: Class) -> Result<&'c Class> {
    match self.classes.entry(class.name.clone()) {
      Entry::Vacant(v) => Ok(v.insert(self.arena.classes.alloc(class))),
      Entry::Occupied(o) => Err(Error::ClassAlreadyExists(o.get().name.to_string())),
    }
  }

  pub fn fetch_module(&mut self, module_name: &str) -> Result<&'c Module> {
    match self.modules.get(module_name) {
      Some(module) => Ok(module),
      None => Err(Error::ModuleNotFound(module_name.to_string())),
    }
  }

  pub fn fetch_class(&self, class_name: &str) -> Result<&'c Class> {
    match self.classes.get(class_name) {
      Some(class) => Ok(class),
      None => Err(Error::ClassNotFound(class_name.to_string())),
    }
  }

  fn read_module(&mut self, module_name: &str) -> Result<Module> {
    let mut file = File::open(module_path::from(module_name))
      .map_err(|_| Error::ModuleNotFound(module_name.to_string()))?;
    Module::read(&mut file).map_err(Error::other)
  }

  pub fn load_eager(&mut self, module_name: &str) -> Result<()> {
    let mut loaded = BTreeSet::new();
    let mut to_load = vec![module_name];

    while let Some(name) = to_load.pop() {
      if self.modules.contains_key(name) {
        continue;
      }
      let mut module = self.read_module(name)?;

      let classes = std::mem::replace(&mut module.classes, BTreeMap::new());
      for class in classes.into_values() {
        self.add_class(class)?;
      }

      let module = self.add_module(module)?;
      for constant in module.constants.iter() {
        if let crate::pool_entry::PoolEntry::Module(name) = constant {
          if loaded.insert(name) {
            to_load.push(name);
          }
        }
      }
    }

    Ok(())
  }
}
