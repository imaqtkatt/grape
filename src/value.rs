use core::fmt;

pub type Byte8 = u8;
/// Grape int type.
pub type Int32 = i32;
/// Grape float type.
pub type Float32 = ordered_float::OrderedFloat<f32>;
/// Grape reference type.
pub type Reference = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Value {
  Byte(Byte8),
  Integer(Int32),
  Float(Float32),
  Reference(Reference),
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Value::Byte(b) => write!(f, "{b}"),
      Value::Integer(n) => write!(f, "{n}"),
      Value::Float(n) => write!(f, "{n}"),
      Value::Reference(r) => write!(f, "@{r:08x}"),
    }
  }
}

impl From<Value> for Byte8 {
  fn from(value: Value) -> Self {
    match value {
      Value::Byte(b) => b,
      other => panic!("Expected byte, found {other:?}"),
    }
  }
}

impl From<Value> for Int32 {
  fn from(value: Value) -> Self {
    match value {
      Value::Integer(i) => i,
      other => panic!("Expected integer, found {other:?}"),
    }
  }
}

impl From<Value> for Float32 {
  fn from(value: Value) -> Self {
    match value {
      Value::Float(f) => f,
      other => panic!("Expected float, found {other:?}"),
    }
  }
}

impl From<Value> for Reference {
  fn from(value: Value) -> Self {
    match value {
      Value::Reference(r#ref) => r#ref,
      other => panic!("Expected a reference, found {other:?}"),
    }
  }
}
