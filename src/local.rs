use crate::value::Value;

#[derive(Debug)]
pub struct Local {
  local: Vec<Value>,
}

impl Local {
  pub fn new(capacity: usize) -> Self {
    Self { local: vec![Value::Integer(0); capacity] }
  }

  #[inline(always)]
  pub fn load(&self, index: usize) -> Value {
    self.local[index]
  }

  #[inline(always)]
  pub fn store(&mut self, index: usize, value: Value) {
    self.local[index] = value;
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
}
