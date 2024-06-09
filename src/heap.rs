pub mod gc;

use std::{
  cell::Cell,
  collections::{BTreeMap, BTreeSet, HashSet},
};

use crate::value::{Reference, Value};

pub struct Heap {
  memory: Vec<Object>,
  free: HashSet<Reference, nohash_hasher::BuildNoHashHasher<Reference>>,
  freed: Vec<Reference>,
}

impl Heap {
  pub fn new() -> Self {
    Self {
      memory: vec![Object::marked(ObjectType::Null)],
      free: Default::default(),
      freed: Vec::new(),
    }
  }

  #[inline(always)]
  pub fn class(&mut self, fields: usize) -> Value {
    self
      .alloc(Object::new(ObjectType::Class(ObjClass { fields: vec![Value::NULL; fields].into() })))
  }

  #[inline(always)]
  pub fn put_field(&mut self, r#ref: Reference, offset: u8, value: Value) {
    let ObjectType::Class(class) = &mut *self.memory[r#ref].value else { panic!() };
    class.fields[offset as usize] = value;
  }

  #[inline(always)]
  pub fn get_field(&self, r#ref: Reference, offset: u8) -> Value {
    let ObjectType::Class(class) = &*self.memory[r#ref].value else { panic!() };
    class.fields[offset as usize]
  }

  #[inline(always)]
  pub fn new_dict(&mut self) -> Value {
    self.alloc(Object::new(ObjectType::Dict(ObjDict { fields: Default::default() })))
  }

  #[inline(always)]
  pub fn get_dict(&self, r#ref: Reference, field: Value) -> Value {
    let ObjectType::Dict(dict) = &*self.memory[r#ref].value else { panic!("Is not an object") };
    dict.fields[&field]
  }

  #[inline(always)]
  pub fn set_dict(&mut self, r#ref: Reference, field: Value, value: Value) {
    if let ObjectType::Dict(dict) = &mut *self.memory[r#ref].value {
      dict.fields.insert(field, value);
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn new_string(&mut self, s: String) -> Value {
    self.alloc(Object::new(ObjectType::String(ObjString { contents: s })))
  }

  #[inline(always)]
  pub fn new_array(&mut self, size: i32) -> Value {
    self.alloc(Object::new(ObjectType::Array(ObjArray {
      arr: vec![Value::NULL; size as usize].into_boxed_slice(),
    })))
  }

  #[inline(always)]
  pub fn array_get(&mut self, r#ref: Reference, index: i32) -> Value {
    let arr = &mut self.memory[r#ref];
    let index = index as usize;
    let ObjectType::Array(ObjArray { arr }) = &*arr.value else { panic!() };
    if index > arr.len() {
      panic!("Index is out of bounds")
    } else {
      arr[index]
    }
  }

  #[inline(always)]
  pub fn array_set(&mut self, r#ref: Reference, index: i32, value: Value) {
    let arr = &mut self.memory[r#ref];
    let index = index as usize;
    let ObjectType::Array(ObjArray { arr }) = &mut *arr.value else { panic!() };
    if index > arr.len() {
      panic!("Index is out of bounds")
    } else {
      arr[index] = value;
    }
  }

  #[inline(always)]
  pub fn new_bytes(&mut self, bytes_vec: Vec<u8>) -> Value {
    self.alloc(Object::new(ObjectType::Bytes(ObjBytes { bytes: bytes_vec })))
  }

  #[inline(always)]
  pub fn bytes_push(&mut self, r#ref: Reference, byte: u8) {
    let bytes = &mut self.memory[r#ref];
    let ObjectType::Bytes(ObjBytes { bytes }) = &mut *bytes.value else { panic!() };
    bytes.push(byte);
  }

  #[inline(always)]
  pub fn get(&self, r#ref: Reference) -> &Object {
    &self.memory[r#ref]
  }

  #[inline(always)]
  pub fn alloc(&mut self, obj: Object) -> Value {
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

  #[inline(always)]
  pub fn free(&mut self, r#ref: usize) {
    self.memory[r#ref] = Object::null();
    self.free.insert(r#ref);
    self.freed.push(r#ref);
  }
}

#[derive(Debug)]
pub struct Object {
  marked: Cell<bool>,
  pub value: Box<ObjectType>,
}

impl Object {
  pub fn null() -> Self {
    Self::new(ObjectType::Null)
  }

  pub fn new(object: ObjectType) -> Self {
    Self { marked: Cell::new(false), value: Box::new(object) }
  }

  pub(crate) fn marked(object: ObjectType) -> Self {
    Self { marked: Cell::new(true), value: Box::new(object) }
  }
}

#[derive(Debug)]
pub enum ObjectType {
  Null,
  String(ObjString),
  Dict(ObjDict),
  Array(ObjArray),
  Bytes(ObjBytes),
  Native(Box<dyn std::any::Any>),
  Class(ObjClass),
}

#[derive(Debug)]
pub struct ObjClass {
  pub(crate) fields: Box<[Value]>,
}

#[derive(Debug)]
pub struct ObjString {
  pub contents: String,
}

#[derive(Debug)]
pub struct ObjDict {
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

impl ObjectType {
  pub fn refs(&self) -> BTreeSet<Reference> {
    match self {
      ObjectType::Dict(map) => {
        let mut set = BTreeSet::new();
        for (key, value) in map.fields.iter() {
          if key.is_not_null() {
            set.insert(key.reference());
          }
          if value.is_not_null() {
            set.insert(value.reference());
          }
        }
        set
      }
      ObjectType::Array(arr) => arr
        .arr
        .iter()
        .filter_map(|v| if v.is_not_null() { Some(v.reference()) } else { None })
        .collect(),
      ObjectType::Class(class) => class
        .fields
        .iter()
        .filter_map(|v| if v.is_not_null() { Some(v.reference()) } else { None })
        .collect(),
      ObjectType::Null
      | ObjectType::String(..)
      | ObjectType::Bytes(..)
      | ObjectType::Native(..) => BTreeSet::new(),
    }
  }
}
