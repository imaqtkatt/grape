use crate::{local::Local, stack::Stack};

use super::{Heap, ObjArray, ObjDict};

impl Heap {
  pub fn gc<const SIZE: usize>(&mut self, local: &Local, stack: &Stack<SIZE>) {
    let mut stack_and_local = stack.iter().chain(local.iter()).collect::<Vec<_>>();
    while let Some(value) = stack_and_local.pop() {
      if value.tag() == crate::value::Value::TAG_STRING {
        self.marked.insert(*value);
        continue;
      }
      if value.tag() == crate::value::Value::TAG_DICT {
        self.marked.insert(*value);
        let ptr = value.reference() as *mut ObjDict;

        let refs = unsafe { (*ptr).refs() };
        stack_and_local.extend(refs);
        continue;
      }
      if value.tag() == crate::value::Value::TAG_ARRAY {
        self.marked.insert(*value);
        let ptr = value.reference() as *mut ObjArray;

        let refs = unsafe { (*ptr).refs() };
        stack_and_local.extend(refs);
        continue;
      }
    }

    self.roots.retain(|root| {
      if self.marked.contains(root) {
        return true;
      }

      match root.tag() {
        crate::value::Value::TAG_STRING => unsafe {
          std::alloc::dealloc(
            root.reference() as *mut _,
            std::alloc::Layout::new::<super::ObjString>(),
          );
        },
        crate::value::Value::TAG_DICT => unsafe {
          std::alloc::dealloc(
            root.reference() as *mut _,
            std::alloc::Layout::new::<super::ObjDict>(),
          );
        },
        crate::value::Value::TAG_ARRAY => unsafe {
          std::alloc::dealloc(
            root.reference() as *mut _,
            std::alloc::Layout::new::<super::ObjArray>(),
          );
        },
        _ => unreachable!(),
      }

      // if root.tag() == crate::value::Value::TAG_STRING {}
      // if root.tag() == crate::value::Value::TAG_DICT {
      //   unsafe {
      //     std::alloc::dealloc(
      //       root.reference() as *mut _,
      //       std::alloc::Layout::new::<super::ObjDict>(),
      //     );
      //   }
      // }
      // if root.tag() == crate::value::Value::TAG_ARRAY {
      //   unsafe {
      //     std::alloc::dealloc(
      //       root.reference() as *mut _,
      //       std::alloc::Layout::new::<super::ObjArray>(),
      //     );
      //   }
      // }
      return false;
    });

    self.marked.clear();
  }
}
