use super::{Code, Function};
use crate::read_bytes::ReadBytes;

impl Function {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Self> {
    let name = rd.read_rc_str()?;
    let locals = rd.read_u16()?;
    let arguments = rd.read_u8()?;

    let code_length = rd.read_u16()?;
    let mut code_buf = vec![0; code_length as usize];
    rd.read_exact(&mut code_buf)?;

    let code = Code::Bytecode(Box::from(code_buf));

    Ok(Self { name, locals, arguments, code })
  }
}
