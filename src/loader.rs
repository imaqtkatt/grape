use std::{
  collections::{btree_map::Entry, BTreeMap, BTreeSet},
  rc::Rc,
};

use crate::{
  class::Class,
  context::Context,
  module::Module,
  module_path,
  runtime::{Error, Result},
};

#[derive(Default)]
pub struct LoaderArena {
  modules: typed_arena::Arena<Module>,
  classes: typed_arena::Arena<Class>,
}

pub struct Loader<'c> {
  arena: &'c LoaderArena,
  modules: BTreeMap<Rc<str>, &'c Module>,
  classes: BTreeMap<Rc<str>, &'c Class>,
}

impl<'c> Loader<'c> {
  pub fn new(arena: &'c LoaderArena) -> Self {
    let std_out: &'c Module = arena.modules.alloc(crate::module::std_out::module());
    let file: &'c Module = arena.modules.alloc(crate::module::file::module());
    let tcp: &'c Module = arena.modules.alloc(crate::module::tcp::module());
    let mut modules = BTreeMap::new();
    modules.insert(Rc::from("std:out"), std_out);
    modules.insert(Rc::from("file"), file);
    modules.insert(Rc::from("tcp"), tcp);
    Self { arena, modules, classes: Default::default() }
  }

  pub fn to_context(self) -> Context<'c> {
    Context { modules: self.modules, classes: self.classes }
  }

  pub fn load_path(&mut self, module: &str) -> Result<()> {
    let mut loaded = BTreeSet::new();
    let mut to_load = vec![module];

    while let Some(module) = to_load.pop() {
      if self.modules.contains_key(module) {
        continue;
      }
      let mut module = self.read_module(module)?;

      let classes = std::mem::take(&mut module.classes);
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

  fn read_module(&mut self, module_name: &str) -> Result<Module> {
    let mut file = std::fs::File::open(module_path::from(module_name))
      .map_err(|_| Error::ModuleNotFound(module_name.to_string()))?;
    Module::read(&mut file).map_err(Error::other)
  }

  fn add_module(&mut self, module: Module) -> Result<&'c Module> {
    match self.modules.entry(module.name.clone()) {
      Entry::Vacant(v) => Ok(v.insert(self.arena.modules.alloc(module))),
      Entry::Occupied(o) => Err(Error::ModuleAlreadyExists(o.get().name.to_string())),
    }
  }

  fn add_class(&mut self, class: Class) -> Result<()> {
    match self.classes.entry(class.name.clone()) {
      Entry::Vacant(v) => {
        v.insert(self.arena.classes.alloc(class));
        Ok(())
      }
      Entry::Occupied(o) => Err(Error::ClassAlreadyExists(o.get().name.to_string())),
    }
  }
}
