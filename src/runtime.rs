pub mod stack_trace;

use core::fmt;
use std::rc::Rc;

use crate::{
  context::Context,
  function::{Code, Function},
  heap::Heap,
  local::Local,
  module::{Module, PoolEntry},
  opcode,
  stack::Stack,
  value::{Int32, Reference, Value},
};

pub struct Runtime {
  ip: usize,
  ctx: Context,
  local: Local,
  module: Rc<Module>,
  function: Rc<Function>,
  heap: Heap,
  stack: Stack,
  call_stack: Vec<Frame>,
}

pub trait RuntimeVisitor {
  fn visit(&self, rt: &Runtime);
}

struct Frame {
  return_address: usize,
  local_frame: usize,
  module: Rc<Module>,
  function: Rc<Function>,
}

impl fmt::Debug for Frame {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.module.name, self.function.name)
  }
}

const STACK_INIT: usize = 0x800;
const MAIN: &str = "main";
const IP_INIT: usize = 0;

impl Runtime {
  fn new(ctx: Context, local: Local, module: Rc<Module>, function: Rc<Function>) -> Self {
    Self {
      ip: IP_INIT,
      ctx,
      local,
      module,
      function,
      heap: Heap::new(),
      stack: Stack::new(STACK_INIT),
      call_stack: Vec::new(),
    }
  }

  pub fn boot(mut ctx: Context) -> Result<Self> {
    let module = ctx.fetch_module(MAIN)?;
    let function = module.fetch_function_with_name(MAIN)?;
    assert!(function.arguments == 0);

    let local = Local::new(function.locals as usize);

    Ok(Runtime::new(ctx, local, module, function))
  }

  fn call(&mut self, module: &str, function: usize) -> Result<()> {
    let module = self.ctx.fetch_module(module)?;
    let to_fetch = module.clone();
    let function = to_fetch.fetch_function_with_identifier(function);

    let frame = self.local.push_frame(function.locals as usize);

    for idx in (0..function.arguments).rev() {
      self.local.store(idx as usize, self.stack.pop()?);
    }

    self.call_stack.push(Frame {
      return_address: std::mem::replace(&mut self.ip, IP_INIT),
      local_frame: frame,
      module: std::mem::replace(&mut self.module, module),
      function: std::mem::replace(&mut self.function, function),
    });

    Ok(())
  }

  pub fn run(&mut self) -> Result<()> {
    loop {
      match self.function.code.clone() {
        Code::Native(native) => {
          if let Some(value) = native(&self.local, &self.heap) {
            self.stack.push(value);
          }
          self.pop_frame();
        }
        Code::Bytecode(ref program) => {
          let instruction = self.fetch(program);

          // println!("{}", opcode::TO_STR[instruction as usize]);
          match instruction {
            opcode::HALT => {
              self.pop_frame();
              break Ok(());
            }
            opcode::RETURN => self.pop_frame(),

            opcode::ICONST_0 => self.stack.iconst_0(),
            opcode::ICONST_1 => self.stack.iconst_1(),

            opcode::LOAD => {
              let index = self.fetch(program) as usize;
              self.stack.push(self.local.load(index));
            }

            opcode::STORE => {
              let index = self.fetch(program) as usize;
              self.local.store(index, self.stack.pop()?);
            }
            opcode::STORE_0 => self.local.store(0, self.stack.pop()?),
            opcode::STORE_1 => self.local.store(1, self.stack.pop()?),
            opcode::STORE_2 => self.local.store(2, self.stack.pop()?),
            opcode::STORE_3 => self.local.store(3, self.stack.pop()?),

            opcode::FCONST_0 => self.stack.fconst_0(),
            opcode::FCONST_1 => self.stack.fconst_1(),

            opcode::LOAD_0 => self.stack.push(self.local.load_0()),
            opcode::LOAD_1 => self.stack.push(self.local.load_1()),
            opcode::LOAD_2 => self.stack.push(self.local.load_2()),
            opcode::LOAD_3 => self.stack.push(self.local.load_3()),

            opcode::I2F => self.stack.i2f()?,
            opcode::F2I => self.stack.f2i()?,

            opcode::GOTO => {
              let indexbyte1 = self.fetch(program) as usize;
              let indexbyte2 = self.fetch(program) as usize;
              self.ip = indexbyte1 << 8 | indexbyte2;
            }

            opcode::CALL => {
              let modulebyte1 = self.fetch(program) as usize;
              let modulebyte2 = self.fetch(program) as usize;
              let functionbyte1 = self.fetch(program) as usize;
              let functionbyte2 = self.fetch(program) as usize;

              let this_module = self.module.clone();
              let entry_index = modulebyte1 << 8 | modulebyte2;
              let PoolEntry::Module(module) = &this_module.constants[entry_index] else {
                return Err(Error::InvalidEntry(entry_index));
              };
              let function = functionbyte1 << 8 | functionbyte2;

              self.call(module, function)?;
            }

            opcode::LOADCONST => {
              let index = self.fetch(program) as usize;
              match self.module.constants[index].clone() {
                PoolEntry::String(s) => self.stack.push(self.heap.new_string(s)),
                PoolEntry::Integer(i) => self.stack.push(Value::Integer(i)),
                PoolEntry::Module(_) => return Err(Error::InvalidEntry(index)),
              }
            }

            opcode::NEW_OBJECT => self.stack.push(self.heap.new_object()),
            opcode::SET_FIELD => {
              let value = self.stack.pop()?;
              let field = self.stack.pop()?;
              let obj_ref: Reference = self.stack.pop()?.into();

              self.heap.set_field(obj_ref, field, value);
            }
            opcode::GET_FIELD => {
              let field = self.stack.pop()?;
              let obj_ref: Reference = self.stack.pop()?.into();
              self.stack.push(self.heap.get_field(obj_ref, field));
            }

            opcode::PUSH_BYTE => {
              let byte = self.fetch(program);
              self.stack.push_byte(byte)
            }
            opcode::PUSH_SHORT => {
              let shortbyte1 = self.fetch(program) as u16;
              let shortbyte2 = self.fetch(program) as u16;
              self.stack.push_short(shortbyte1 << 8 | shortbyte2);
            }

            opcode::POP => std::mem::drop(self.stack.pop()),

            opcode::IFEQ => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 == value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }
            opcode::IFNEQ => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 != value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }
            opcode::IFGT => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 > value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }
            opcode::IFGE => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 >= value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }
            opcode::IFLT => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 < value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }
            opcode::IFLE => {
              let value2: Int32 = self.stack.pop()?.into();
              let value1: Int32 = self.stack.pop()?.into();
              if value1 <= value2 {
                let branchbyte1 = self.fetch(program) as usize;
                let branchbyte2 = self.fetch(program) as usize;
                self.ip = branchbyte1 << 8 | branchbyte2;
              } else {
                self.ip += 2;
              }
            }

            opcode::IADD => self.stack.iadd()?,
            opcode::ISUB => self.stack.isub()?,
            opcode::IMUL => self.stack.imul()?,
            opcode::IDIV => self.stack.idiv()?,
            opcode::IREM => self.stack.irem()?,
            opcode::IAND => self.stack.iand()?,
            opcode::IOR => self.stack.ior()?,
            opcode::IXOR => self.stack.ixor()?,
            opcode::ISHL => self.stack.ishl()?,
            opcode::ISHR => self.stack.ishr()?,
            opcode::IUSHR => self.stack.iushr()?,
            opcode::INEG => self.stack.ineg()?,

            opcode::DUP => self.stack.dup()?,

            opcode::NEW_STRING => unimplemented!(),

            opcode::NEW_ARRAY => {
              let size: Int32 = self.stack.pop()?.into();
              self.stack.push(self.heap.new_array(size));
            }

            opcode::ARRAY_GET => {
              let index: Int32 = self.stack.pop()?.into();
              let array_ref: Reference = self.stack.pop()?.into();

              self.stack.push(self.heap.array_get(array_ref, index));
            }

            opcode::ARRAY_SET => {
              let value = self.stack.pop()?;
              let index: Int32 = self.stack.pop()?.into();
              let array_ref: Reference = self.stack.pop()?.into();

              self.heap.array_set(array_ref, index, value);
            }

            opcode => panic!("Unknown opcode {opcode:X?}"),
          }
        }
      }
    }
  }

  #[inline(always)]
  fn pop_frame(&mut self) {
    if let Some(frame) = self.call_stack.pop() {
      self.ip = frame.return_address;
      self.module = frame.module;
      self.function = frame.function;
      self.local.pop_frame(frame.local_frame);
    }
  }

  #[inline(always)]
  fn fetch(&mut self, program: &[u8]) -> u8 {
    let instruction = program[self.ip];
    self.ip += 1;
    instruction
  }
}

impl Runtime {
  pub fn accept(&self, visitor: impl RuntimeVisitor) {
    visitor.visit(self);
  }
}

#[derive(Debug)]
/// A runtime error.
pub enum Error {
  StackUnderflow,
  ModuleNotFound(String),
  ModuleAlreadyExists(String),
  FunctionNotFound(String),
  InvalidEntry(usize),
  Other(Box<dyn std::error::Error + 'static>),
}

impl Error {
  pub fn other<E: std::error::Error + 'static>(e: E) -> Error {
    Error::Other(Box::new(e))
  }
}

pub type Result<T> = std::result::Result<T, Error>;

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::StackUnderflow => write!(f, "Stack Underflow"),
      Error::ModuleNotFound(name) => write!(f, "Module '{name}' not found."),
      Error::ModuleAlreadyExists(name) => write!(f, "Module '{name}' already exists."),
      Error::FunctionNotFound(name) => write!(f, "Function '{name}' not found."),
      Error::InvalidEntry(index) => write!(f, "Invalid constant pool entry '{index}'."),
      Error::Other(e) => write!(f, "{e}"),
    }
  }
}
