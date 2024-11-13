use super::RuntimeVisitor;

pub struct RunGc;

impl RuntimeVisitor for RunGc {
  fn visit(&self, rt: &mut super::Runtime) {
    rt.local.local.clear();
    rt.stack.clear();
    rt.heap.gc(&rt.local, &rt.stack);
  }
}
