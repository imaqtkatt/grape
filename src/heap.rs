pub mod gc;

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{
  runtime::{/*Error,*/ Result},
  value::{Reference, Value},
};

// const HEAP_MEMORY: usize = 1 << 13;

pub struct Heap {
  // memory: Vec<Object>,
  // free: HashSet<Reference, nohash_hasher::BuildNoHashHasher<Reference>>,
  // freed: Vec<Reference>,
  roots: Vec<Value>,
  marked: HashSet<Value>,
}

// TODO: rename
impl Heap {
  #[inline(always)]
  pub fn new() -> Self {
    // let mut memory = vec![Object::marked(ObjectType::Null)];
    // memory.reserve_exact(HEAP_MEMORY);
    Self {
      // memory,
      // free: Default::default(),
      // freed: Vec::new(),
      roots: Vec::new(),
      marked: HashSet::new(),
    }
  }

  #[inline(always)]
  pub fn class(&mut self, fields: usize, class_ref: *const crate::class::Class) -> Value {
    // self.alloc(Object::new(ObjectType::Class(ObjClass {
    //   fields: vec![Value::NULL; fields].into(),
    //   class_ref,
    // })))
    todo!()
  }

  pub fn get_method(&mut self, r#ref: Reference, name: &str) -> &crate::function::Function {
    todo!()
    // let ObjectType::Class(class) = &*self.memory[r#ref].value else { panic!() };
    // unsafe { (*class.class_ref).fetch_function_with_name_unchecked(name) }
  }

  #[inline(always)]
  pub fn set_field_with_offset(&mut self, r#ref: Reference, offset: u8, value: Value) {
    todo!()
    // let ObjectType::Class(class) = &mut *self.memory[r#ref].value else { panic!() };
    // class.fields[offset as usize] = value;
  }

  #[inline(always)]
  pub fn set_field2(&mut self, r#ref: Reference, field_name: &str, value: Value) {
    todo!()
    // let ObjectType::Class(class) = &mut *self.memory[r#ref].value else { panic!() };
    // let field = &unsafe { &*(class.class_ref) }.fields[field_name];
    // class.fields[field.offset as usize] = value;
  }

  #[inline(always)]
  pub fn get_field_with_offset(&self, r#ref: Reference, offset: u8) -> Value {
    todo!()
    // let ObjectType::Class(class) = &*self.memory[r#ref].value else { panic!() };
    // class.fields[offset as usize]
  }

  #[inline(always)]
  pub(crate) fn get_field2(&self, r#ref: Reference, field_name: &str) -> Value {
    todo!()
    // let ObjectType::Class(class) = &*self.memory[r#ref].value else { panic!() };
    // let field = &unsafe { &*(class.class_ref) }.fields[field_name];
    // class.fields[field.offset as usize]
  }

  // #[inline(always)]
  // pub fn new_dict(&mut self) -> Value {
  //   self.alloc(Object::new(ObjectType::Dict(ObjDict { fields: Default::default() })))
  // }

  #[inline(always)]
  pub fn get_dict(&self, r#ref: Reference, field: Value) -> Value {
    // let ObjectType::Dict(dict) = &*self.memory[r#ref].value else { panic!("Is not an object") };
    // dict.fields[&field]
    let ptr = r#ref as *mut ObjDict;
    unsafe { (*ptr).fields[&field] }
  }

  #[inline(always)]
  pub fn set_dict(&mut self, r#ref: Reference, field: Value, value: Value) {
    let ptr = r#ref as *mut ObjDict;
    unsafe { (*ptr).fields.insert(field, value) };
  }

  // #[inline(always)]
  // pub fn new_string(&mut self, s: String) -> Value {
  //   self.alloc(Object::new(ObjectType::String(ObjString { contents: s })))
  // }

  // #[inline(always)]
  // pub fn new_array(&mut self, size: i32) -> Value {
  //   self.alloc(Object::new(ObjectType::Array(ObjArray {
  //     arr: vec![Value::NULL; size as usize].into_boxed_slice(),
  //   })))
  // }

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
    self.roots.push(value);
    value
  }

  #[inline(always)]
  pub fn array_get(&mut self, r#ref: Reference, index: i32) -> Value {
    let ptr = r#ref as *mut ObjArray;
    let index = index as usize;
    unsafe {
      if index > (*ptr).arr.len() {
        panic!()
      } else {
        (*ptr).arr[index]
      }
    }
    // let arr = &mut self.memory[r#ref];
    // let index = index as usize;
    // let ObjectType::Array(ObjArray { arr }) = &*arr.value else { panic!() };
    // if index > arr.len() {
    //   panic!("Index is out of bounds")
    // } else {
    //   arr[index]
    // }
  }

  #[inline(always)]
  pub fn array_set(&mut self, r#ref: Reference, index: i32, value: Value) {
    let ptr = r#ref as *mut ObjArray;
    let index = index as usize;
    unsafe {
      if index > (*ptr).arr.len() {
        panic!()
      } else {
        (*ptr).arr[index] = value;
      }
    }
    // let arr = &mut self.memory[r#ref];
    // let index = index as usize;
    // let ObjectType::Array(ObjArray { arr }) = &mut *arr.value else { panic!() };
    // if index > arr.len() {
    //   panic!("Index is out of bounds")
    // } else {
    //   arr[index] = value;
    // }
  }

  // #[inline(always)]
  // pub fn new_bytes(&mut self, bytes_vec: Vec<u8>) -> Value {
  //   self.alloc(Object::new(ObjectType::Bytes(ObjBytes { bytes: bytes_vec })))
  // }

  // #[inline(always)]
  // pub fn bytes_push(&mut self, r#ref: Reference, byte: u8) {
  //   let bytes = &mut self.memory[r#ref];
  //   let ObjectType::Bytes(ObjBytes { bytes }) = &mut *bytes.value else { panic!() };
  //   bytes.push(byte);
  // }

  // #[inline(always)]
  // pub fn get(&self, r#ref: Reference) -> &Object {
  //   // &self.memory[r#ref]
  //   todo!()
  // }

  // #[inline(always)]
  // pub fn alloc(&mut self, obj: Object) -> Value {
  //   if let Some(r#ref) = self.freed.pop() {
  //     self.free.remove(&r#ref);
  //     self.memory[r#ref] = obj;
  //     Value::mk_reference(r#ref)
  //   } else {
  //     let r#ref = self.memory.len();
  //     self.memory.push(obj);
  //     Value::mk_reference(r#ref)
  //   }
  // }

  // #[inline(always)]
  // pub fn free(&mut self, r#ref: usize) {
  //   self.memory[r#ref] = Object::null();
  //   self.free.insert(r#ref);
  //   self.freed.push(r#ref);
  // }

  pub(crate) fn call_method(
    &self,
    class_ref: usize,
    function_name: &str,
  ) -> Result<(*const crate::class::Class, *const crate::function::Function)> {
    todo!()
    // let ObjectType::Class(class) = &*self.memory[class_ref].value else { panic!() };
    // let function =
    //   unsafe { (&*class.class_ref).methods.get(function_name).ok_or(Error::FieldAccessError) };
    // Ok((class.class_ref, function?))
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
    self.roots.push(value);
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
    self.roots.push(value);
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

#[derive(Debug)]
pub struct ObjBytes {
  pub bytes: Vec<u8>,
}

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

impl Default for Heap {
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
