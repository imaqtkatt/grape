pub mod mark_sweep;

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{
  runtime::{Error, Result},
  value::{Reference, Value},
};

pub struct Gc {
  roots: Vec<Value>,
  marked: HashSet<Value>,
}

impl Gc {
  #[inline(always)]
  pub fn track(&mut self, value: Value) {
    self.roots.push(value)
  }

  #[inline(always)]
  pub fn mark(&mut self, value: Value) -> bool {
    self.marked.insert(value)
  }
}

impl Gc {
  #[inline(always)]
  pub fn new() -> Self {
    Self { roots: Vec::new(), marked: HashSet::new() }
  }

  #[inline(always)]
  pub fn class(&mut self, fields: usize, class_ref: *const crate::class::Class) -> Value {
    let layout = std::alloc::Layout::new::<ObjClass>();
    let fields = vec![Value::NULL; fields].into();
    let ptr = unsafe {
      let ptr = std::alloc::alloc(layout);
      ptr.cast::<ObjClass>().write(ObjClass { fields, class_ref });
      ptr
    };
    let addr = ptr as usize;
    let value = Value::new(Value::TAG_CLASS, addr as u64);
    self.track(value);
    value
  }

  pub fn get_method(r#ref: Reference, name: &str) -> &crate::function::Function {
    let ptr = r#ref as *mut ObjClass;
    unsafe {
      let class_ref = (*ptr).class_ref;
      (*class_ref).fetch_function_with_name_unchecked(name)
    }
  }

  #[inline(always)]
  pub fn set_field_with_offset(r#ref: Reference, offset: u8, value: Value) {
    let ptr = r#ref as *mut ObjClass;
    unsafe { (*ptr).fields[offset as usize] = value }
  }

  #[inline(always)]
  pub fn set_field2(r#ref: Reference, field_name: &str, value: Value) {
    let ptr = r#ref as *mut ObjClass;
    unsafe {
      let class_ref = (*ptr).class_ref;
      let field = &(*class_ref).fields[field_name];
      (*ptr).fields[field.offset as usize] = value;
    }
  }

  #[inline(always)]
  pub fn get_field_with_offset(&self, r#ref: Reference, offset: u8) -> Value {
    let ptr = r#ref as *mut ObjClass;
    unsafe { (*ptr).fields[offset as usize] }
  }

  #[inline(always)]
  pub(crate) fn get_field2(r#ref: Reference, field_name: &str) -> Value {
    let ptr = r#ref as *mut ObjClass;
    unsafe {
      let class_ref = (*ptr).class_ref;
      let field = &(*class_ref).fields[field_name];
      (*ptr).fields[field.offset as usize]
    }
  }

  #[inline(always)]
  pub fn get_dict(r#ref: Reference, field: Value) -> Value {
    let ptr = r#ref as *mut ObjDict;
    unsafe { (*ptr).fields[&field] }
  }

  #[inline(always)]
  pub fn set_dict(r#ref: Reference, field: Value, value: Value) {
    let ptr = r#ref as *mut ObjDict;
    unsafe { (*ptr).fields.insert(field, value) };
  }

  #[inline(always)]
  pub fn alloc_array(&mut self, size: i32) -> Value {
    let layout = std::alloc::Layout::new::<ObjArray>();
    let ptr = unsafe {
      let ptr = std::alloc::alloc(layout);
      ptr
        .cast::<ObjArray>()
        .write(ObjArray { arr: vec![Value::NULL; size as usize].into_boxed_slice() });
      ptr
    };
    let addr = ptr as usize;
    let value = Value::new(Value::TAG_ARRAY, addr as u64);
    self.track(value);
    value
  }

  #[inline(always)]
  pub fn array_get(r#ref: Reference, index: i32) -> Value {
    let ptr = r#ref as *mut ObjArray;
    let index = index as usize;
    unsafe {
      if index > (*ptr).arr.len() {
        panic!()
      } else {
        (*ptr).arr[index]
      }
    }
  }

  #[inline(always)]
  pub fn array_set(r#ref: Reference, index: i32, value: Value) {
    let ptr = r#ref as *mut ObjArray;
    let index = index as usize;
    unsafe {
      if index > (*ptr).arr.len() {
        panic!()
      } else {
        (*ptr).arr[index] = value;
      }
    }
  }

  pub(crate) fn call_method(
    class_ref: usize,
    function_name: &str,
  ) -> Result<(*const crate::class::Class, *const crate::function::Function)> {
    let ptr = class_ref as *mut ObjClass;
    unsafe {
      let class_ref = (*ptr).class_ref;
      let function = (*class_ref).methods.get(function_name).ok_or(Error::FieldAccessError)?;
      Ok((class_ref, function))
    }
  }

  #[inline(always)]
  pub(crate) fn alloc_string(&mut self, s: String) -> Value {
    let layout = std::alloc::Layout::new::<ObjString>();
    let ptr = unsafe {
      let ptr = std::alloc::alloc(layout);
      ptr.cast::<ObjString>().write(ObjString { contents: s });
      ptr
    };

    let addr = ptr as usize;
    let value = Value::new(Value::TAG_STRING, addr as u64);
    self.track(value);
    value
  }

  #[inline(always)]
  pub(crate) fn alloc_dict(&mut self) -> Value {
    let layout = std::alloc::Layout::new::<ObjDict>();
    let ptr = unsafe {
      let ptr = std::alloc::alloc(layout);
      ptr.cast::<ObjDict>().write(ObjDict { fields: BTreeMap::new() });
      ptr
    };

    let addr = ptr as usize;
    let value = Value::new(Value::TAG_DICT, addr as u64);
    self.track(value);
    value
  }
}

#[derive(Debug)]
pub struct ObjClass {
  pub(crate) fields: Box<[Value]>,
  pub(crate) class_ref: *const crate::class::Class,
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

// #[derive(Debug)]
// pub struct ObjBytes {
//   pub bytes: Vec<u8>,
// }

impl ObjDict {
  pub fn refs(&self) -> BTreeSet<&Value> {
    let mut set = BTreeSet::new();
    for (key, value) in self.fields.iter() {
      if key.is_not_null() {
        set.insert(key);
      }
      if value.is_not_null() {
        set.insert(value);
      }
    }
    set
  }
}

impl ObjArray {
  pub fn refs(&self) -> BTreeSet<&Value> {
    self.arr.iter().filter_map(|v| if v.is_not_null() { Some(v) } else { None }).collect()
  }
}

impl ObjClass {
  pub fn refs(&self) -> BTreeSet<&Value> {
    self.fields.iter().filter_map(|v| if v.is_not_null() { Some(v) } else { None }).collect()
  }
}

impl Default for Gc {
  fn default() -> Self {
    Self::new()
  }
}

// impl ObjectType {
//   pub fn refs(&self) -> BTreeSet<Reference> {
//     match self {
//       ObjectType::Dict(map) => BTreeSet::new(),
//       ObjectType::Array(arr) => BTreeSet::new(),
//       ObjectType::Class(class) => class
//         .fields
//         .iter()
//         .filter_map(|v| if v.is_not_null() { Some(v.reference()) } else { None })
//         .collect(),
//       ObjectType::Null
//       | ObjectType::String(..)
//       | ObjectType::Bytes(..)
//       | ObjectType::Native(..) => BTreeSet::new(),
//     }
//   }
// }
