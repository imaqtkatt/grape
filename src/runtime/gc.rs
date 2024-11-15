use super::{Runtime, RuntimeVisitor};

pub struct CleanGc;

impl RuntimeVisitor for CleanGc {
  fn visit(&self, rt: &mut Runtime) {
    rt.local.local.clear();
    rt.stack.clear();
    rt.gc.mark_sweep(&rt.local, &rt.stack);
  }
}
