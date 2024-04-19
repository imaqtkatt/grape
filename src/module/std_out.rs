use crate::{formatting, function::Function, heap, local, value};

use super::{builder::ModuleBuilder, Module};

fn println(local: &local::Local, heap: &heap::Heap) -> Option<value::Value> {
  println!("{}", formatting::display_value(&local.load_0(), heap));
  None
}

fn print(local: &local::Local, heap: &heap::Heap) -> Option<value::Value> {
  print!("{}", formatting::display_value(&local.load_0(), heap));
  None
}

fn debug(local: &local::Local, _: &heap::Heap) -> Option<value::Value> {
  println!("{:?}", local.load_0());
  None
}

fn eprintln(local: &local::Local, heap: &heap::Heap) -> Option<value::Value> {
  eprintln!("{}", formatting::display_value(&local.load_0(), heap));
  None
}

pub fn module() -> Module {
  ModuleBuilder::new()
    .with_name("std:out")
    .with_function(Function::native("println", 0, 1, println))
    .with_function(Function::native("print", 1, 1, print))
    .with_function(Function::native("debug", 2, 1, debug))
    .with_function(Function::native("eprintln", 2, 1, eprintln))
    .build()
}