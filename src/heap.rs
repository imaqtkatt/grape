use std::{
  cell::Cell,
  collections::{BTreeMap, BTreeSet, HashSet},
};

use crate::{
  local::Local,
  stack::Stack,
  value::{Reference, Value},
};

pub const HEAP_MEMORY: usize = 0xFFFF;

pub struct Heap {
  memory: Vec<Obj2>,
  free: HashSet<Reference>,
  freed: Vec<Reference>,
}

impl Heap {
  pub fn gc(&mut self, local: &Local, stack: &Stack) {
    for item in stack.iter() {
      if item.is_reference_non_null() {
        *self.memory[item.raw() as Reference].marked.get_mut() = true;
      }
    }

    for item in local.iter() {
      if item.is_reference_non_null() {
        let got = &mut self.memory[item.raw() as Reference];
        *got.marked.get_mut() = true;
        let mut refs = got.value.refs();
        while let Some(r#ref) = refs.pop_first() {
          let got = &mut self.memory[r#ref];
          *got.marked.get_mut() = true;
          let value_refs = got.value.refs();
          refs.extend(value_refs);
        }
      }
    }

    let mut to_free = Vec::new();
    for i in 1..self.memory.len() {
      let obj = &mut self.memory[i];
      if !obj.marked.get() && self.free.insert(i) {
        to_free.push(i);
      } else {
        *obj.marked.get_mut() = false;
      }
    }

    if to_free.is_empty() {
      return;
    }

    for r#ref in dbg!(to_free) {
      println!("Freed {:?}", self.memory.get(r#ref));
      self.freed.push(r#ref);
      self.memory[r#ref] = Obj2::new(Object::Null);
    }
  }

  pub fn new() -> Self {
    let mut memory = vec![Obj2::marked(Object::Null)];
    memory.reserve_exact(HEAP_MEMORY);
    Self { memory, free: HashSet::new(), freed: Vec::new() }
  }

  #[inline(always)]
  pub fn new_object(&mut self) -> Value {
    self.alloc(Obj2::new(Object::Map(ObjMap { fields: Default::default() })))
  }

  #[inline(always)]
  pub fn get_field(&self, obj_ref: Reference, field: Value) -> Value {
    let Object::Map(m) = &*self.memory[obj_ref].value else { panic!("Is not an object") };
    m.fields[&field]
  }

  #[inline(always)]
  pub fn set_field(&mut self, obj_ref: Reference, field: Value, value: Value) {
    if let Object::Map(m) = &mut *self.memory[obj_ref].value {
      m.fields.insert(field, value);
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn new_string(&mut self, s: String) -> Value {
    self.alloc(Obj2::new(Object::String(ObjString { contents: s })))
  }

  #[inline(always)]
  pub fn new_array(&mut self, size: i32) -> Value {
    self.alloc(Obj2::new(Object::Array(ObjArray {
      arr: vec![Value::NULL; size as usize].into_boxed_slice(),
    })))
  }

  #[inline(always)]
  pub fn array_get(&mut self, array_ref: Reference, index: i32) -> Value {
    let arr = &mut self.memory[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { arr }) = &*arr.value else { panic!() };
    if index > arr.len() {
      panic!("Index is out of bounds")
    } else {
      arr[index]
    }
  }

  #[inline(always)]
  pub fn array_set(&mut self, array_ref: Reference, index: i32, value: Value) {
    let arr = &mut self.memory[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { arr }) = &mut *arr.value else { panic!() };
    if index > arr.len() {
      panic!("Index is out of bounds")
    } else {
      arr[index] = value;
    }
  }

  #[inline(always)]
  pub fn new_bytes(&mut self, bytes_vec: Vec<u8>) -> Value {
    self.alloc(Obj2::new(Object::Bytes(ObjBytes { bytes: bytes_vec })))
  }

  #[inline(always)]
  pub fn bytes_push(&mut self, bytes_ref: Reference, byte: u8) {
    let bytes = &mut self.memory[bytes_ref];
    let Object::Bytes(ObjBytes { bytes }) = &mut *bytes.value else { panic!() };
    bytes.push(byte);
  }

  #[inline(always)]
  pub fn get(&self, r#ref: Reference) -> &Obj2 {
    &self.memory[r#ref]
  }

  #[inline(always)]
  fn alloc(&mut self, obj: Obj2) -> Value {
    if let Some(r#ref) = self.freed.pop() {
      self.free.remove(&r#ref);
      self.memory[r#ref] = obj;
      Value::mk_reference(r#ref)
    } else {
      let r#ref = self.memory.len();
      self.memory.push(obj);
      Value::mk_reference(r#ref)
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

  pub(crate) fn marked(object: Object) -> Self {
    Self { marked: Cell::new(true), value: Box::new(object) }
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
        .filter_map(|v| if v.is_reference_non_null() { Some(v.raw() as Reference) } else { None })
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
