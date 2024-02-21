use crate::value::{gint_t, Value};

pub struct Stack {
  stack: Vec<Value>,
}

impl Stack {
  pub fn new(capacity: usize) -> Self {
    Self {
      stack: Vec::with_capacity(capacity),
    }
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Value {
    self.stack.pop().expect("To not be empty")
  }

  pub fn iconst_0(&mut self) {
    self.push(Value::Integer(0));
  }

  pub fn iconst_1(&mut self) {
    self.push(Value::Integer(1));
  }

  pub fn fconst_0(&mut self) {
    self.push(Value::Float(0.));
  }

  pub fn fconst_1(&mut self) {
    self.push(Value::Float(1.));
  }

  pub fn push_byte(&mut self, byte: u8) {
    self.push(Value::Integer(byte as gint_t));
  }

  pub fn push_short(&mut self, short: u16) {
    self.push(Value::Integer(short as gint_t));
  }

  pub fn iadd(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 + value2));
  }

  pub fn isub(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 - value2));
  }

  pub fn imul(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 * value2));
  }

  pub fn idiv(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 / value2));
  }
}
