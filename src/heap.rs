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

  pub fn new_array(&mut self, size: i32) -> Value {
    let r#ref = self.mem.len();
    self.mem.push(Object::Array(ObjArray {
      len: size as usize,
      arr: vec![Value::Object(0); size as usize],
    }));
    Value::Array(r#ref)
  }

  pub fn array_get(&mut self, array_ref: usize, index: i32) -> Value {
    let arr = &mut self.mem[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = arr else {
      panic!()
    };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index]
    }
  }

  pub fn array_set(&mut self, array_ref: usize, index: i32, value: Value) {
    let arr = &mut self.mem[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = arr else {
      panic!()
    };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index] = value;
    }
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
  // TODO: refactor this
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
      Object::Array(ObjArray { arr, .. }) => {
        print!("[");
        for el in arr {
          el.pretty(heap);
          print!(",");
        }
        println!("]");
      }
    }
  }
}
