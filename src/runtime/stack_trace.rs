use super::{Runtime, RuntimeVisitor};

pub struct StackTrace;

impl RuntimeVisitor for StackTrace {
  fn visit(&self, rt: &Runtime) {
    println!("At {}:{}%{}", rt.callable.name(), rt.function.name, rt.ip.borrow());
    for frame in rt.call_stack.iter().rev() {
      println!(
        "  ~{}:{}%{}",
        frame.callable.name(),
        frame.function.name,
        frame.return_address.borrow()
      );
    }
  }
}
