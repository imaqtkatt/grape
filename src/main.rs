use function::builder::FunctionBuilder;
use loader::{Loader, LoaderArena};
use module::builder::ModuleBuilder;
use pool_entry::PoolEntry;
use runtime::BootOptions;

use crate::{
  opcode::*,
  runtime::{Result, Runtime},
};

pub mod class;
pub mod context;
pub mod formatting;
pub mod function;
pub mod heap;
pub mod loader;
pub mod local;
pub mod module;
pub mod module_path;
pub mod opcode;
pub mod pool_entry;
pub mod read_bytes;
pub mod runtime;
pub mod stack;
pub mod value;
pub mod write_bytes;

#[rustfmt::skip]
fn run() -> Result<()> {
  let matches = clap::Command::new("gvm")
    .about("Grape Virtual Machine")
    .version(env!("CARGO_PKG_VERSION"))
    .arg(
      clap::Arg::new("entrypoint")
        .help("Name of the entrypoint module")
        .required(false)
        .long("entrypoint")
        .default_value(None)
    )
    .get_matches();

  // let m = main_module();
  // let mut f = std::fs::File::options().create_new(true).write(true).open("./main.grape").unwrap();
  // m.write(&mut f).unwrap();

  let entrypoint_module = matches.get_one("entrypoint").map(|e: &String| e.to_string());

  let loader_arena = LoaderArena::default();
  let mut loader = Loader::new(&loader_arena);
  if let Some(entrypoint) = &entrypoint_module {
    loader.load_path(entrypoint)?;
  } else {
    loader.load_path("main")?;
  }
  let context = &mut loader.to_context();
  // ctx.add_module(main_class())?;

  let mut runtime = Runtime::boot(BootOptions { entrypoint_module, context })?;
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

// TODO: move this

#[rustfmt::skip]
#[allow(unused)]
fn main_class() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::Class("Box".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::Function("println".to_string()))
    .with_constant(PoolEntry::Field("value".to_string()))
    .with_constant(PoolEntry::Function("show".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name("main")
        .with_arguments(0)
        .with_locals(1)
        .with_bytecode(&[
          I_PUSH_BYTE, 42,
          NEW, 0, 1,
          STORE_0,
          LOAD_0,
          CALL, 0, 2, 0, 3,
          LOAD_0,
          GET_FIELD, 0, 4,
          LOAD_0,
          CALL_METHOD, 0, 1, 0, 5,
          HALT,
        ])
        .build()
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_tcp() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("127.0.0.1:8080".to_string()))
    .with_constant(PoolEntry::String("HTTP/1.1 200 OK\r\n\r\noi kkkkk".to_string()))
    .with_constant(PoolEntry::Module("tcp".to_string()))
    .with_constant(PoolEntry::Function("new_listener".to_string()))
    .with_constant(PoolEntry::Function("accept".to_string()))
    .with_constant(PoolEntry::Function("recv_string".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::Function("println".to_string()))
    .with_constant(PoolEntry::Function("send_string".to_string()))
    .with_constant(PoolEntry::Function("destroy".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name("main")
        .with_arguments(0)
        .with_locals(2)
        .with_bytecode(&[
          LOADCONST, 0x1,
          CALL, 0, 3, 0, 4,
          STORE_0,
          // loop
          LOAD_0,
          CALL, 0, 3, 0, 5,
          STORE_1,
          LOAD_1,
          CALL, 0, 3, 0, 6,
          CALL, 0, 7, 0, 8,
          LOAD_1,
          LOADCONST, 0x2,
          CALL, 0, 3, 0, 9,
          LOAD_1,
          CALL, 0, 3, 0, 10,
          GOTO, 0, 8, // loop
          //
          HALT,
        ])
        .build()
    )
    .build()
}

#[rustfmt::skip]
#[allow(unused)]
fn main_gc() -> module::Module {
  ModuleBuilder::new()
    .with_name("main")
    .with_constant(PoolEntry::String("Hello, World 1".to_string()))
    .with_constant(PoolEntry::String("Hello, World 2".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_function(
      FunctionBuilder::new()
        .with_name("main")
        .with_arguments(0)
        .with_locals(1)
        .with_bytecode(&[
          LOADCONST, 1,
          STORE_0,
          CALL, 0, 0, 0, 1,
          LOAD_0,
          CALL, 0, 3, 0, 0,
          ICONST_0,
          ICONST_0,
          IADD,
          POP,
          LOAD_0,
          CALL, 0, 3, 0, 0,
          HALT,
        ])
        .build()
    )
    .with_function(
      FunctionBuilder::new()
        .with_name("allocs_string")
        .with_arguments(0)
        .with_locals(1)
        .with_bytecode(&[
          LOADCONST, 2,
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
        .with_name("main")
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
        .with_name("main")
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
fn main_module() -> module::Module {
  ModuleBuilder::new()
    .with_name("bytecodes:main")
    .with_constant(PoolEntry::String("oioiiooiiioioioiiiooiio".to_string()))
    .with_constant(PoolEntry::Module("std:out".to_string()))
    .with_constant(PoolEntry::String("rec fib(35):".to_string()))
    .with_constant(PoolEntry::String("iter fib(35):".to_string()))
    .with_constant(PoolEntry::Function("fib".to_string()))
    .with_constant(PoolEntry::Function("fib2".to_string()))
    .with_constant(PoolEntry::Function("println".to_string()))
    .with_constant(PoolEntry::Function("debug".to_string()))
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
        .with_name("main")
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
          CALL, 0, 2, 0, 7, // std:out:print
          LOADCONST, 1,
          STORE_1,
          LOAD_1,
          CALL, 0, 2, 0, 8, // std:out:debug
          LOAD_1,
          CALL, 0, 2, 0, 7, // std:out:print
          LOADCONST, 3,     // "rec fib(35):"
          CALL, 0, 2, 0, 7, // std:out:print
          I_PUSH_BYTE, 35,
          CALL, 0, 0, 0, 5, // fib
          CALL, 0, 2, 0, 7, // std:out:print
          LOADCONST, 4,     // "iter fib(35):"
          CALL, 0, 2, 0, 7, // std:out:print
          I_PUSH_BYTE, 35,
          CALL, 0, 0, 0, 6, // fib2
          CALL, 0, 2, 0, 7, // std:out:print
          HALT,
        ])
        .build(),
    )
    .with_function(
      // func snd(_, a) {
      //   a
      // }
      FunctionBuilder::new()
        .with_name("snd")
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
        .with_name("fib")
        .with_locals(1)
        .with_arguments(1)
        .with_bytecode(&[
          LOAD_0,
          I_PUSH_BYTE, 2,
          I_IFLT, 0, 25,
          LOAD_0,
          ICONST_1,
          ISUB,
          CALL, 0, 0, 0, 5, // main:fib(n - 1)
          LOAD_0,
          I_PUSH_BYTE, 2,
          ISUB,
          CALL, 0, 0, 0, 5, // main:fib(n - 2)
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
        .with_name("fib2")
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
        .with_name("main")
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
