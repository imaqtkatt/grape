use crate::{local::Local, stack::Stack};

use super::{Heap, Object};

impl Heap {
  pub fn gc(&mut self, local: &Local, stack: &Stack) {
    let mut stack_and_local = stack.iter().chain(local.iter());
    while let Some(value) = stack_and_local.next() {
      if value.is_reference_non_null() {
        let got = &mut self.memory[value.reference()];
        *got.marked.get_mut() = true;

        let mut refs = got.value.refs();

        while let Some(r#ref) = refs.pop_first() {
          let got = &mut self.memory[r#ref];
          *got.marked.get_mut() = true;
          let value_refs = got.value.refs();
          refs.extend(value_refs);
        }
      }
    }

    let mut to_free = Vec::new();
    for i in 1..self.memory.len() {
      let obj = &mut self.memory[i];
      if !obj.marked.get() && self.free.insert(i) {
        to_free.push(i);
      } else {
        *obj.marked.get_mut() = false;
      }
    }

    if to_free.is_empty() {
      return;
    }

    for r#ref in to_free {
      self.freed.push(r#ref);
      self.memory[r#ref] = Object::null();
    }
  }
}
