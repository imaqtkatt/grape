use function::builder::FunctionBuilder;
use module::builder::ModuleBuilder;

use crate::{
  context::Context,
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
fn main() -> Result<()> {
  let _main = ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("oioiiooiiioioioiiiooiio".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(FunctionBuilder::new()
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
        PUSH_BYTE, 35,
        CALL, 0, 0, 0, 2, // fib
        CALL, 0, 2, 0, 0, // std:out:print
        HALT])
      .build())
    .with_function(FunctionBuilder::new()
      .with_name_and_identifier("snd", 1)
      .with_locals(2)
      .with_arguments(2)
      .with_bytecode(&[LOAD_1, RETURN])
      .build())
    .with_function(FunctionBuilder::new()
      .with_name_and_identifier("fib", 2)
      .with_locals(1)
      .with_arguments(1)
      .with_bytecode(&[
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
        RETURN])
      .build())
    .build();

  // let mut f = std::fs::File::options().append(true).create(true).open("main.grape").unwrap();
  // main.write(&mut f).unwrap();

  let mut ctx = Context::new();
  ctx.add_module(module::std_out::module()).expect("Add std:out module");
  // ctx.add_module(main).expect("Add main module");

  let mut runtime = Runtime::boot(ctx)?;
  if let Err(e) = runtime.run() {
    eprintln!("Error: {e}");
    runtime.accept(runtime::stack_trace::StackTrace);
  }

  Ok(())
}
