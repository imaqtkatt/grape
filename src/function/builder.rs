use super::{Code, Function, NativeFn};

#[derive(Default)]
pub struct FunctionBuilder {
  name: Box<str>,
  locals: u16,
  arguments: u8,
  code: Option<Code>,
}

impl FunctionBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_name(mut self, name: &str) -> Self {
    self.name = Box::from(name);
    self
  }

  pub fn with_locals(mut self, locals: u16) -> Self {
    self.locals = locals;
    self
  }

  pub fn with_arguments(mut self, arguments: u8) -> Self {
    self.arguments = arguments;
    self
  }

  pub fn with_bytecode(mut self, bytecode: &[u8]) -> Self {
    self.code = Some(Code::Bytecode(Box::from(bytecode)));
    self
  }

  pub fn with_native(mut self, native: NativeFn) -> Self {
    self.code = Some(Code::Native(native));
    self
  }

  pub fn build(self) -> Function {
    assert!(self.code.is_some());

    Function {
      name: self.name.into(),
      locals: self.locals,
      arguments: self.arguments,
      code: self.code.unwrap(),
    }
  }
}
