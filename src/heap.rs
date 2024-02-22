use std::collections::HashMap;

use crate::value::Value;

pub const HEAP_MEM: usize = 0xFFF;

pub struct Heap {
  mem: Vec<Object>,
}

impl Heap {
  pub fn empty() -> Self {
    Self { mem: Vec::new() }
  }

  pub fn new() -> Self {
    let mut mem = vec![Object::Null];
    mem.reserve(HEAP_MEM);
    Self { mem }
  }

  pub fn new_string(&mut self, s: String) -> Value {
    let r#ref = self.mem.len();
    self.mem.push(Object::String(ObjString { contents: s }));
    Value::String(r#ref)
  }

  pub fn get(&self, index: usize) -> &Object {
    &self.mem[index]
  }
}

pub enum Object {
  Null,
  String(ObjString),
  Map(ObjMap),
  Array(ObjArray),
}

pub struct ObjString {
  contents: String,
}

pub struct ObjMap {
  fields: HashMap<Value, Value>,
}

pub struct ObjArray {
  len: usize,
  arr: Vec<Value>,
}

impl Object {
  pub fn pretty(&self, heap: &Heap) {
    match self {
      Object::Null => println!("null"),
      Object::String(ObjString { contents, .. }) => println!("{contents}"),
      Object::Map(m) => {
        println!("{{");
        for (k, v) in &m.fields {
          print!("  ");
          k.pretty(heap);
          print!(" -> ");
          v.pretty(heap);
          println!();
        }
        println!("}}");
      }
      Object::Array(_) => todo!(),
    }
  }
}
