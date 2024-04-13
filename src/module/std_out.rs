use crate::{formatting, function::Function, heap, local, value};

use super::Module;

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
  Module {
    name: Box::from("std:out"),
    names: vec![
      String::from("std:out"),
      String::from("println"),
      String::from("print"),
      String::from("debug"),
      String::from("eprintln"),
    ],
    constants: Vec::new(),
    functions: vec![
      Function::native("println", 1, println),
      Function::native("print", 1, print),
      Function::native("debug", 1, debug),
      Function::native("eprintln", 1, eprintln),
    ],
  }
}
