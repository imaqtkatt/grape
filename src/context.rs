use std::{
  collections::{btree_map::Entry, BTreeMap, BTreeSet},
  fs::File,
  rc::Rc,
};

use crate::{
  module::{Class, Module},
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
  modules: BTreeMap<Rc<str>, &'c Module>,
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
      Entry::Occupied(_) => todo!(),
    }
  }

  pub fn fetch_module(&mut self, module_name: &str) -> Result<&'c Module> {
    match self.modules.get(module_name) {
      Some(module) => Ok(module),
      None => self.read_module(module_name),
    }
  }

  pub fn fetch_class(&self, class_name: &str) -> Result<&'c Class> {
    match self.classes.get(class_name) {
      Some(class) => Ok(class),
      None => todo!(),
    }
  }

  fn read_module(&mut self, module_name: &str) -> Result<&'c Module> {
    let mut file = File::open(module_path::from(module_name))
      .map_err(|_| Error::ModuleNotFound(module_name.to_string()))?;
    let module = Module::read(&mut file).map_err(Error::other)?;
    self.add_module(module)
  }

  pub fn load_eager(&mut self, module_name: &str) -> Result<()> {
    let mut loaded = BTreeSet::new();
    let mut to_load = vec![module_name];

    while let Some(name) = to_load.pop() {
      let module = self.fetch_module(name)?;
      for constant in module.constants.iter() {
        if let crate::module::PoolEntry::Module(name) = constant {
          if loaded.insert(name) {
            to_load.push(name);
          }
        }
      }
    }

    Ok(())
  }
}
