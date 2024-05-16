use core::fmt;

pub type Byte8 = u8;
/// Grape int type.
pub type Int32 = i32;
/// Grape float type.
pub type Float32 = f32;
/// Grape reference type.
pub type Reference = usize;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Value(pub u64);

impl Value {
  pub const TAG_REFERENCE: u64 = 1;
  pub const TAG_BYTE: u64 = 1 << 2;
  pub const TAG_INTEGER: u64 = 1 << 3;
  pub const TAG_FLOAT: u64 = 1 << 4;

  const REFERENCE: u64 = 0x0000_0001_0000_0000;
  const BYTE: u64 = 0x0000_0004_0000_0000;
  const INTEGER: u64 = 0x0000_0008_0000_0000;
  const FLOAT: u64 = 0x0000_0010_0000_0000;

  pub const NULL: Value = Self(Self::REFERENCE);

  #[inline(always)]
  pub const fn tag(&self) -> u64 {
    self.0 >> 32
  }

  #[inline(always)]
  pub const fn raw(&self) -> u32 {
    (self.0 & 0xFFFF_FFFF) as u32
  }

  #[inline(always)]
  pub fn raw_mut(&mut self) -> &mut u64 {
    &mut self.0
  }

  #[inline(always)]
  pub const fn mk_reference(r#ref: usize) -> Self {
    Self(Self::REFERENCE | r#ref as u64)
  }

  #[inline(always)]
  pub const fn mk_byte(byte: u8) -> Self {
    Self(Self::BYTE | byte as u64)
  }

  #[inline(always)]
  pub const fn mk_integer(integer: i32) -> Self {
    Self(Self::INTEGER | integer as u64)
  }

  #[inline(always)]
  pub fn mk_float(float: f32) -> Self {
    Self(Self::FLOAT | float.to_bits() as u64)
  }

  #[inline(always)]
  pub const fn byte(&self) -> u8 {
    (self.0 & 0xF) as u8
  }

  #[inline(always)]
  pub const fn integer(&self) -> i32 {
    (self.0 & 0xFFFF_FFFF) as i32
  }

  #[inline(always)]
  pub unsafe fn integer_mut(&mut self) -> &mut i32 {
    &mut *(&mut self.0 as *mut u64 as *mut i32)
  }

  #[inline(always)]
  pub fn float(&self) -> f32 {
    f32::from_bits((self.0 & 0xFFFF_FFFF) as u32)
  }
}

impl fmt::Debug for Value {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.tag() {
      Self::TAG_REFERENCE => write!(f, "@{:08x}", self.raw()),
      Self::TAG_BYTE => write!(f, "{}", self.byte()),
      Self::TAG_INTEGER => write!(f, "{}", self.integer()),
      Self::TAG_FLOAT => write!(f, "{}", self.float()),
      _ => unreachable!(),
    }
  }
}

impl From<Value> for Byte8 {
  fn from(value: Value) -> Self {
    assert!(value.tag() == Value::TAG_BYTE);
    value.byte()
  }
}

impl From<Value> for Int32 {
  fn from(value: Value) -> Self {
    assert!(value.tag() == Value::TAG_INTEGER);
    value.integer()
  }
}

impl From<Value> for Float32 {
  fn from(value: Value) -> Self {
    assert!(value.tag() == Value::TAG_FLOAT);
    value.float()
  }
}

impl From<Value> for Reference {
  fn from(value: Value) -> Self {
    assert!(value.tag() == Value::TAG_REFERENCE);
    value.raw() as Reference
  }
}
