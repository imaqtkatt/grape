use core::fmt;

use crate::{heap::{Heap, ObjArray, ObjMap, ObjString, Object}, value::Value};

struct Formatting<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Formatting<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0(f)
  }
}

pub fn display_value<'a>(v: &'a Value, heap: &'a Heap) -> impl fmt::Display + 'a {
  Formatting(move |f| match v {
    Value::Integer(n) => write!(f, "{n}"),
    Value::Float(n) => write!(f, "{n}"),
    Value::Object(r#ref) | Value::Array(r#ref) | Value::String(r#ref) => {
      write!(f, "{}", display_object(*r#ref, heap))
    }
  })
}

pub fn display_object<'a>(o: usize, heap: &'a Heap) -> impl fmt::Display + 'a {
  Formatting(move |f| match heap.get(o) {
    Object::Null => write!(f, "null"),
    Object::String(ObjString { contents }) => write!(f, "{contents}"),
    Object::Map(ObjMap { fields }) => {
      writeln!(f, "{{")?;
      for (k, v) in fields.iter() {
        writeln!(
          f,
          "  {} -> {}",
          display_value(k, heap),
          display_value(v, heap)
        )?
      }
      write!(f, "}}")
    }
    Object::Array(ObjArray { len, arr }) => {
      write!(f, "[")?;
      for i in 0..*len {
        write!(f, "{};", display_value(&arr[i], heap))?;
      }
      write!(f, "]")
    }
  })
}
