pub mod builder;
pub mod read;
pub mod write;

use core::fmt;
use std::rc::Rc;

use crate::{heap::Heap, local::Local, runtime::Result, value::Value};

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
  /// The function name.
  pub name: Rc<str>,
  /// The locals used in the code.
  pub locals: u16,
  /// The function arguments.
  pub arguments: u8,
  /// The function bytecode or native call.
  pub code: Code,
}

pub type NativeRet = Result<Option<Value>>;
pub type NativeFn = fn(&mut Local, &mut Heap) -> NativeRet;

pub enum Code {
  Bytecode(Box<[u8]>),
  Native(NativeFn),
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
  pub fn native(name: &str, args: u8, f: NativeFn) -> Self {
    Self { name: Rc::from(name), locals: args as u16, arguments: args, code: Code::Native(f) }
  }
}
