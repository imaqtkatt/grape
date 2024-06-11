use std::slice::Iter;

use crate::value::Value;

#[derive(Debug)]
pub struct Local {
  pub local: Vec<Value>,
  base: usize,
}

impl Local {
  pub fn new(capacity: usize) -> Self {
    Self { local: vec![Value::mk_integer(0); capacity], base: 0 }
  }

  pub(crate) fn iter(&self) -> Iter<Value> {
    self.local.iter()
  }

  #[inline(always)]
  pub fn push_frame(&mut self, size: usize) -> usize {
    let new_base = self.local.len();
    let old_base = std::mem::replace(&mut self.base, new_base);
    self.local.resize(new_base + size, Value::mk_integer(0));
    old_base
  }

  #[inline(always)]
  pub fn pop_frame(&mut self, base: usize) {
    self.local.truncate(self.base);
    self.base = base;
  }

  #[inline(always)]
  pub fn load(&self, index: usize) -> Value {
    self.local[self.base + index]
  }

  #[inline(always)]
  pub fn store(&mut self, index: usize, value: Value) {
    self.local[self.base + index] = value;
  }

  #[inline(always)]
  pub fn iinc(&mut self, index: usize, inc: i32) {
    let value = &mut self.local[self.base + index];
    assert!(value.tag() == Value::TAG_INTEGER);
    unsafe { *value.integer_mut() += inc };
  }
}
