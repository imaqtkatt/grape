pub mod stack_trace;

use core::fmt;
use std::cell::RefCell;

use crate::{
  context::Context,
  function::{Code, Function},
  heap::Heap,
  local::Local,
  module::{Module, PoolEntry},
  opcode,
  stack::Stack,
  value::{Byte8, Int32, Reference, Value},
};

pub struct Runtime<'c> {
  ip: RefCell<usize>,
  ctx: &'c mut Context<'c>,
  local: Local,
  module: &'c Module,
  function: &'c Function,
  heap: Heap,
  stack: Stack,
  call_stack: Vec<Frame<'c>>,
}

pub trait RuntimeVisitor {
  fn visit(&self, rt: &Runtime);
}

struct Frame<'c> {
  return_address: RefCell<usize>,
  local_frame: usize,
  module: &'c Module,
  function: &'c Function,
}

impl fmt::Debug for Frame<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.module.name, self.function.name)
  }
}

const STACK_INIT: usize = 0x800;
const MAIN: &str = "main";
const IP_INIT: usize = 0;

pub struct BootOptions<'c> {
  pub eager: bool,
  pub entrypoint_module: Option<String>,
  pub context: &'c mut Context<'c>,
}

impl<'c> Runtime<'c> {
  fn new(
    ctx: &'c mut Context<'c>,
    local: Local,
    module: &'c Module,
    function: &'c Function,
  ) -> Self {
    Self {
      ip: RefCell::new(IP_INIT),
      ctx,
      local,
      module,
      function,
      heap: Heap::new(),
      stack: Stack::new(STACK_INIT),
      call_stack: Vec::new(),
    }
  }

  pub fn boot(opts: BootOptions<'c>) -> Result<Runtime<'c>> {
    let module: &'c Module;
    if let Some(entrypoint_module) = opts.entrypoint_module {
      module = opts.context.fetch_module(&entrypoint_module)?;
    } else {
      module = opts.context.fetch_module(MAIN)?;
    }
    let function = module.fetch_function_with_name(MAIN)?;
    assert!(function.arguments == 0);
    if opts.eager {
      opts.context.load_eager(&module.name)?;
    }

    let local = Local::new(function.locals as usize);

    Ok(Runtime::new(opts.context, local, module, function))
  }

  fn call(&mut self, module_name: &str, function_index: usize) -> Result<()> {
    let module: &Module;
    let function: &Function;
    if *module_name == *self.module.name {
      module = self.module;
      function = module.fetch_function_with_identifier(function_index);
    } else {
      module = self.ctx.fetch_module(module_name)?;
      function = module.fetch_function_with_identifier(function_index);
    }

    let frame = self.local.push_frame(function.locals as usize);

    self.stack.check_underflow(function.arguments as usize)?;
    for index in (0..function.arguments).rev() {
      self.local.store(index as usize, self.stack.pop_unchecked());
    }

    self.call_stack.push(Frame {
      return_address: std::mem::replace(&mut self.ip, RefCell::new(IP_INIT)),
      local_frame: frame,
      module: std::mem::replace(&mut self.module, module),
      function: std::mem::replace(&mut self.function, function),
    });

    Ok(())
  }

  pub fn run(&mut self) -> Result<()> {
    loop {
      match self.function.code {
        Code::Native(ref native) => {
          if let Some(value) = native(&mut self.local, &mut self.heap) {
            self.stack.push(value);
          }
          self.pop_frame();
        }
        Code::Bytecode(ref program) => {
          let instruction = self.fetch(program);

          // println!("{} with {:?}", opcode::TO_STR[instruction as usize], self.stack);
          match instruction {
            opcode::HALT => break Ok(()),

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

            opcode::GOTO => *self.ip.get_mut() = self.fetch_2(program) as usize,

            opcode::CALL => {
              let indexes = self.fetch_4(program);
              let module_index = indexes >> 16;
              let function_index = indexes & 0xFF;
              if let PoolEntry::Module(module) = &self.module.constants[module_index] {
                self.call(module, function_index)?
              } else {
                Err(Error::InvalidEntry(module_index))?
              }
            }

            opcode::LOADCONST => {
              let entry_index = self.fetch(program) as usize;
              match &self.module.constants[entry_index] {
                PoolEntry::String(s) => self.stack.push(self.heap.new_string(s.clone())),
                PoolEntry::Integer(i) => self.stack.push(Value::mk_integer(*i)),
                PoolEntry::Module(_) => return Err(Error::InvalidEntry(entry_index)),
                PoolEntry::Float(f) => self.stack.push(Value::mk_float(*f)),
              }
            }

            opcode::NEW_OBJECT => self.stack.push(self.heap.new_object()),
            opcode::SET_FIELD => {
              self.stack.check_underflow(3)?;
              let value = self.stack.pop_unchecked();
              let field = self.stack.pop_unchecked();
              let obj_ref: Reference = self.stack.pop_unchecked().into();

              self.heap.set_field(obj_ref, field, value);
            }
            opcode::GET_FIELD => {
              self.stack.check_underflow(2)?;
              let field = self.stack.pop_unchecked();
              let obj_ref: Reference = self.stack.pop_unchecked().into();
              self.stack.push(self.heap.get_field(obj_ref, field));
            }

            opcode::I_PUSH_BYTE => {
              let byte = self.fetch(program);
              self.stack.push_byte(byte)
            }
            opcode::I_PUSH_SHORT => {
              let short = self.fetch_2(program);
              self.stack.push_short(short);
            }

            opcode::POP => std::mem::drop(self.stack.pop()),

            opcode::I_IFEQ => {
              if self.stack.ifeq()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }
            opcode::I_IFNEQ => {
              if self.stack.ifneq()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }
            opcode::I_IFGT => {
              if self.stack.ifgt()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }
            opcode::I_IFGE => {
              if self.stack.ifge()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }
            opcode::I_IFLT => {
              if self.stack.iflt()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }
            opcode::I_IFLE => {
              if self.stack.ifle()? {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
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
              self.stack.check_underflow(1)?;
              let size: Int32 = self.stack.pop_unchecked().into();
              self.stack.push(self.heap.new_array(size));
            }

            opcode::ARRAY_GET => {
              self.stack.check_underflow(2)?;
              let index: Int32 = self.stack.pop_unchecked().into();
              let array_ref: Reference = self.stack.pop_unchecked().into();

              self.stack.push(self.heap.array_get(array_ref, index));
            }

            opcode::ARRAY_SET => {
              self.stack.check_underflow(3)?;
              let value = self.stack.pop_unchecked();
              let index: Int32 = self.stack.pop_unchecked().into();
              let array_ref: Reference = self.stack.pop_unchecked().into();

              self.heap.array_set(array_ref, index, value);
            }

            opcode::IINC => {
              let index = self.fetch(program) as usize;
              let inc = self.fetch(program) as i32;
              self.local.iinc(index, inc);
            }

            opcode::IF_NULL => {
              self.stack.check_underflow(1)?;
              let r#ref: Reference = self.stack.pop_unchecked().into();

              if r#ref == 0 {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }

            opcode::IFNOT_NULL => {
              self.stack.check_underflow(1)?;
              let r#ref: Reference = self.stack.pop_unchecked().into();

              if r#ref != 0 {
                *self.ip.get_mut() = self.fetch_2(program) as usize;
              } else {
                *self.ip.get_mut() += 2;
              }
            }

            opcode::CONST_NULL => self.stack.push(Value::NULL),

            opcode::IEXP => self.stack.iexp()?,

            opcode::IS_ZERO => self.stack.is_zero()?,

            opcode::TAILCALL => {
              self.stack.check_underflow(self.function.arguments as usize)?;
              for index in (0..self.function.arguments).rev() {
                self.local.store(index as usize, self.stack.pop_unchecked());
              }
              *self.ip.get_mut() = IP_INIT;
            }

            opcode::FADD => self.stack.fadd()?,
            opcode::FSUB => self.stack.fsub()?,
            opcode::FMUL => self.stack.fmul()?,
            opcode::FDIV => self.stack.fdiv()?,
            opcode::FREM => self.stack.frem()?,
            opcode::FNEG => self.stack.fneg()?,

            opcode::PUSH_BYTE => {
              let byte = self.fetch(program);
              self.stack.push(Value::mk_byte(byte));
            }

            opcode::BADD => self.stack.badd()?,
            opcode::BSUB => self.stack.bsub()?,
            opcode::BMUL => self.stack.bmul()?,
            opcode::BDIV => self.stack.bdiv()?,
            opcode::BREM => self.stack.brem()?,
            opcode::BAND => self.stack.band()?,
            opcode::BOR => self.stack.bor()?,
            opcode::BXOR => self.stack.bxor()?,
            opcode::BSHL => self.stack.bshl()?,
            opcode::BSHR => self.stack.bshr()?,
            opcode::BNEG => self.stack.bneg()?,

            opcode::NEW_BYTES => {
              let len = self.fetch_2(program) as usize;
              self.stack.check_underflow(len)?;
              let mut bytes_vec = Vec::with_capacity(len);
              let mut len = len;
              while len > 0 {
                len -= 1;
                bytes_vec.insert(0, self.stack.pop_unchecked().into());
              }
              self.stack.push(self.heap.new_bytes(bytes_vec));
            }

            opcode::BYTES_PUSH => {
              self.stack.check_underflow(2)?;
              let byte: Byte8 = self.stack.pop_unchecked().into();
              let bytes_ref: Reference = self.stack.pop_unchecked().into();
              self.heap.bytes_push(bytes_ref, byte);
            }

            opcode => unreachable!("Reached unknown opcode {opcode:X?}"),
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
    let ip = self.ip.get_mut();
    let instruction = program[*ip];
    *ip += 1;
    instruction
  }

  #[inline(always)]
  fn fetch_2(&mut self, program: &[u8]) -> u16 {
    let ip = self.ip.get_mut();
    let instruction = (program[*ip] as u16) << 8 | program[*ip + 1] as u16;
    *ip += 2;
    instruction
  }

  #[inline(always)]
  #[allow(unused)]
  fn fetch_4(&mut self, program: &[u8]) -> usize {
    let ip = self.ip.get_mut();
    let instruction = (program[*ip] as usize) << 24
      | (program[*ip + 1] as usize) << 16
      | (program[*ip + 2] as usize) << 8
      | program[*ip + 3] as usize;
    *ip += 4;
    instruction
  }
}

impl Runtime<'_> {
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
