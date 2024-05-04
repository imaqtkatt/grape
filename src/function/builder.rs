use std::rc::Rc;

use crate::{heap::Heap, local::Local, value::Value};

use super::{Code, Function};

#[derive(Default)]
pub struct FunctionBuilder {
  identifier: usize,
  name: Box<str>,
  locals: u16,
  arguments: u8,
  code: Option<Code>,
}

impl FunctionBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_name_and_identifier(mut self, name: &str, identifier: usize) -> Self {
    self.name = Box::from(name);
    self.identifier = identifier;
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
    self.code = Some(Code::Bytecode(Rc::new(bytecode.to_vec())));
    self
  }

  pub fn with_native<NativeFnImpl>(mut self, native: NativeFnImpl) -> Self
  where
    NativeFnImpl: Fn(&Local, &Heap) -> Option<Value> + 'static,
  {
    self.code = Some(Code::Native(Rc::new(native)));
    self
  }

  pub fn build(self) -> Function {
    assert!(self.code.is_some());

    Function {
      identifier: self.identifier,
      name: self.name,
      locals: self.locals,
      arguments: self.arguments,
      code: self.code.unwrap(),
    }
  }
}
