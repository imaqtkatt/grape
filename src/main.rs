use crate::{
  context::Context,
  function::{Code, Function},
  module::Module,
  opcode::*,
  runtime::Runtime,
};

pub mod context;
pub mod function;
pub mod local;
pub mod module;
pub mod opcode;
pub mod read_bytes;
pub mod runtime;
pub mod stack;
pub mod value;

fn std_out_print(local: &local::Local) -> Option<value::Value> {
  println!("{}", local.load_0());
  None
}

#[rustfmt::skip]
fn main() {
  let std_out = Module {
    name: Box::from("std:out"),
    names: vec![String::from("std:out"), String::from("print")],
    constants: Vec::new(),
    functions: vec![
      Function::native("print", 1, std_out_print)
    ],
  };

  let main = Module {
    name: Box::from("main"),
    names: vec![
      String::from("main"),
      String::from("snd"),
      String::from("std:out"),
      String::from("print"),
      String::from("fib"),
    ],
    constants: Vec::new(),
    functions: vec![
      // proc main() {
      //   std:out:print(fib(35))
      // }
      Function {
        name: Box::from("main"),
        locals: 1,
        arguments: 0,
        code: Code::Bytecode(vec![
          PUSH_BYTE, 35,
          CALL, 0, 0, 0, 4,
          CALL, 0, 2, 0, 3,
          RET,
        ]),
      },
      // func snd(_, x) {
      //   x
      // }
      Function {
        name: Box::from("snd"),
        locals: 2,
        arguments: 2,
        code: Code::Bytecode(vec![
          LOAD_1,
          RETURN
        ]),
      },
      Function {
        name: Box::from("fib"),
        locals: 1,
        arguments: 1,
        // func fib(x) {
        //   if x < 2 {
        //     x
        //   } else {
        //     fib(x - 1) + fib(x - 2)
        //   }
        // }
        code: Code::Bytecode(vec![
          LOAD_0,            // 0
          PUSH_BYTE, 2,      // 1
          IFLT, 0, 26,
          LOAD_0,
          PUSH_BYTE, 1,
          ISUB,
          CALL, 0, 0, 0, 4,  // fib(x1 - 1)
          LOAD_0,
          PUSH_BYTE, 2,
          ISUB,
          CALL, 0, 0, 0, 4,  // fib(x2 - 2)
          IADD,
          RETURN,
          //
          LOAD_0,
          RETURN,
        ]),
      }
    ],
  };

  let mut ctx = Context::new();
  ctx.add_module(main);
  ctx.add_module(std_out);

  let ret = Runtime::boot(&ctx);
  println!("ret = {ret:?}")
}
