use core::fmt;

use crate::{
  gc::{Gc, ObjArray, ObjDict, ObjString},
  value::Value,
};

struct Formatting<F: Fn(&mut fmt::Formatter) -> fmt::Result>(pub F);

impl<F: Fn(&mut fmt::Formatter) -> fmt::Result> fmt::Display for Formatting<F> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0(f)
  }
}

pub fn display_value<'a>(v: &'a Value, _heap: &'a Gc) -> impl fmt::Display + 'a {
  Formatting(move |f| match v.tag() {
    Value::TAG_BYTE => write!(f, "{}", v.byte()),
    Value::TAG_INTEGER => write!(f, "{}", v.integer()),
    Value::TAG_FLOAT => write!(f, "{}", v.float()),
    Value::TAG_NULL => write!(f, "null"),
    Value::TAG_STRING => {
      let ptr = v.reference() as *mut ObjString;
      write!(f, "{}", unsafe { &(*ptr).contents })
    }
    Value::TAG_DICT => {
      let ptr = v.reference() as *mut ObjDict;
      write!(f, "dict({:?})", unsafe { &(*ptr).fields })
    }
    Value::TAG_ARRAY => {
      let ptr = v.reference() as *mut ObjArray;
      write!(f, "array({:?})", unsafe { &(*ptr).arr })
    }
    Value::TAG_CLASS => write!(f, "class({:?})", v),
    _ => unreachable!(),
  })
}

// pub fn display_object(o: usize, heap: &Heap) -> impl fmt::Display + '_ {
//   Formatting(move |f| match &*heap.get(o).value {
//     ObjectType::Null => write!(f, "null"),
//     ObjectType::String(ObjString { contents }) => write!(f, "{contents}"),
//     ObjectType::Dict(ObjDict { fields }) => {
//       writeln!(f, "{{")?;
//       for (k, v) in fields.iter() {
//         writeln!(f, "  {} -> {}", display_value(k, heap), display_value(v, heap))?
//       }
//       write!(f, "}}")
//     }
//     ObjectType::Array(ObjArray { arr }) => {
//       write!(f, "[")?;
//       for value in arr.iter() {
//         write!(f, "{};", display_value(value, heap))?;
//       }
//       write!(f, "]")
//     }
//     ObjectType::Bytes(ObjBytes { bytes }) => {
//       write!(f, "<< ")?;
//       for byte in bytes {
//         write!(f, "{byte} ")?;
//       }
//       write!(f, ">>")
//     }
//     ObjectType::Native(_) => write!(f, "<native>"),
//     ObjectType::Class(_) => write!(f, "<class@{:012X}>", o),
//   })
// }
