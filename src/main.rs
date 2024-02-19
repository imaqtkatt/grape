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

#[rustfmt::skip]
fn main() {
  let main = Module {
    name: Box::from("main"),
    names: vec![String::from("main"), String::from("snd")],
    constants: Vec::new(),
    functions: vec![
      // func main() {
      //   snd(0, 1) + 2
      // }
      Function {
        name: Box::from("main"),
        locals: 0,
        arguments: 0,
        code: Code::Bytecode(vec![
          PUSH_BYTE, 0,
          PUSH_BYTE, 1,
          CALL, 0, 0, 0, 1, // main/snd
          PUSH_BYTE, 2,
          IADD,
          RETURN,
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
    ],
  };

  let mut ctx = Context::new();
  ctx.add_module(main);

  let ret = Runtime::boot(&ctx);
  println!("ret = {ret:?}")
}
