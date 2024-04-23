use std::rc::Rc;

use crate::{
  context::Context,
  function::Code,
  heap::Heap,
  local::Local,
  module::{Module, PoolEntry},
  opcode,
  runtime_error::{Result, RtError},
  stack::Stack,
  value::{g_int, g_ref, Value},
};

pub struct Runtime<'ctx> {
  ctx: &'ctx mut Context,
  local: Local,
  module: Rc<Module>,
  heap: &'ctx mut Heap,
  stack: &'ctx mut Stack,
}

const STACK_INIT: usize = 0x800;
const MAIN: &str = "main";

impl<'ctx> Runtime<'ctx> {
  pub fn new(
    ctx: &'ctx mut Context,
    local: Local,
    module: Rc<Module>,
    heap: &'ctx mut Heap,
    stack: &'ctx mut Stack,
  ) -> Self {
    Self { ctx, local, module, heap, stack }
  }

  pub fn boot(ctx: &'ctx mut Context) -> Result<Option<Value>> {
    let module = ctx.fetch_module(MAIN)?;
    let function = module.fetch_function_with_name(MAIN)?;
    assert!(function.arguments == 0);

    let local = Local::new(function.locals as usize);

    let Code::Bytecode(code) = &function.code else { unreachable!() };
    let mut heap = Heap::new();
    let mut stack = Stack::new(STACK_INIT);
    Runtime::new(ctx, local, module.clone(), &mut heap, &mut stack).run(code)
  }

  fn call(&mut self, module: &str, function: usize) -> Result<Option<Value>> {
    let module = self.ctx.fetch_module(module)?;
    let to_fetch = module.clone();
    let function = to_fetch.fetch_function_with_identifier(function);

    let frame = self.local.push_frame(function.locals as usize);

    for idx in (0..function.arguments).rev() {
      self.local.store(idx as usize, self.stack.pop()?);
    }

    let result = match &function.code {
      Code::Bytecode(program) => {
        let old_module = std::mem::replace(&mut self.module, module);
        let ret = self.run(program);
        _ = std::mem::replace(&mut self.module, old_module);
        ret
      }
      Code::Native(native) => Ok(native(&self.local, self.heap)),
    };

    self.local.pop_frame(frame);

    result
  }

  fn run(&mut self, program: &[u8]) -> Result<Option<Value>> {
    let ip = &mut 0;

    loop {
      let instruction = self.fetch(ip, program);

      // println!("{}", opcode::TO_STR[instruction as usize]);
      match instruction {
        opcode::RET => break Ok(None),
        opcode::RETURN => break Ok(Some(self.stack.pop()?)),

        opcode::ICONST_0 => self.stack.iconst_0(),
        opcode::ICONST_1 => self.stack.iconst_1(),

        opcode::LOAD => {
          let index = self.fetch(ip, program) as usize;
          self.stack.push(self.local.load(index));
        }

        opcode::STORE => {
          let index = self.fetch(ip, program) as usize;
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
          let indexbyte1 = self.fetch(ip, program) as usize;
          let indexbyte2 = self.fetch(ip, program) as usize;
          *ip = indexbyte1 << 8 | indexbyte2;
        }

        opcode::CALL => {
          let modulebyte1 = self.fetch(ip, program) as usize;
          let modulebyte2 = self.fetch(ip, program) as usize;
          let functionbyte1 = self.fetch(ip, program) as usize;
          let functionbyte2 = self.fetch(ip, program) as usize;

          let this_module = self.module.clone();
          let entry_index = modulebyte1 << 8 | modulebyte2;
          let PoolEntry::Module(module) = &this_module.constants[entry_index] else {
            return Err(RtError::InvalidEntry(entry_index));
          };
          let function = functionbyte1 << 8 | functionbyte2;

          if let Some(value) = self.call(module, function)? {
            self.stack.push(value);
          }
        }

        opcode::LOADCONST => {
          let index = self.fetch(ip, program) as usize;
          match self.module.constants[index].clone() {
            crate::module::PoolEntry::String(s) => self.stack.push(self.heap.new_string(s)),
            crate::module::PoolEntry::Integer(i) => self.stack.push(Value::Integer(i)),
            crate::module::PoolEntry::Module(_) => return Err(RtError::InvalidEntry(index)),
          }
        }

        opcode::NEW_OBJECT => self.stack.push(self.heap.new_object()),
        opcode::SET_FIELD => {
          let value = self.stack.pop()?;
          let field = self.stack.pop()?;
          let obj_ref: g_ref = self.stack.pop()?.into();

          self.heap.set_field(obj_ref, field, value);
        }
        opcode::GET_FIELD => {
          let field = self.stack.pop()?;
          let obj_ref: g_ref = self.stack.pop()?.into();
          self.stack.push(self.heap.get_field(obj_ref, field));
        }

        opcode::PUSH_BYTE => self.stack.push_byte(self.fetch(ip, program)),
        opcode::PUSH_SHORT => {
          let shortbyte1 = self.fetch(ip, program) as u16;
          let shortbyte2 = self.fetch(ip, program) as u16;
          self.stack.push_short(shortbyte1 << 8 | shortbyte2);
        }

        opcode::POP => std::mem::drop(self.stack.pop()),

        opcode::IFEQ => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 == value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFNEQ => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 != value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFGT => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 > value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFGE => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 >= value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFLT => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 < value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFLE => {
          let value2: g_int = self.stack.pop()?.into();
          let value1: g_int = self.stack.pop()?.into();
          if value1 <= value2 {
            let branchbyte1 = self.fetch(ip, program) as usize;
            let branchbyte2 = self.fetch(ip, program) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
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
          let size: g_int = self.stack.pop()?.into();
          self.stack.push(self.heap.new_array(size));
        }

        opcode::ARRAY_GET => {
          let index: g_int = self.stack.pop()?.into();
          let array_ref: g_ref = self.stack.pop()?.into();

          self.stack.push(self.heap.array_get(array_ref, index));
        }

        opcode::ARRAY_SET => {
          let value = self.stack.pop()?;
          let index: g_int = self.stack.pop()?.into();
          let array_ref: g_ref = self.stack.pop()?.into();

          self.heap.array_set(array_ref, index, value);
        }

        opcode => panic!("Unknown opcode {opcode:X?}"),
      }
    }
  }

  #[inline(always)]
  fn fetch(&self, ip: &mut usize, program: &[u8]) -> u8 {
    let instruction = program[*ip];
    *ip += 1;
    instruction
  }
}
