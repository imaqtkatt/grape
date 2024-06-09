use super::{Runtime, RuntimeVisitor};

pub struct StackTrace;

impl RuntimeVisitor for StackTrace {
  fn visit(&self, rt: &Runtime) {
    match rt.current {
      super::Current::Module => {
        println!("At {}:{}%{}", unsafe { &*rt.module }.name, rt.function.name, rt.ip.borrow());
      }
      super::Current::Class => {
        println!("At {}:{}%{}", unsafe { &*rt.class }.name, rt.function.name, rt.ip.borrow());
      }
    }
    for frame in rt.call_stack.iter().rev() {
      match frame.returning_to {
        super::Current::Module => {
          println!(
            "  ~{}:{}%{}",
            unsafe { &*frame.module }.name,
            frame.function.name,
            frame.return_address.borrow()
          );
        }
        super::Current::Class => {
          println!(
            "  ~{}:{}%{}",
            unsafe { &*frame.class }.name,
            frame.function.name,
            frame.return_address.borrow()
          );
        }
      }
    }
  }
}
