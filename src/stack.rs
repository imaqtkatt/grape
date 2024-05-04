use crate::{
  runtime::{Error, Result},
  value::{g_float, g_int, Value},
};

pub struct Stack {
  stack: Vec<Value>,
}

impl Stack {
  pub fn new(capacity: usize) -> Self {
    Self { stack: Vec::with_capacity(capacity) }
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.stack.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Result<Value> {
    self.stack.pop().ok_or(Error::StackUnderflow)
  }

  pub fn dup(&mut self) -> Result<()> {
    let value = self.pop()?;
    self.push(value);
    self.push(value);
    Ok(())
  }

  pub fn iconst_0(&mut self) {
    self.push(Value::Integer(0));
  }

  pub fn iconst_1(&mut self) {
    self.push(Value::Integer(1));
  }

  pub fn fconst_0(&mut self) {
    self.push(Value::Float(ordered_float::OrderedFloat(0.)));
  }

  pub fn fconst_1(&mut self) {
    self.push(Value::Float(ordered_float::OrderedFloat(1.)));
  }

  pub fn push_byte(&mut self, byte: u8) {
    self.push(Value::Integer(byte as g_int));
  }

  pub fn push_short(&mut self, short: u16) {
    self.push(Value::Integer(short as g_int));
  }

  pub fn iadd(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 + value2));
    Ok(())
  }

  pub fn isub(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 - value2));
    Ok(())
  }

  pub fn imul(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 * value2));
    Ok(())
  }

  pub fn idiv(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 / value2));
    Ok(())
  }

  pub fn irem(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 % value2));
    Ok(())
  }

  pub fn iand(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 & value2));
    Ok(())
  }

  pub fn ior(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 | value2));
    Ok(())
  }

  pub fn ixor(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 ^ value2));
    Ok(())
  }

  pub fn ishl(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 << value2));
    Ok(())
  }

  pub fn ishr(&mut self) -> Result<()> {
    let value2: g_int = self.pop()?.into();
    let value1: g_int = self.pop()?.into();
    self.push(Value::Integer(value1 >> value2));
    Ok(())
  }

  pub fn iushr(&mut self) -> Result<()> {
    let rhs = g_int::from(self.pop()?) as u32;
    let lhs = g_int::from(self.pop()?) as u32;
    self.push(Value::Integer((lhs >> rhs) as i32));
    Ok(())
  }

  pub fn ineg(&mut self) -> Result<()> {
    let value: g_int = self.pop()?.into();
    self.push(Value::Integer(value.wrapping_neg()));
    Ok(())
  }

  pub fn i2f(&mut self) -> Result<()> {
    let value: g_int = self.pop()?.into();
    self.push(Value::Float(ordered_float::OrderedFloat(value as f32)));
    Ok(())
  }

  pub fn f2i(&mut self) -> Result<()> {
    let value: g_float = self.pop()?.into();
    self.push(Value::Integer(value.into_inner() as g_int));
    Ok(())
  }
}
