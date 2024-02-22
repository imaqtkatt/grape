use crate::value::{gfloat_t, gint_t, Value};

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

  pub fn dup(&mut self) {
    let value = self.pop();
    self.push(value);
    self.push(value);
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

  pub fn irem(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 % value2));
  }

  pub fn iand(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 & value2));
  }

  pub fn ior(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 | value2));
  }

  pub fn ixor(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 ^ value2));
  }

  pub fn ishl(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 << value2));
  }

  pub fn ishr(&mut self) {
    let value2: gint_t = self.pop().into();
    let value1: gint_t = self.pop().into();
    self.push(Value::Integer(value1 >> value2));
  }

  pub fn iushr(&mut self) {
    let rhs = gint_t::from(self.pop()) as u32;
    let lhs = gint_t::from(self.pop()) as u32;
    self.push(Value::Integer((lhs >> rhs) as i32));
  }

  pub fn ineg(&mut self) {
    let value: gint_t = self.pop().into();
    self.push(Value::Integer(value.wrapping_neg()))
  }

  pub fn i2f(&mut self) {
    let value: gint_t = self.pop().into();
    self.push(Value::Float(value as gfloat_t))
  }

  pub fn f2i(&mut self) {
    let value: gfloat_t = self.pop().into();
    self.push(Value::Integer(value as gint_t))
  }
}
