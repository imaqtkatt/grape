pub mod builder;
pub mod std_out;

use crate::function::Function;
use crate::read_bytes::ReadBytes;
use crate::runtime_error;

/// Bytecode Module representation.
///
/// ```
/// {
///   magic_number: u32,
///   module_name_length: u16,
///   module_name: str<module_name_length>,
///   names_count: u16,
///   names: Vec<str, names_count>,
///   pool_count: u16,
///   constants: Vec<PoolEntry, pool_count>,
///   functions_count: u16,
///   functions: Vec<Function, functions_count>,
/// }
/// ```
#[derive(Debug)]
pub struct Module {
  /// The module name.
  pub name: Box<str>,
  /// Known module and function names through the module.
  pub names: Vec<String>,
  /// The constant pool.
  pub constants: Vec<PoolEntry>,
  /// The module functions.
  pub functions: Vec<Function>,
}

#[derive(Clone, Debug)]
pub enum PoolEntry {
  String(String),
  Integer(i32),
}

pub const TAG_STRING: u8 = 0x1;
pub const TAG_INTEGER: u8 = 0x2;

const UVAS: u32 = 1970692467;

impl Module {
  pub fn read<R: std::io::Read>(rd: &mut R) -> std::io::Result<Module> {
    let magic = rd.read_u32()?;

    if magic != 1970692467 {
      return Err(std::io::Error::other("Is not a grape file"));
    }

    let name = rd.read_box_str()?;

    let names_count = rd.read_u16()?;
    let mut names = Vec::with_capacity(names_count as usize);
    for _ in 0..names_count {
      let name_length = rd.read_u16()?;
      let mut name_buf = vec![0; name_length as usize];
      rd.read_exact(&mut name_buf)?;
      names.push(String::from_utf8(name_buf).map_err(std::io::Error::other)?)
    }

    let pool_count = rd.read_u16()?;
    let mut constants = Vec::with_capacity(pool_count as usize);
    for _ in 0..pool_count {
      let tag = rd.read_u8()?;
      match tag {
        TAG_STRING => constants.push(PoolEntry::String(rd.read_string()?)),
        TAG_INTEGER => constants.push(PoolEntry::Integer(rd.read_u32()? as i32)),
        _ => unreachable!(),
      }
    }

    let functions_count = rd.read_u16()?;
    let mut functions = Vec::with_capacity(functions_count as usize);
    for id in 0..functions_count {
      let mut function = Function::read(rd)?;
      function.identifier = id as usize;
      functions.push(function);
    }

    Ok(Self { name, names, constants, functions })
  }
}

impl Module {
  pub fn fetch_function_with_name(&self, name: &str) -> runtime_error::Result<&Function> {
    self
      .functions
      .iter()
      .find(|f| f.name.as_ref() == name)
      .ok_or(runtime_error::RtError::FunctionNotFound(name.to_string()))
  }

  pub fn fetch_function_with_identifier(&self, identifier: usize) -> &Function {
    &self.functions[identifier]
  }
}

impl Module {
  pub fn write<W: std::io::Write>(&self, wr: &mut W) -> std::io::Result<()> {
    wr.write_all(&UVAS.to_be_bytes())?;
    let name_len = (self.name.len() as u16).to_be_bytes();
    wr.write_all(&name_len)?;
    wr.write_all(self.name.as_bytes())?;
    let names_count = (self.names.len() as u16).to_be_bytes();
    wr.write_all(&names_count)?;
    for name in self.names.iter() {
      let name_length = (name.len() as u16).to_be_bytes();
      wr.write_all(&name_length)?;
      wr.write_all(name.as_bytes())?;
    }

    let pool_count = (self.constants.len() as u16).to_be_bytes();
    wr.write_all(&pool_count)?;

    for element in self.constants.iter() {
      match element {
        PoolEntry::String(s) => {
          wr.write_all(&TAG_STRING.to_be_bytes())?;
          wr.write_all(&(s.len() as u16).to_be_bytes())?;
          wr.write_all(s.as_bytes())?;
        }
        PoolEntry::Integer(i) => {
          wr.write_all(&TAG_INTEGER.to_be_bytes())?;
          wr.write_all(&i.to_be_bytes())?;
        }
      }
    }

    let functions_count = (self.functions.len() as u16).to_be_bytes();
    wr.write_all(&functions_count)?;

    for function in self.functions.iter() {
      function.write(wr)?;
    }

    Ok(())
  }
}
