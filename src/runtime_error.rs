use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum RtError {
  ModuleNotFound(String),
  FunctionNotFound(String),
  Other(Box<dyn Error + 'static>),
}

pub fn other<E: Error + 'static>(e: E) -> RtError {
  RtError::Other(Box::new(e))
}

pub type Result<T> = std::result::Result<T, RtError>;

impl fmt::Display for RtError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      RtError::ModuleNotFound(name) => write!(f, "Module '{name}' not found."),
      RtError::FunctionNotFound(name) => write!(f, "Function '{name}' not found."),
      RtError::Other(e) => write!(f, "{e}"),
    }
  }
}
