#[derive(Clone, Copy, Debug)]
pub enum Value {
  Integer(i32),
  Float(f32),
  Object(usize),
  Array(usize),
  String(usize),
}
