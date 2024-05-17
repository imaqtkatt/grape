use crate::{
  formatting,
  function::{Function, NativeRet},
  heap::Heap,
  local::Local,
};

use super::{builder::ModuleBuilder, Module};

fn println(local: &mut Local, heap: &mut Heap) -> NativeRet {
  println!("{}", formatting::display_value(&local.load_0(), heap));
  Ok(None)
}

fn print(local: &mut Local, heap: &mut Heap) -> NativeRet {
  print!("{}", formatting::display_value(&local.load_0(), heap));
  Ok(None)
}

fn debug(local: &mut Local, _: &mut Heap) -> NativeRet {
  println!("{:?}", local.load_0());
  Ok(None)
}

fn eprintln(local: &mut Local, heap: &mut Heap) -> NativeRet {
  eprintln!("{}", formatting::display_value(&local.load_0(), heap));
  Ok(None)
}

pub fn module() -> Module {
  ModuleBuilder::new()
    .with_name("std:out")
    .with_function(Function::native("println", 0, 1, println))
    .with_function(Function::native("print", 1, 1, print))
    .with_function(Function::native("debug", 2, 1, debug))
    .with_function(Function::native("eprintln", 3, 1, eprintln))
    .build()
}
