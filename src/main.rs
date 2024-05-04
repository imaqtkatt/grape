//use std::fs::File;

use std::rc::Rc;

use module::builder::ModuleBuilder;

use crate::{
  context::Context,
  function::{Code, Function},
  module::PoolEntry,
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
  let _main = ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("oioiiooiiioioioiiiooiio".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(Function {
      // proc main() {
      //   x0 = [2]
      //   x0[1] = 0
      //   std:out:print(x0)
      //
      //   x1 = "oioiiooiiioioioiiiooiio"
      //   std:out:debug(x1)
      //   std:out:print(x1)
      //
      //   std:out:print(fib(35))
      // }
      identifier: 0,
      name: Box::from("main"),
      locals: 2,
      arguments: 0,
      code: Code::Bytecode(Rc::new(vec![
        PUSH_BYTE, 2,
        NEW_ARRAY,
        STORE_0,
        LOAD_0,
        ICONST_1,
        ICONST_0,
        ARRAY_SET,
        LOAD_0,
        CALL, 0, 2, 0, 0, // std:out:print
        LOADCONST, 1,
        STORE_1,
        LOAD_1,
        CALL, 0, 2, 0, 2, // std:out:debug
        LOAD_1,
        CALL, 0, 2, 0, 0, // std:out:print
        PUSH_BYTE, 35,
        CALL, 0, 0, 0, 2, // fib
        CALL, 0, 2, 0, 0, // std:out:print
        RET,
      ]))
    })
    .with_function(Function {
      identifier: 1,
      name: Box::from("snd"),
      locals: 2,
      arguments: 2,
      code: Code::Bytecode(Rc::new(vec![LOAD_1, RETURN])),
    })
    .with_function(Function {
      identifier: 2,
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
      code: Code::Bytecode(Rc::new(vec![
        LOAD_0,            // 0
        PUSH_BYTE, 2,      // 1
        IFLT, 0, 26,
        LOAD_0,
        PUSH_BYTE, 1,
        ISUB,
        CALL, 0, 0, 0, 2,  // main:fib(x0 - 1)
        LOAD_0,
        PUSH_BYTE, 2,
        ISUB,
        CALL, 0, 0, 0, 2,  // main:fib(x0 - 2)
        IADD,
        RETURN,
        //
        LOAD_0,
        RETURN,
      ])),
    })
    .build();

  // let mut f = std::fs::File::options().append(true).create(true).open("main.grape").unwrap();
  // main.write(&mut f).unwrap();

  let mut ctx = Context::new();
  ctx.add_module(module::std_out::module()).expect("Add std:out module");
  //ctx.add_module(main).expect("Add main module");

  match Runtime::boot(ctx) {
    Ok(_) => {},
    Err(e) => eprintln!("Error: {e}"),
  }
}
