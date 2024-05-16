use super::{Runtime, RuntimeVisitor};

pub struct StackTrace;

impl RuntimeVisitor for StackTrace {
  fn visit(&self, rt: &Runtime) {
    println!("At {}:{}%{}", rt.module.name, rt.function.name, rt.ip.take());
    for frame in rt.call_stack.iter().rev() {
      println!("  ~{}:{}%{}", frame.module.name, frame.function.name, frame.return_address.take());
    }
  }
}
