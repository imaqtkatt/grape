use std::rc::Rc;

use crate::{
  context::Context,
  function::Code,
  heap::Heap,
  local::Local,
  module::Module,
  opcode,
  runtime_error::Result,
  stack::Stack,
  value::{g_int, g_ref, Value},
};

pub struct Runtime<'ctx> {
  ctx: &'ctx mut Context,
  program: &'ctx [u8],
  local: Local,
  module: Rc<Module>,
  heap: &'ctx mut Heap,
}

const STACK_INIT: usize = 1 << 10;
const MAIN: &str = "main";

impl<'ctx> Runtime<'ctx> {
  pub fn new(
    ctx: &'ctx mut Context,
    program: &'ctx [u8],
    local: Local,
    module: Rc<Module>,
    heap: &'ctx mut Heap,
  ) -> Self {
    Self {
      ctx,
      program,
      local,
      module,
      heap,
    }
  }

  pub fn boot(ctx: &'ctx mut Context) -> Result<Option<Value>> {
    let module = ctx.fetch_module(MAIN)?;
    let function = module.fetch_function(MAIN)?;
    assert!(function.arguments == 0);

    let local = Local::new(function.locals as usize);

    let Code::Bytecode(code) = &function.code else {
      unreachable!();
    };
    Runtime::new(ctx, code, local, module.clone(), &mut Heap::new()).run(Stack::new(STACK_INIT))
  }

  fn call(&mut self, module: &str, function: &str, stack: &mut Stack) -> Result<Option<Value>> {
    let module = self.ctx.fetch_module(module)?;
    let function = module.fetch_function(function)?;

    let mut local = Local::new(function.locals as usize);

    for idx in (0..function.arguments).rev() {
      local.store(idx as usize, stack.pop());
    }

    match &function.code {
      Code::Bytecode(program) => {
        let mut rt = Runtime::new(self.ctx, program, local, module.clone(), self.heap);
        rt.run(Stack::new(STACK_INIT))
      }
      Code::Native(native) => Ok(native(&local, self.heap)),
    }
  }

  fn run(&mut self, mut stack: Stack) -> Result<Option<Value>> {
    let ip = &mut 0;

    loop {
      let instruction = self.fetch(ip);

      // println!("{}", opcode::TO_STR[instruction as usize]);
      match instruction {
        opcode::RET => break Ok(None),
        opcode::RETURN => break Ok(Some(stack.pop())),

        opcode::ICONST_0 => stack.iconst_0(),
        opcode::ICONST_1 => stack.iconst_1(),

        opcode::LOAD => {
          let index = self.fetch(ip) as usize;
          stack.push(self.local.load(index));
        }

        opcode::STORE => {
          let index = self.fetch(ip) as usize;
          self.local.store(index, stack.pop());
        }
        opcode::STORE_0 => self.local.store(0, stack.pop()),
        opcode::STORE_1 => self.local.store(1, stack.pop()),
        opcode::STORE_2 => self.local.store(2, stack.pop()),
        opcode::STORE_3 => self.local.store(3, stack.pop()),

        opcode::FCONST_0 => stack.fconst_0(),
        opcode::FCONST_1 => stack.fconst_1(),

        opcode::LOAD_0 => stack.push(self.local.load_0()),
        opcode::LOAD_1 => stack.push(self.local.load_1()),
        opcode::LOAD_2 => stack.push(self.local.load_2()),
        opcode::LOAD_3 => stack.push(self.local.load_3()),

        opcode::I2F => stack.i2f(),
        opcode::F2I => stack.f2i(),

        opcode::GOTO => {
          let indexbyte1 = self.fetch(ip) as usize;
          let indexbyte2 = self.fetch(ip) as usize;
          *ip = indexbyte1 << 8 | indexbyte2;
        }

        opcode::CALL => {
          let modulebyte1 = self.fetch(ip) as usize;
          let modulebyte2 = self.fetch(ip) as usize;
          let functionbyte1 = self.fetch(ip) as usize;
          let functionbyte2 = self.fetch(ip) as usize;

          let this_module = self.module.clone();
          let module = &this_module.names[modulebyte1 << 8 | modulebyte2];
          let function = &this_module.names[functionbyte1 << 8 | functionbyte2];

          if let Some(value) = self.call(module, function, &mut stack)? {
            stack.push(value);
          }
        }

        opcode::LOADCONST => {
          let index = self.fetch(ip) as usize;
          match self.module.constants[index].clone() {
            crate::module::PoolEntry::String(s) => stack.push(self.heap.new_string(s)),
            crate::module::PoolEntry::Integer(i) => stack.push(Value::Integer(i)),
          }
        }

        opcode::NEW_OBJECT => stack.push(self.heap.new_object()),
        opcode::SET_FIELD => {
          let value = stack.pop();
          let field = stack.pop();
          let obj_ref: g_ref = stack.pop().into();

          self.heap.set_field(obj_ref, field, value);
        }
        opcode::GET_FIELD => {
          let field = stack.pop();
          let obj_ref: g_ref = stack.pop().into();
          stack.push(self.heap.get_field(obj_ref, field));
        }

        opcode::PUSH_BYTE => stack.push_byte(self.fetch(ip)),
        opcode::PUSH_SHORT => {
          let shortbyte1 = self.fetch(ip) as u16;
          let shortbyte2 = self.fetch(ip) as u16;
          stack.push_short(shortbyte1 << 8 | shortbyte2);
        }

        opcode::POP => std::mem::drop(stack.pop()),

        opcode::IFEQ => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 == value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFNEQ => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 != value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFGT => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 > value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFGE => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 >= value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFLT => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 < value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }
        opcode::IFLE => {
          let value2: g_int = stack.pop().into();
          let value1: g_int = stack.pop().into();
          if value1 <= value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        }

        opcode::IADD => stack.iadd(),
        opcode::ISUB => stack.isub(),
        opcode::IMUL => stack.imul(),
        opcode::IDIV => stack.idiv(),
        opcode::IREM => stack.irem(),
        opcode::IAND => stack.iand(),
        opcode::IOR => stack.ior(),
        opcode::IXOR => stack.ixor(),
        opcode::ISHL => stack.ishl(),
        opcode::ISHR => stack.ishr(),
        opcode::IUSHR => stack.iushr(),
        opcode::INEG => stack.ineg(),

        opcode::DUP => stack.dup(),

        opcode::NEW_STRING => unimplemented!(),

        opcode::NEW_ARRAY => {
          let size: g_int = stack.pop().into();
          stack.push(self.heap.new_array(size));
        }

        opcode::ARRAY_GET => {
          let index: g_int = stack.pop().into();
          let array_ref: g_ref = stack.pop().into();

          stack.push(self.heap.array_get(array_ref, index));
        }

        opcode::ARRAY_SET => {
          let value = stack.pop();
          let index: g_int = stack.pop().into();
          let array_ref: g_ref = stack.pop().into();

          self.heap.array_set(array_ref, index, value);
        }

        opcode => panic!("Unknown opcode {opcode:X?}"),
      }
    }
  }

  #[inline(always)]
  fn fetch(&self, ip: &mut usize) -> u8 {
    let instruction = self.program[*ip];
    *ip += 1;
    instruction
  }
}
