use std::ops::Neg;

use crate::{
  runtime::{Error, Result},
  value::{Byte8, Float32, Int32, Value},
};

#[derive(Debug)]
pub struct Stack(pub Vec<Value>);

impl Stack {
  #[inline(always)]
  pub fn new(capacity: usize) -> Self {
    Self(Vec::with_capacity(capacity))
  }

  #[inline(always)]
  pub fn push(&mut self, value: Value) {
    self.0.push(value);
  }

  #[inline(always)]
  pub fn pop(&mut self) -> Result<Value> {
    self.0.pop().ok_or(Error::StackUnderflow)
  }

  #[inline(always)]
  pub fn pop_unchecked(&mut self) -> Value {
    self.0.pop().unwrap()
  }

  #[inline(always)]
  pub fn check_underflow(&self, len: usize) -> Result<()> {
    if self.0.len() < len {
      Err(Error::StackUnderflow)
    } else {
      Ok(())
    }
  }

  #[inline(always)]
  pub fn dup(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value = self.pop_unchecked();
    self.push(value);
    self.push(value);
    Ok(())
  }

  #[inline(always)]
  pub fn iconst_0(&mut self) {
    self.push(Value::mk_integer(0));
  }

  #[inline(always)]
  pub fn iconst_1(&mut self) {
    self.push(Value::mk_integer(1));
  }

  #[inline(always)]
  pub fn fconst_0(&mut self) {
    self.push(Value::mk_float(0.));
  }

  #[inline(always)]
  pub fn fconst_1(&mut self) {
    self.push(Value::mk_float(1.));
  }

  #[inline(always)]
  pub fn push_byte(&mut self, byte: u8) {
    self.push(Value::mk_integer(byte as Int32));
  }

  #[inline(always)]
  pub fn push_short(&mut self, short: u16) {
    self.push(Value::mk_integer(short as Int32));
  }

  #[inline(always)]
  pub fn iadd(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 + value2));
    Ok(())
  }

  #[inline(always)]
  pub fn isub(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 - value2));
    Ok(())
  }

  #[inline(always)]
  pub fn imul(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 * value2));
    Ok(())
  }

  #[inline(always)]
  pub fn idiv(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 / value2));
    Ok(())
  }

  #[inline(always)]
  pub fn irem(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 % value2));
    Ok(())
  }

  #[inline(always)]
  pub fn iand(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 & value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ior(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 | value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ixor(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 ^ value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ishl(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 << value2));
    Ok(())
  }

  #[inline(always)]
  pub fn ishr(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value1 >> value2));
    Ok(())
  }

  #[inline(always)]
  pub fn iushr(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let rhs = Int32::from(self.pop_unchecked()) as u32;
    let lhs = Int32::from(self.pop_unchecked()) as u32;
    self.push(Value::mk_integer((lhs >> rhs) as i32));
    Ok(())
  }

  #[inline(always)]
  pub fn ineg(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value.wrapping_neg()));
    Ok(())
  }

  #[inline(always)]
  pub fn i2f(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Int32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value as f32));
    Ok(())
  }

  #[inline(always)]
  pub fn f2i(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_integer(value as Int32));
    Ok(())
  }

  #[inline(always)]
  pub fn iexp(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let mut value2: Int32 = self.pop_unchecked().into();
    let mut value1: Int32 = self.pop_unchecked().into();
    let mut result: Int32 = 1;
    while value2 != 0 {
      if value2 & 1 == 1 {
        result *= value1;
      }
      value2 >>= 1;
      value1 *= value1;
    }
    self.push(Value::mk_integer(result));
    Ok(())
  }

  #[inline(always)]
  pub fn is_zero(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value = self.pop_unchecked();
    match value.tag() {
      Value::TAG_BYTE => self.0.push(Value::mk_byte(if value.byte() == 0 { 1 } else { 0 })),
      Value::TAG_INTEGER => {
        self.0.push(Value::mk_integer(if value.integer() == 0 { 1 } else { 0 }))
      }
      Value::TAG_FLOAT => self.0.push(Value::mk_float(if value.float() == 0. { 1. } else { 0. })),
      Value::TAG_REFERENCE => panic!("Invalid argument"),
      _ => unreachable!(),
    }
    Ok(())
  }

  #[inline(always)]
  pub fn ifeq(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 == value2)
  }

  #[inline(always)]
  pub fn ifneq(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 != value2)
  }

  #[inline(always)]
  pub fn ifgt(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 > value2)
  }

  #[inline(always)]
  pub fn ifge(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 >= value2)
  }

  #[inline(always)]
  pub fn iflt(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 < value2)
  }

  #[inline(always)]
  pub fn ifle(&mut self) -> Result<bool> {
    self.check_underflow(2)?;
    let value2: Int32 = self.pop_unchecked().into();
    let value1: Int32 = self.pop_unchecked().into();
    Ok(value1 <= value2)
  }

  #[inline(always)]
  pub fn fadd(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Float32 = self.pop_unchecked().into();
    let value1: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value1 + value2));
    Ok(())
  }

  #[inline(always)]
  pub fn fsub(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Float32 = self.pop_unchecked().into();
    let value1: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value1 - value2));
    Ok(())
  }

  #[inline(always)]
  pub fn fmul(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Float32 = self.pop_unchecked().into();
    let value1: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value1 * value2));
    Ok(())
  }

  #[inline(always)]
  pub fn fdiv(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Float32 = self.pop_unchecked().into();
    let value1: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value1 / value2));
    Ok(())
  }

  #[inline(always)]
  pub fn frem(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Float32 = self.pop_unchecked().into();
    let value1: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value1 % value2));
    Ok(())
  }

  #[inline(always)]
  pub fn fneg(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Float32 = self.pop_unchecked().into();
    self.push(Value::mk_float(value.neg()));
    Ok(())
  }

  #[inline(always)]
  pub fn badd(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 + value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bsub(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 - value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bmul(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 * value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bdiv(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 / value2));
    Ok(())
  }

  #[inline(always)]
  pub fn brem(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 % value2));
    Ok(())
  }

  #[inline(always)]
  pub fn band(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 & value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bor(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 | value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bxor(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 | value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bshl(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 << value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bshr(&mut self) -> Result<()> {
    self.check_underflow(2)?;
    let value2: Byte8 = self.pop_unchecked().into();
    let value1: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value1 >> value2));
    Ok(())
  }

  #[inline(always)]
  pub fn bneg(&mut self) -> Result<()> {
    self.check_underflow(1)?;
    let value: Byte8 = self.pop_unchecked().into();
    self.push(Value::mk_byte(value.wrapping_neg()));
    Ok(())
  }
}
