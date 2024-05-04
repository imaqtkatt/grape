#![allow(non_camel_case_types)]

use core::fmt;

/// Grape int type.
pub type g_int = i32;
/// Grape float type.
pub type g_float = ordered_float::OrderedFloat<f32>;
/// Grape reference type.
pub type g_ref = usize;

#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
  Integer(g_int),
  Float(g_float),
  Reference(g_ref),
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Integer(n) => write!(f, "{n}"),
      Value::Float(n) => write!(f, "{n}"),
      Value::Reference(r) => write!(f, "@{r:08x}"),
    }
  }
}

impl From<Value> for g_int {
  fn from(value: Value) -> Self {
    match value {
      Value::Integer(i) => i,
      other => panic!("Expected integer, found {other:?}"),
    }
  }
}

impl From<Value> for g_float {
  fn from(value: Value) -> Self {
    match value {
      Value::Float(f) => f,
      other => panic!("Expected float, found {other:?}"),
    }
  }
}

impl From<Value> for g_ref {
  fn from(value: Value) -> Self {
    match value {
      Value::Reference(r#ref) => r#ref,
      other => panic!("Expected a reference, found {other:?}"),
    }
  }
}
