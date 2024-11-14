use crate::{local::Local, stack::Stack};

use super::{Heap, ObjArray, ObjClass, ObjDict, ObjString};

impl Heap {
  pub fn gc<const SIZE: usize>(&mut self, local: &Local, stack: &Stack<SIZE>) {
    let mut stack_and_local = stack.iter().chain(local.iter()).collect::<Vec<_>>();
    while let Some(value) = stack_and_local.pop() {
      match value.tag() {
        crate::value::Value::TAG_STRING => {
          self.marked.insert(*value);
        }
        crate::value::Value::TAG_DICT => {
          self.marked.insert(*value);
          let ptr = value.reference() as *mut ObjDict;

          let refs = unsafe { (*ptr).refs() };
          stack_and_local.extend(refs);
        }
        crate::value::Value::TAG_ARRAY => {
          self.marked.insert(*value);
          let ptr = value.reference() as *mut ObjArray;

          let refs = unsafe { (*ptr).refs() };
          stack_and_local.extend(refs);
        }
        crate::value::Value::TAG_CLASS => {
          self.marked.insert(*value);
          let ptr = value.reference() as *mut ObjClass;

          let refs = unsafe { (*ptr).refs() };
          stack_and_local.extend(refs);
        }
        _ => (),
      }
    }

    self.roots.retain(|root| {
      if self.marked.contains(root) {
        return true;
      }

      match root.tag() {
        crate::value::Value::TAG_STRING => unsafe {
          std::alloc::dealloc(root.reference() as *mut _, std::alloc::Layout::new::<ObjString>());
        },
        crate::value::Value::TAG_DICT => unsafe {
          std::alloc::dealloc(root.reference() as *mut _, std::alloc::Layout::new::<ObjDict>());
        },
        crate::value::Value::TAG_ARRAY => unsafe {
          std::alloc::dealloc(root.reference() as *mut _, std::alloc::Layout::new::<ObjArray>());
        },
        crate::value::Value::TAG_CLASS => unsafe {
          std::alloc::dealloc(root.reference() as *mut _, std::alloc::Layout::new::<ObjClass>())
        },
        _ => unreachable!(),
      }

      return false;
    });

    self.marked.clear();
  }
}
