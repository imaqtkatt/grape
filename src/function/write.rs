use super::{Code, Function};
use crate::write_bytes::WriteBytes;

impl Function {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_str(&self.name)?;
    wr.write_u16(self.locals)?;
    wr.write_u8(self.arguments)?;
    if let Code::Bytecode(code) = &self.code {
      wr.write_u16(code.len() as u16)?;
      wr.write_all(code)?;
    } else {
      panic!("Cannot write native function")
    }
    Ok(())
  }
}
