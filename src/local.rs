use crate::value::Value;

#[derive(Debug)]
pub struct Local {
  local: Vec<Value>,
  base: usize,
}

impl Local {
  pub fn new(capacity: usize) -> Self {
    Self { local: vec![Value::mk_integer(0); capacity], base: 0 }
  }

  pub fn push_frame(&mut self, size: usize) -> usize {
    let old_base = self.base;
    let new_base = self.local.len();
    self.local.resize(new_base + size, Value::mk_integer(0));
    self.base = new_base;
    old_base
  }

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

  pub fn load_0(&self) -> Value {
    self.load(0)
  }

  pub fn load_1(&self) -> Value {
    self.load(1)
  }

  pub fn load_2(&self) -> Value {
    self.load(2)
  }

  pub fn load_3(&self) -> Value {
    self.load(3)
  }

  #[inline(always)]
  pub fn iinc(&mut self, index: usize, inc: i32) {
    let value = &mut self.local[self.base + index];
    assert!(value.tag() == Value::TAG_INTEGER);
    unsafe { *value.integer_mut() += inc };
  }
}
