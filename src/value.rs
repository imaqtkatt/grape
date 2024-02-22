#![allow(non_camel_case_types)]

use core::fmt;

/// Grape int type.
pub type gint_t = i32;
/// Grape float type.
pub type gfloat_t = f32;
/// Grape ref type.
pub type gref_t = usize;

#[derive(Clone, Copy, Debug)]
pub enum Value {
  Integer(gint_t),
  Float(gfloat_t),
  Object(gref_t),
  Array(gref_t),
  String(gref_t),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Integer(n) => write!(f, "{n}"),
      Value::Float(n) => write!(f, "{n}"),
      Value::Object(r) | Value::Array(r) | Value::String(r) => write!(f, "ref@{r:8}"),
    }
  }
}

impl From<Value> for gint_t {
  fn from(value: Value) -> Self {
    if let Value::Integer(i) = value {
      i
    } else {
      panic!("Expected integer, found {value:?}")
    }
  }
}

impl From<Value> for gfloat_t {
  fn from(value: Value) -> Self {
    if let Value::Float(i) = value {
      i
    } else {
      panic!("Expected integer, found {value:?}")
    }
  }
}

impl From<Value> for gref_t {
  fn from(value: Value) -> Self {
    match value {
      Value::Object(r) | Value::Array(r) | Value::String(r) => r,
      other => panic!("Expected a reference, found {other:?}"),
    }
  }
}
