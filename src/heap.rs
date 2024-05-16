use std::collections::BTreeMap;

use crate::value::Value;

pub const HEAP_MEM: usize = 0xFFFF;

pub struct Heap {
  mem: Vec<Object>,
}

impl Heap {
  pub fn new() -> Self {
    let mut mem = vec![Object::Null];
    mem.reserve_exact(HEAP_MEM);
    Self { mem }
  }

  #[inline(always)]
  pub fn new_object(&mut self) -> Value {
    let r#ref = self.new_ref();
    self.mem.push(Object::Map(ObjMap { fields: Default::default() }));
    r#ref
  }

  #[inline(always)]
  pub fn get_field(&self, obj_ref: usize, field: Value) -> Value {
    if let Object::Map(m) = &self.mem[obj_ref] {
      m.fields[&field]
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn set_field(&mut self, obj_ref: usize, field: Value, value: Value) {
    if let Object::Map(m) = &mut self.mem[obj_ref] {
      m.fields.insert(field, value);
    } else {
      panic!("Is not an object")
    }
  }

  #[inline(always)]
  pub fn new_string(&mut self, s: String) -> Value {
    let r#ref = self.new_ref();
    self.mem.push(Object::String(ObjString { contents: s }));
    r#ref
  }

  #[inline(always)]
  pub fn new_array(&mut self, size: i32) -> Value {
    let r#ref = self.new_ref();
    self.mem.push(Object::Array(ObjArray {
      len: size as usize,
      arr: vec![Value::mk_reference(0); size as usize].into_boxed_slice(),
    }));
    r#ref
  }

  #[inline(always)]
  pub fn array_get(&mut self, array_ref: usize, index: i32) -> Value {
    let arr = &mut self.mem[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = arr else { panic!() };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index]
    }
  }

  #[inline(always)]
  pub fn array_set(&mut self, array_ref: usize, index: i32, value: Value) {
    let arr = &mut self.mem[array_ref];
    let index = index as usize;
    let Object::Array(ObjArray { len, arr }) = arr else { panic!() };
    if index > *len {
      panic!("Index is out of bounds")
    } else {
      arr[index] = value;
    }
  }

  #[inline(always)]
  pub fn new_bytes(&mut self, bytes_vec: Vec<u8>) -> Value {
    let r#ref = self.new_ref();
    self.mem.push(Object::Bytes(ObjBytes { bytes: bytes_vec }));
    r#ref
  }

  #[inline(always)]
  pub fn bytes_push(&mut self, bytes_ref: usize, byte: u8) {
    let bytes = &mut self.mem[bytes_ref];
    let Object::Bytes(ObjBytes { bytes }) = bytes else { panic!() };
    bytes.push(byte);
  }

  #[inline(always)]
  pub fn get(&self, index: usize) -> &Object {
    &self.mem[index]
  }

  #[inline(always)]
  fn new_ref(&mut self) -> Value {
    Value::mk_reference(self.mem.len())
  }
}

pub enum Object {
  Null,
  String(ObjString),
  Map(ObjMap),
  Array(ObjArray),
  Bytes(ObjBytes),
}

pub struct ObjString {
  pub contents: String,
}

pub struct ObjMap {
  pub fields: BTreeMap<Value, Value>,
}

pub struct ObjArray {
  pub len: usize,
  pub arr: Box<[Value]>,
}

pub struct ObjBytes {
  pub bytes: Vec<u8>,
}

impl Default for Heap {
  fn default() -> Self {
    Self::new()
  }
}
