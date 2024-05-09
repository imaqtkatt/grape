use crate::{
  runtime::{Error, Result},
  value::{Float32, Int32, Value},
};

#[derive(Debug)]
pub struct Stack {
  pub stack: Vec<Value>,
}

impl Stack {
  #[inline(always)]
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

  #[inline(always)]
  pub fn check_underflow(&self, len: usize) -> Result<()> {
    if self.stack.len() < len {
      Err(Error::StackUnderflow)
    } else {
      Ok(())
    }
  }

  #[inline(always)]
  pub fn dup(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value = self.stack.pop().unwrap();
    self.push(value);
    self.push(value);
    Ok(())
  }

  #[inline(always)]
  pub fn iconst_0(&mut self) {
    self.push(Value::Integer(0));
  }

  #[inline(always)]
  pub fn iconst_1(&mut self) {
    self.push(Value::Integer(1));
  }

  #[inline(always)]
  pub fn fconst_0(&mut self) {
    self.push(Value::Float(ordered_float::OrderedFloat(0.)));
  }

  #[inline(always)]
  pub fn fconst_1(&mut self) {
    self.push(Value::Float(ordered_float::OrderedFloat(1.)));
  }

  #[inline(always)]
  pub fn push_byte(&mut self, byte: u8) {
    self.push(Value::Integer(byte as Int32));
  }

  #[inline(always)]
  pub fn push_short(&mut self, short: u16) {
    self.push(Value::Integer(short as Int32));
  }

  #[inline(always)]
  pub fn iadd(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 + value2));
    Ok(())
  }

  #[inline(always)]
  pub fn isub(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 - value2));
    Ok(())
  }

  #[inline(always)]
  pub fn imul(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 * value2));
    Ok(())
  }

  #[inline(always)]
  pub fn idiv(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 / value2));
    Ok(())
  }

  #[inline(always)]
  pub fn irem(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 % value2));
    Ok(())
  }

  #[inline(always)]
  pub fn iand(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 & value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ior(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 | value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ixor(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 ^ value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ishl(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 << value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ishr(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value1 >> value2));
    Ok(())
  }

  #[inline(always)]
  pub fn iushr(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let rhs = Int32::from(self.stack.pop().unwrap()) as u32;
    let lhs = Int32::from(self.stack.pop().unwrap()) as u32;
    self.push(Value::Integer((lhs >> rhs) as i32));
    Ok(())
  }

  #[inline(always)]
  pub fn ineg(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value.wrapping_neg()));
    Ok(())
  }

  #[inline(always)]
  pub fn i2f(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Int32 = self.stack.pop().unwrap().into();
    self.push(Value::Float(ordered_float::OrderedFloat(value as f32)));
    Ok(())
  }

  #[inline(always)]
  pub fn f2i(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Float32 = self.stack.pop().unwrap().into();
    self.push(Value::Integer(value.into_inner() as Int32));
    Ok(())
  }

  #[inline(always)]
  pub fn iflt(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.stack.pop().unwrap().into();
    let value1: Int32 = self.stack.pop().unwrap().into();
    Ok(value1 < value2)
  }
}
