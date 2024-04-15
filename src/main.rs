use crate::{
  context::Context,
  function::{Code, Function},
  module::{Module, PoolEntry},
  opcode::*,
  runtime::Runtime,
};

pub mod context;
pub mod formatting;
pub mod function;
pub mod heap;
pub mod local;
pub mod module;
pub mod module_path;
pub mod opcode;
pub mod read_bytes;
pub mod runtime;
pub mod runtime_error;
pub mod stack;
pub mod value;

#[rustfmt::skip]
fn main() {
  let main = Module {
    name: Box::from("main"),
    names: vec![
      String::from("main"),
      String::from("snd"),
      String::from("std:out"),
      String::from("println"),
      String::from("fib"),
      String::from("debug"),
    ],
    constants: vec![
      PoolEntry::String(String::from("oioiiooiiioioioiiiooiio")),
    ],
    functions: vec![
      // proc main() {
      //   x0 = [2]
      //   x0[1] = 0
      //
      //   x1 = "oioiiooiiioioioiiiooiio"
      //   std:out:debug(x1)
      //   std:out:print(x1)
      //
      //   std:out:print(fib(35))
      // }
      Function {
        name: Box::from("main"),
        locals: 2,
        arguments: 0,
        code: Code::Bytecode(vec![
          PUSH_BYTE, 2,
          NEW_ARRAY,
          STORE_0,
          LOAD_0,
          ICONST_1,
          ICONST_0,
          ARRAY_SET,
          LOAD_0,
          CALL, 0, 2, 0, 3,
          LOADCONST, 0,
          STORE_1,
          LOAD_1,
          CALL, 0, 2, 0, 5,
          LOAD_1,
          CALL, 0, 2, 0, 3,
          PUSH_BYTE, 35,
          CALL, 0, 0, 0, 4,
          CALL, 0, 2, 0, 3,
          RET,
        ]),
      },
      // func snd(_, x1) {
      //   x1
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
        // func fib(x0) {
        //   if x < 2 {
        //     x0
        //   } else {
        //     fib(x0 - 1) + fib(x0 - 2)
        //   }
        // }
        code: Code::Bytecode(vec![
          LOAD_0,            // 0
          PUSH_BYTE, 2,      // 1
          IFLT, 0, 26,
          LOAD_0,
          PUSH_BYTE, 1,
          ISUB,
          CALL, 0, 0, 0, 4,  // fib(x0 - 1)
          LOAD_0,
          PUSH_BYTE, 2,
          ISUB,
          CALL, 0, 0, 0, 4,  // fib(x0 - 2)
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
  ctx.add_module(module::std_out::module()).expect("Add std:out module");
  ctx.add_module(main).expect("Add main module");

  match Runtime::boot(&mut ctx) {
    Ok(_) => {},
    Err(e) => eprintln!("Error: {e}"),
  }
}
