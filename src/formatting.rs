use core::fmt;

use crate::{
  heap::{Heap, ObjArray, ObjBytes, ObjMap, ObjString, Object},
  value::Value,
};

struct Formatting<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Formatting<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0(f)
  }
}

pub fn display_value<'a>(v: &'a Value, heap: &'a Heap) -> impl fmt::Display + 'a {
  Formatting(move |f| match v.tag() {
    Value::TAG_BYTE => write!(f, "{}", v.raw() as u8),
    Value::TAG_INTEGER => write!(f, "{}", v.raw() as i32),
    Value::TAG_FLOAT => write!(f, "{}", v.raw() as f32),
    Value::TAG_REFERENCE => write!(f, "{}", display_object(v.raw() as usize, heap)),
    _ => unreachable!(),
  })
}

pub fn display_object(o: usize, heap: &Heap) -> impl fmt::Display + '_ {
  Formatting(move |f| match heap.get(o) {
    Object::Null => write!(f, "null"),
    Object::String(ObjString { contents }) => write!(f, "{contents}"),
    Object::Map(ObjMap { fields }) => {
      writeln!(f, "{{")?;
      for (k, v) in fields.iter() {
        writeln!(f, "  {} -> {}", display_value(k, heap), display_value(v, heap))?
      }
      write!(f, "}}")
    }
    Object::Array(ObjArray { len, arr }) => {
      write!(f, "[")?;
      for value in arr.iter().take(*len) {
        write!(f, "{};", display_value(value, heap))?;
      }
      write!(f, "]")
    }
    Object::Bytes(ObjBytes { bytes }) => {
      write!(f, "<< ")?;
      for byte in bytes {
        write!(f, "{byte} ")?;
      }
      write!(f, ">>")
    }
  })
}
