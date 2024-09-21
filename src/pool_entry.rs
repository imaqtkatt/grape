pub mod read;
pub mod write;

#[derive(Clone, Debug)]
pub enum PoolEntry {
  String(String),
  Integer(i32),
  Module(String),
  Float(f32),
  Function(String),
  Class(String),
  Field(String),
}

impl PoolEntry {
  pub const TAG_STRING: u8 = 0x1;
  pub const TAG_INTEGER: u8 = 0x2;
  pub const TAG_MODULE: u8 = 0x3;
  pub const TAG_FLOAT: u8 = 0x4;
  pub const TAG_FUNCTION: u8 = 0x5;
  pub const TAG_CLASS: u8 = 0x6;
  pub const TAG_FIELD: u8 = 0x7;
}
