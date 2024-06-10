use super::{Current, Runtime, RuntimeVisitor};

pub struct StackTrace;

impl RuntimeVisitor for StackTrace {
  fn visit(&self, rt: &Runtime) {
    match rt.current {
      Current::Module(module) => {
        println!("At {}:{}%{}", unsafe { &*module }.name, rt.function.name, rt.ip.borrow());
      }
      Current::Class(class) => {
        println!("At {}:{}%{}", unsafe { &*class }.name, rt.function.name, rt.ip.borrow());
      }
    }
    for frame in rt.call_stack.iter().rev() {
      match frame.current {
        Current::Module(module) => {
          println!(
            "  ~{}:{}%{}",
            unsafe { &*module }.name,
            frame.function.name,
            frame.return_address.borrow()
          );
        }
        Current::Class(class) => {
          println!(
            "  ~{}:{}%{}",
            unsafe { &*class }.name,
            frame.function.name,
            frame.return_address.borrow()
          );
        }
      }
    }
  }
}
