use context::{Context, ContextArena};
use function::builder::FunctionBuilder;
use module::builder::ModuleBuilder;

use crate::{
  module::PoolEntry,
  opcode::*,
  runtime::{Result, Runtime},
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
pub mod stack;
pub mod value;
pub mod write_bytes;

#[rustfmt::skip]
fn run() -> Result<()> {
  let _main = ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("oioiiooiiioioioiiiooiio".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::String("rec fib(35):".to_string()))
    .with_constant(PoolEntry::String("iter fib(35):".to_string()))
    .with_function(
      // proc main() {
      //   let arr = [!2]
      //   arr[1] = 0;
      //   std:out:print(arr);
      //
      //   let s = "oioiiooiiioioioiiiooiio";
      //   std:out:debug(s);
      //   std:out:print(s);
      //
      //   std:out:print("rec fib(35):");
      //   std:out:print(fib(35));
      //   std:out:print("iter fib(35):");
      //   std:out:print(fib2(35));
      // }
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_locals(2)
        .with_arguments(0)
        .with_bytecode(&[
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
          LOADCONST, 3,     // "rec fib(35):"
          CALL, 0, 2, 0, 0, // std:out:print
          PUSH_BYTE, 35,
          CALL, 0, 0, 0, 2, // fib
          CALL, 0, 2, 0, 0, // std:out:print
          LOADCONST, 4,     // "iter fib(35):"
          CALL, 0, 2, 0, 0, // std:out:print
          PUSH_BYTE, 35,
          CALL, 0, 0, 0, 3, // fib2
          CALL, 0, 2, 0, 0, // std:out:print
          HALT,
        ])
        .build(),
    )
    .with_function(
      // func snd(_, a) {
      //   a
      // }
      FunctionBuilder::new()
        .with_name_and_identifier("snd", 1)
        .with_locals(2)
        .with_arguments(2)
        .with_bytecode(&[LOAD_1, RETURN])
        .build(),
    )
    .with_function(
      // func fib(n) {
      //   if n < 2 {
      //     n
      //   } else {
      //     fib(n - 1) + fib(n - 2)
      //   }
      // }
      FunctionBuilder::new()
        .with_name_and_identifier("fib", 2)
        .with_locals(1)
        .with_arguments(1)
        .with_bytecode(&[
          LOAD_0,
          PUSH_BYTE, 2,
          IFLT, 0, 26,
          LOAD_0,
          PUSH_BYTE, 1,
          ISUB,
          CALL, 0, 0, 0, 2, // main:fib(n - 1)
          LOAD_0,
          PUSH_BYTE, 2,
          ISUB,
          CALL, 0, 0, 0, 2, // main:fib(n - 2)
          IADD,
          RETURN,
          //
          LOAD_0,
          RETURN,
        ])
        .build(),
    )
    .with_function(
      // func fib2(n) {
      //   let x = 0;
      //   let y = 1;
      //   let ret = 1;
      //   for (i = 0; i < n; i++) {
      //     x = y;
      //     y = ret;
      //     ret = x + y;
      //   }
      //   ret
      // }
      FunctionBuilder::new()
        .with_arguments(1)
        .with_locals(5)
        .with_name_and_identifier("fib2", 3)
        .with_bytecode(&[
          // x = 0
          ICONST_0,
          STORE_1,
          // y = 1
          ICONST_1,
          STORE_2,
          // z = 1
          ICONST_1,
          STORE_3,
          // for (i = 0; i < n; i++)
          ICONST_0,
          STORE, 4,
          LOAD, 4,
          LOAD_0,
          IFGE, 0, 29, // jump return
          LOAD_2,
          STORE_1,
          LOAD_3,
          STORE_2,
          LOAD_1,
          LOAD_2,
          IADD,
          STORE_3,
          IINC, 4, 1, // i++
          GOTO, 0, 9,
          // return ret
          LOAD_1,
          RETURN,
        ])
        .build()
    )
    .build();

  // let mut f = std::fs::File::options().append(true).create(true).open("main.grape").unwrap();
  // main.write(&mut f).unwrap();

  let ctx_arena = ContextArena::default();
  let ctx = &mut Context::new(&ctx_arena);
  ctx.add_module(module::std_out::module())?;
  ctx.load_eager("main")?;
  // ctx.add_module(main).expect("Add main module");

  let mut runtime = Runtime::boot(ctx)?;
  if let Err(e) = runtime.run() {
    eprintln!("Error: {e}");
    runtime.accept(runtime::stack_trace::StackTrace);
  }

  Ok(())
}

fn main() {
  if let Err(e) = run() {
    eprintln!("{e}");
  }
}
