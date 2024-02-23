#![allow(non_camel_case_types)]

use core::fmt;

use crate::heap::Heap;

/// Grape int type.
pub type g_int = i32;
/// Grape float type.
pub type g_float = f32;
/// Grape ref type.
pub type g_ref = usize;

#[derive(Clone, Copy)]
pub enum Value {
  Integer(g_int),
  Float(g_float),
  Object(g_ref),
  Array(g_ref),
  String(g_ref),
}

impl Value {
  // TODO: refactor this
  pub fn pretty(&self, heap: &Heap) {
    match self {
      Value::Integer(i) => println!("{i}"),
      Value::Float(f) => println!("{f}"),
      Value::Object(idx) | Value::Array(idx) | Value::String(idx) => heap.get(*idx).pretty(heap),
    }
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Integer(n) => write!(f, "{n}"),
      Value::Float(n) => write!(f, "{n}"),
      Value::Object(r) | Value::Array(r) | Value::String(r) => write!(f, "ref@{r:08}"),
    }
  }
}

impl From<Value> for g_int {
  fn from(value: Value) -> Self {
    if let Value::Integer(i) = value {
      i
    } else {
      panic!("Expected integer, found {value:?}")
    }
  }
}

impl From<Value> for g_float {
  fn from(value: Value) -> Self {
    if let Value::Float(i) = value {
      i
    } else {
      panic!("Expected integer, found {value:?}")
    }
  }
}

impl From<Value> for g_ref {
  fn from(value: Value) -> Self {
    match value {
      Value::Object(r) | Value::Array(r) | Value::String(r) => r,
      other => panic!("Expected a reference, found {other:?}"),
    }
  }
}
