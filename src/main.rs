use context::{Context, ContextArena};
use function::builder::FunctionBuilder;
use module::builder::ModuleBuilder;
use runtime::BootOptions;

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
  let matches = clap::Command::new("gvm")
    .about("Grape Virtual Machine")
    .version("0.1.0")
    .arg(
      clap::Arg::new("eager")
        .help("Load modules eagerly before running")
        .required(false)
        .long("eager")
        .action(clap::ArgAction::SetTrue)
    )
    .arg(
      clap::Arg::new("entrypoint")
        .help("Name of the entrypoint module")
        .required(false)
        .long("entrypoint")
        .default_value(None)
    )
    .get_matches();

  // let mut f = std::fs::File::options().append(true).create(true).open("main.grape").unwrap();
  // main.write(&mut f).unwrap();

  // let args = Cli::parse();

  let ctx_arena = ContextArena::default();
  let ctx = &mut Context::new(&ctx_arena);
  {
    ctx.add_module(module::std_out::module())?;
    ctx.add_module(module::file::module())?;
  }
  ctx.add_module(main_sla())?;
  // ctx.add_module(module_test_file())?;
  // ctx.add_module(main_module())?;
  // ctx.add_module(main_tailcall())?;
  // ctx.add_module(main_bytes())?;
  // ctx.add_module(main_float())?;

  let mut runtime = Runtime::boot(BootOptions {
    eager: matches.get_flag("eager"),
    entrypoint_module: matches.get_one("entrypoint").map(|e: &String| e.to_string()),
    context: ctx,
  })?;
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

#[rustfmt::skip]
#[allow(unused)]
fn main_sla() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("Hello, World".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_arguments(0)
        .with_locals(1)
        .with_bytecode(&[
          LOADCONST, 1,
          STORE_0,
          CALL, 0, 0, 0, 1,
          LOAD_0,
          CALL, 0, 2, 0, 0,
          ICONST_0,
          ICONST_0,
          IADD,
          POP,
          LOAD_0,
          CALL, 0, 2, 0, 0,
          HALT,
        ])
        .build()
    )
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("allocs_string", 1)
        .with_arguments(0)
        .with_locals(1)
        .with_bytecode(&[
          LOADCONST, 1,
          STORE_0,
          ICONST_0,
          ICONST_0,
          IADD,
          POP,
          RETURN,
        ])
        .build()
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_float() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::Float(std::f32::consts::PI))
    .with_constant(PoolEntry::Float(5.))
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_locals(1)
        .with_arguments(0)
        .with_bytecode(&[
          LOADCONST, 2,
          LOADCONST, 3,
          FMUL,
          CALL, 0, 1, 0, 0,
          HALT,
        ])
        .build()
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_bytes() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_locals(1)
        .with_arguments(0)
        .with_bytecode(&[
          PUSH_BYTE, 42,
          PUSH_BYTE, 43,
          PUSH_BYTE, 55,
          NEW_BYTES, 0, 3,
          STORE_0,
          LOAD_0,
          PUSH_BYTE, 88,
          BYTES_PUSH,
          LOAD_0,
          CALL, 0, 1, 0, 0,
          HALT,
        ])
        .build()
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_tailcall() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::Integer(15))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::String("----------".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_locals(0)
        .with_arguments(0)
        .with_bytecode(&[
          LOADCONST, 1,
          ICONST_1,
          CALL, 0, 0, 0, 1, // tail_fact(12, 1)
          CALL, 0, 2, 0, 0, // std:out:print
          LOADCONST, 3,
          CALL, 0, 2, 0, 0, // std:out:print
          LOADCONST, 1,
          CALL, 0, 0, 0, 2, // fact(12)
          CALL, 0, 2, 0, 0, // std:out:print
          HALT
        ])
        .build(),
    )
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("tail_fact", 1)
        .with_locals(2)
        .with_arguments(2)
        .with_bytecode(&[
          LOAD_0,
          ICONST_0,
          I_IFEQ, 0, 12,
          LOAD_0,
          ICONST_1,
          ISUB,
          LOAD_1,
          LOAD_0,
          IMUL,
          TAILCALL,
          // return acc
          LOAD_1,
          RETURN,
        ])
        .build(),
    )
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("fact", 2)
        .with_locals(1)
        .with_arguments(1)
        .with_bytecode(&[
          LOAD_0,
          ICONST_0,
          I_IFEQ, 0, 16,
          LOAD_0,
          LOAD_0,
          ICONST_1,
          ISUB,
          CALL, 0, 0, 0, 2,
          IMUL,
          RETURN,
          // return 1
          ICONST_1,
          RETURN,
        ])
        .build(),
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_module() -> module::Module {
  ModuleBuilder::new()
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
          I_PUSH_BYTE, 2,
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
          LOADCONST, 3, // "rec fib(35):"
          CALL, 0, 2, 0, 0, // std:out:print
          I_PUSH_BYTE, 35,
          CALL, 0, 0, 0, 2, // fib
          CALL, 0, 2, 0, 0, // std:out:print
          LOADCONST, 4, // "iter fib(35):"
          CALL, 0, 2, 0, 0, // std:out:print
          I_PUSH_BYTE, 35,
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
          I_PUSH_BYTE, 2,
          I_IFLT, 0, 25,
          LOAD_0,
          ICONST_1,
          ISUB,
          CALL, 0, 0, 0, 2, // main:fib(n - 1)
          LOAD_0,
          I_PUSH_BYTE, 2,
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
          STORE_1, // y = 1
          ICONST_1,
          STORE_2, // z = 1
          ICONST_1,
          STORE_3, // for (i = 0; i < n; i++)
          ICONST_0,
          STORE, 4,
          LOAD, 4,
          LOAD_0,
          I_IFGE, 0, 29, // jump return
          LOAD_2,
          STORE_1,
          LOAD_3,
          STORE_2,
          LOAD_1,
          LOAD_2,
          IADD,
          STORE_3,
          IINC, 4, 1, // i++
          GOTO, 0, 9, // return ret
          LOAD_1,
          RETURN,
        ])
        .build(),
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn module_test_file() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("main.wine".to_string()))
    .with_constant(PoolEntry::Module("file".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name_and_identifier("main", 0)
        .with_arguments(0)
        .with_locals(0)
        .with_bytecode(&[
          LOADCONST, 1,
          CALL, 0, 2, 0, 0, // file:read_to_string
          CALL, 0, 3, 0, 0, // std:out:println
          LOADCONST, 1,
          CALL, 0, 2, 0, 1, // file:read_to_bytes
          CALL, 0, 3, 0, 0, // std:out:println
          HALT,
        ])
        .build()
    )
    .build()
}
