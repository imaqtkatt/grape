pub mod builder;
pub mod read;
pub mod write;

use crate::{heap::Heap, local::Local, value::Value};
use core::fmt;

/// Bytecode Function representation.
///
/// ```
/// {
///   function_name_length: u16,
///   function_name: str<function_name_length>,
///   locals: u16,
///   arguments: u8,
///   code_length: u16,
///   code: Vec<code_length>,
/// }
/// ```
#[derive(Debug)]
pub struct Function {
  /// The function lookup identifier.
  pub identifier: usize,
  /// The function name.
  pub name: Box<str>,
  /// The locals used in the code.
  pub locals: u16,
  /// The function arguments.
  pub arguments: u8,
  /// The function bytecode or native call.
  pub code: Code,
}

type NativeFn = dyn Fn(&Local, &Heap) -> Option<Value> + 'static + Send + Sync;

pub enum Code {
  Bytecode(Box<[u8]>),
  Native(Box<NativeFn>),
}

impl fmt::Debug for Code {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Code::Native(..) => write!(f, "<native>"),
      Code::Bytecode(code) => write!(f, "{code:?}"),
    }
  }
}

impl Function {
  pub fn native<NativeFnImpl>(name: &str, id: usize, args: u8, f: NativeFnImpl) -> Self
  where
    NativeFnImpl: Fn(&Local, &Heap) -> Option<Value> + 'static + Send + Sync,
  {
    Self {
      identifier: id,
      name: Box::from(name),
      locals: args as u16,
      arguments: args,
      code: Code::Native(Box::new(f)),
    }
  }
}
