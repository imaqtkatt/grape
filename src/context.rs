use std::{
  collections::{btree_map::Entry, BTreeMap, BTreeSet},
  fs::File,
  rc::Rc,
};

use crate::{
  module::{Module, PoolEntry},
  module_path,
  runtime::{Error, Result},
};

#[derive(Default)]
pub struct ContextArena {
  modules: typed_arena::Arena<Module>,
}

impl ContextArena {
  pub fn new(n: usize) -> Self {
    Self { modules: typed_arena::Arena::with_capacity(n) }
  }
}

pub struct RuntimeContext<'c> {
  pub arena: &'c ContextArena,
  pub modules_map: BTreeMap<Rc<str>, u16>,
  modules: Vec<&'c Module>,
}

impl<'c> RuntimeContext<'c> {
  pub fn fetch_module(&mut self, module_name: &str) -> Result<&'c Module> {
    match self.modules_map.get(module_name).copied() {
      Some(module) => Ok(self.modules[module as usize]),
      None => Err(Error::ModuleNotFound(module_name.to_string())),
    }
  }

  pub fn fetch_module_indexed(&self, module_index: usize) -> &'c Module {
    self.modules[module_index]
  }
}

#[derive(Default)]
pub struct Context {
  pub modules_map: BTreeMap<Rc<str>, u16>,
  pub modules: Vec<Module>,
}

impl Context {
  pub fn new() -> Self {
    let modules = vec![
      crate::module::std_out::module(),
      crate::module::file::module(),
      crate::module::tcp::module(),
    ];
    let mut modules_map = BTreeMap::new();
    modules_map.insert(Rc::from("std:out"), 0);
    modules_map.insert(Rc::from("file"), 1);
    modules_map.insert(Rc::from("tcp"), 2);
    Self { modules_map, modules }
  }

  pub fn to_runtime_context<'c>(self, arena: &'c ContextArena) -> RuntimeContext<'c> {
    let mut modules: Vec<&'c Module> = Vec::with_capacity(self.modules.len());
    for mut module in self.modules.into_iter() {
      module.id = self.modules_map[&module.name];
      let module = arena.modules.alloc(module);
      modules.push(module);
    }
    RuntimeContext { arena, modules_map: self.modules_map, modules }
  }

  pub fn resolve_global_modules(&mut self) -> Result<()> {
    for module in self.modules.iter_mut() {
      for function in module.functions.iter_mut() {
        if let crate::function::Code::Bytecode(program) = &mut function.code {
          let mut idx = 0;
          while idx < program.len() {
            match program[idx] {
              crate::opcode::CALL => {
                let module_index = ((program[idx + 1] as usize) << 8) | program[idx + 2] as usize;
                let PoolEntry::Module(module_name) = &module.constants[module_index] else {
                  Err(Error::InvalidEntry(module_index))?
                };
                let global_module_index = self.modules_map[module_name.as_str()];
                program[idx + 1] = (global_module_index >> 8) as u8;
                program[idx + 2] = global_module_index as u8;
                idx += 5;
              }
              crate::opcode::HALT => idx += 1,
              crate::opcode::RETURN => idx += 1,
              crate::opcode::ICONST_0 => idx += 1,
              crate::opcode::ICONST_1 => idx += 1,
              crate::opcode::LOAD => idx += 2,
              crate::opcode::STORE => idx += 2,
              crate::opcode::FCONST_0 => idx += 1,
              crate::opcode::FCONST_1 => idx += 1,
              crate::opcode::LOAD_0 => idx += 1,
              crate::opcode::LOAD_1 => idx += 1,
              crate::opcode::LOAD_2 => idx += 1,
              crate::opcode::LOAD_3 => idx += 1,
              crate::opcode::I2F => idx += 1,
              crate::opcode::F2I => idx += 1,
              crate::opcode::LOADCONST => idx += 2,
              crate::opcode::GOTO => idx += 3,
              crate::opcode::I_PUSH_BYTE => idx += 2,
              crate::opcode::IADD => idx += 1,
              crate::opcode::ISUB => idx += 1,
              crate::opcode::IMUL => idx += 1,
              crate::opcode::I_IFLT => idx += 3,
              crate::opcode::I_IFGT => idx += 3,
              crate::opcode::I_IFGE => idx += 3,
              crate::opcode::IINC => idx += 3,
              crate::opcode::STORE_0 => idx += 1,
              crate::opcode::STORE_1 => idx += 1,
              crate::opcode::STORE_2 => idx += 1,
              crate::opcode::STORE_3 => idx += 1,
              crate::opcode::NEW_ARRAY => idx += 1,
              crate::opcode::ARRAY_GET => idx += 1,
              crate::opcode::ARRAY_SET => idx += 1,
              x => panic!("TODO: {}", crate::opcode::TO_STR[x as usize]),
            }
          }
        }
      }
    }

    Ok(())
  }

  pub fn add_module(&mut self, module: Module) -> Result<()> {
    match self.modules_map.entry(module.name.clone()) {
      Entry::Occupied(_) => Err(Error::ModuleAlreadyExists(module.name.to_string())),
      Entry::Vacant(v) => {
        let id = self.modules.len() as u16;
        v.insert(id);
        self.modules.push(module);
        Ok(())
      }
    }
  }

  pub fn load_eager(&mut self, module_name: &str) -> Result<()> {
    let mut loaded = BTreeSet::new();
    let mut to_load = vec![module_name.to_string()];

    while let Some(name) = to_load.pop() {
      if self.modules_map.contains_key(name.as_str()) {
        continue;
      }
      self.read_module(&name)?;
      let module_id = self.modules_map[name.as_str()];
      let module = &self.modules[module_id as usize];
      for constant in module.constants.iter() {
        if let crate::module::PoolEntry::Module(name) = constant {
          if loaded.insert(name.clone()) {
            to_load.push(name.clone());
          }
        }
      }
    }

    Ok(())
  }

  fn read_module(&mut self, module_name: &str) -> Result<()> {
    let mut file = File::open(module_path::from(module_name))
      .map_err(|_| Error::ModuleNotFound(module_name.to_string()))?;
    let module = Module::read(&mut file).map_err(Error::other)?;
    self.add_module(module)
  }
}
