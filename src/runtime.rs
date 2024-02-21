use crate::{
  context::Context, function::Code, local::Local, module::Module, opcode, stack::Stack,
  value::{gint_t, Value},
};

pub struct Runtime<'ctx> {
  ctx: &'ctx Context,
  program: &'ctx [u8],
  local: Local,
  module: &'ctx Module,
}

const STACK_INIT: usize = 1 << 10;
const MAIN: &str = "main";

impl<'ctx> Runtime<'ctx> {
  pub fn new(ctx: &'ctx Context, program: &'ctx [u8], local: Local, module: &'ctx Module) -> Self {
    Self {
      ctx,
      program,
      local,
      module,
    }
  }

  pub fn boot(ctx: &'ctx Context) -> Option<Value> {
    let module = ctx.fetch_module(MAIN)?;
    let function = module.fetch_function(MAIN)?;
    assert!(function.arguments == 0);

    let local = Local::new(function.locals as usize);

    let Code::Bytecode(code) = &function.code else {
      unreachable!();
    };
    let mut rt = Runtime::new(ctx, code, local, module);
    rt.run(Stack::new(STACK_INIT))
  }

  fn call(&mut self, module: &str, function: &str, stack: &mut Stack) -> Option<Value> {
    let Some(module) = self.ctx.fetch_module(module) else {
      panic!("Module '{module}' not found.")
    };
    let Some(function) = module.fetch_function(function) else {
      panic!("Function '{function}' not found.")
    };

    let mut local = Local::new(function.locals as usize);

    for idx in (0..function.arguments).rev() {
      local.store(idx as usize, stack.pop());
    }

    match &function.code {
      Code::Bytecode(program) => {
        let mut rt = Runtime::new(self.ctx, program, local, module);
        rt.run(Stack::new(STACK_INIT))
      }
      Code::Native(native) => native(&local),
    }
  }

  fn run(&mut self, mut stack: Stack) -> Option<Value> {
    let ip = &mut 0;

    loop {
      let instruction = self.fetch(ip);

      // println!("{}", opcode::TO_STR[instruction as usize]);
      match instruction {
        opcode::RET => break None,
        opcode::RETURN => break Some(stack.pop()),
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
        opcode::FCONST_0 => stack.fconst_0(),
        opcode::FCONST_1 => stack.fconst_1(),
        opcode::LOAD_0 => stack.push(self.local.load_0()),
        opcode::LOAD_1 => stack.push(self.local.load_1()),
        opcode::LOAD_2 => stack.push(self.local.load_2()),
        opcode::LOAD_3 => stack.push(self.local.load_3()),
        opcode::I2F => todo!(),
        opcode::F2I => todo!(),
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

          let module = &self.module.names[(modulebyte1 << 8 | modulebyte2) as usize];
          let function = &self.module.names[(functionbyte1 << 8 | functionbyte2) as usize];

          if let Some(value) = self.call(module, function, &mut stack) {
            stack.push(value);
          }
        }
        opcode::LOADCONST => {
          let index = self.fetch(ip) as usize;
          match self.module.constants[index].clone() {
            crate::module::PoolEntry::String(_) => todo!(),
            crate::module::PoolEntry::Integer(i) => stack.push(Value::Integer(i)),
          }
        }
        opcode::NEW_OBJECT => todo!(),
        opcode::SET_FIELD => todo!(),
        opcode::GET_FIELD => todo!(),
        opcode::PUSH_BYTE => stack.push_byte(self.fetch(ip)),
        opcode::PUSH_SHORT => {
          let shortbyte1 = self.fetch(ip) as u16;
          let shortbyte2 = self.fetch(ip) as u16;
          stack.push_short(shortbyte1 << 8 | shortbyte2);
        }
        opcode::POP => _ = stack.pop(),
        opcode::IFEQ => todo!(),
        opcode::IFNEQ => todo!(),
        opcode::IFGT => todo!(),
        opcode::IFGE => todo!(),
        opcode::IFLT => {
          let value2: gint_t = stack.pop().into();
          let value1: gint_t = stack.pop().into();
          if value1 < value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        },
        opcode::IFLE => {
          let value2: gint_t = stack.pop().into();
          let value1: gint_t = stack.pop().into();
          if value1 <= value2 {
            let branchbyte1 = self.fetch(ip) as usize;
            let branchbyte2 = self.fetch(ip) as usize;
            *ip = branchbyte1 << 8 | branchbyte2;
          } else {
            *ip += 2;
          }
        },
        opcode::IADD => stack.iadd(),
        opcode::ISUB => stack.isub(),
        opcode::IMUL => stack.imul(),
        opcode::IDIV => stack.idiv(),
        opcode::IREM => todo!(),
        opcode::IAND => todo!(),
        opcode::IOR => todo!(),
        opcode::IXOR => todo!(),
        opcode::ISHL => todo!(),
        opcode::ISHR => todo!(),
        opcode::IUSHR => todo!(),
        opcode::INEG => todo!(),
        _ => panic!(),
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
