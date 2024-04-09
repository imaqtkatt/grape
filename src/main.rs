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
pub mod opcode;
pub mod read_bytes;
pub mod runtime;
pub mod stack;
pub mod value;

fn std_out_print(local: &local::Local, heap: &heap::Heap) -> Option<value::Value> {
  println!("{}", formatting::display_value(&local.load_0(), heap));
  None
}

fn std_out_debug(local: &local::Local, _: &heap::Heap) -> Option<value::Value> {
  println!("{:?}", local.load_0());
  None
}

#[rustfmt::skip]
fn main() {
  let std_out = Module {
    name: Box::from("std:out"),
    names: vec![String::from("std:out"), String::from("print")],
    constants: Vec::new(),
    functions: vec![
      Function::native("print", 1, std_out_print),
      Function::native("debug", 1, std_out_debug),
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

  let ret = Runtime::boot(&mut ctx);
  println!("ret = {ret:?}")
}
