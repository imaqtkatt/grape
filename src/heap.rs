use std::{
  cell::Cell,
  collections::{BTreeMap, BTreeSet},
};

use crate::{
  local::Local,
  stack::Stack,
  value::{Reference, Value},
};

pub const HEAP_MEM: usize = 0xFFFF;

pub struct Heap {
  memory: Vec<Obj2>,
  freed: Vec<Reference>,
}

impl Heap {
  pub fn gc(&mut self, local: &Local, stack: &Stack) {
    println!("Running GC...");

    for item in stack.0.iter() {
      if item.tag() == Value::TAG_REFERENCE {
        let obj = &mut self.memory[item.raw() as Reference];
        *obj.marked.get_mut() = true;
      }
    }

    println!("local: {:?}", local.local);
    for item in local.iter() {
      if item.tag() == Value::TAG_REFERENCE {
        let got = &mut self.memory[item.raw() as Reference];
        *got.marked.get_mut() = true;
        let refs = got.value.refs();
        for r#ref in refs {
          *self.memory[r#ref].marked.get_mut() = true;
        }
      }
    }

    let mut to_free = Vec::new();
    for i in 1..self.memory.len() {
      let obj = &mut self.memory[i];
      if !obj.marked.get() && !self.freed.contains(&i) {
        to_free.push(i);
      } else {
        *obj.marked.get_mut() = false;
      }
    }

    for r#ref in dbg!(to_free) {
      println!("Freed {:?}", self.memory.get(r#ref));
      self.freed.push(r#ref);
      self.memory[r#ref] = Obj2::new(Object::Null);
    }
  }

  pub fn new() -> Self {
    let mut memory = vec![Obj2 { marked: Cell::new(false), value: Box::new(Object::Null) }];
    memory.reserve_exact(HEAP_MEM);
    Self { memory, freed: Vec::new() }
  }

  #[inline(always)]
  pub fn new_object(&mut self) -> Value {
    let r#ref = self.new_ref();
    self.memory.push(Obj2::new(Object::Map(ObjMap { fields: Default::default() })));
    r#ref
  }

  #[inline(always)]
  pub fn get_field(&self, obj_ref: usize, field: Value) -> Value {
    if let Object::Map(m) = &*self.memory[obj_ref].value {
      m.fields[&field]
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn set_field(&mut self, obj_ref: usize, field: Value, value: Value) {
    if let Object::Map(m) = &mut *self.memory[obj_ref].value {
      m.fields.insert(field, value);
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn new_string(&mut self, s: String) -> Value {
    let r#ref = self.new_ref();
    self.memory.push(Obj2::new(Object::String(ObjString { contents: s })));
    r#ref
  }

  #[inline(always)]
  pub fn new_array(&mut self, size: i32) -> Value {
    let r#ref = self.new_ref();
    self.memory.push(Obj2::new(Object::Array(ObjArray {
      len: size as usize,
      arr: vec![Value::mk_reference(0); size as usize].into_boxed_slice(),
    })));
    r#ref
  }

  #[inline(always)]
  pub fn array_get(&mut self, array_ref: usize, index: i32) -> Value {
    let arr = &mut self.memory[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = &*arr.value else { panic!() };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index]
    }
  }

  #[inline(always)]
  pub fn array_set(&mut self, array_ref: usize, index: i32, value: Value) {
    let arr = &mut self.memory[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = &mut *arr.value else { panic!() };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index] = value;
    }
  }

  #[inline(always)]
  pub fn new_bytes(&mut self, bytes_vec: Vec<u8>) -> Value {
    let r#ref = self.new_ref();
    self.memory.push(Obj2::new(Object::Bytes(ObjBytes { bytes: bytes_vec })));
    r#ref
  }

  #[inline(always)]
  pub fn bytes_push(&mut self, bytes_ref: usize, byte: u8) {
    let bytes = &mut self.memory[bytes_ref];
    let Object::Bytes(ObjBytes { bytes }) = &mut *bytes.value else { panic!() };
    bytes.push(byte);
  }

  #[inline(always)]
  // pub fn get(&self, index: usize) -> &Object {
  pub fn get(&self, r#ref: Reference) -> &Obj2 {
    &self.memory[r#ref]
  }

  #[inline(always)]
  fn new_ref(&mut self) -> Value {
    if let Some(r#ref) = self.freed.pop() {
      Value::mk_reference(r#ref)
    } else {
      Value::mk_reference(self.memory.len())
    }
  }
}

#[derive(Debug)]
pub struct Obj2 {
  marked: Cell<bool>,
  pub value: Box<Object>,
}

impl Obj2 {
  pub fn new(object: Object) -> Self {
    Self { marked: Cell::new(false), value: Box::new(object) }
  }
}

#[derive(Debug)]
pub enum Object {
  Null,
  String(ObjString),
  Map(ObjMap),
  Array(ObjArray),
  Bytes(ObjBytes),
}

impl Object {
  pub fn refs(&self) -> BTreeSet<Reference> {
    match self {
      Object::String(_) | Object::Null | Object::Bytes(_) => BTreeSet::new(),
      Object::Map(_) => todo!(),
      Object::Array(arr) => arr
        .arr
        .iter()
        .filter_map(|v| {
          if v.tag() == Value::TAG_REFERENCE && v.raw() > 0 {
            Some(v.raw() as Reference)
          } else {
            None
          }
        })
        .collect(),
    }
  }
}

#[derive(Debug)]
pub struct ObjString {
  pub contents: String,
}

#[derive(Debug)]
pub struct ObjMap {
  pub fields: BTreeMap<Value, Value>,
}

#[derive(Debug)]
pub struct ObjArray {
  pub len: usize,
  pub arr: Box<[Value]>,
}

#[derive(Debug)]
pub struct ObjBytes {
  pub bytes: Vec<u8>,
}

impl Default for Heap {
  fn default() -> Self {
    Self::new()
  }
}
