use crate::{heap::Heap, local::Local, read_bytes::ReadBytes, value::Value};
use core::fmt;
use std::rc::Rc;

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
  pub name: Box<str>,
  /// The locals used in the code.
  pub locals: u16,
  /// The function arguments.
  pub arguments: u8,
  /// The function bytecode or native call.
  pub code: Code,
}

type NativeFn = dyn Fn(&Local, &Heap) -> Option<Value>;

pub enum Code {
  Bytecode(Vec<u8>),
  Native(Rc<NativeFn>),
}

impl Function {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Self> {
    let name = rd.read_box_str()?;
    let locals = rd.read_u16()?;
    let arguments = rd.read_u8()?;

    let code_length = rd.read_u16()?;
    let mut code_buf = vec![0; code_length as usize];
    rd.read_exact(&mut code_buf)?;

    let code = Code::Bytecode(code_buf);

    Ok(Self {
      name,
      locals,
      arguments,
      code,
    })
  }
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
  pub fn native(
    name: &str,
    args: u8,
    f: impl Fn(&Local, &Heap) -> Option<Value> + 'static,
  ) -> Self {
    Self {
      name: Box::from(name),
      locals: args as u16,
      arguments: args,
      code: Code::Native(Rc::new(f)),
    }
  }
}

impl Function {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    let name_len = (self.name.len() as u16).to_be_bytes();
    wr.write_all(&name_len)?;
    wr.write_all(self.name.as_bytes())?;
    wr.write_all(&self.locals.to_be_bytes())?;
    wr.write_all(&self.arguments.to_be_bytes())?;
    if let Code::Bytecode(code) = &self.code {
      let code_length = (code.len() as u16).to_be_bytes();
      wr.write_all(&code_length)?;
      wr.write_all(code)?;
    }
    Ok(())
  }
}
