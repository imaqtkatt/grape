use std::{fs, io::Read};

use crate::{
  function::{Function, NativeRet},
  gc::{Gc, ObjString},
  local::Local,
  runtime::Error,
  value::Reference,
};

use super::{builder::ModuleBuilder, Module};

fn read_to_string(local: &mut Local, heap: &mut Gc) -> NativeRet {
  let file_string: Reference = local.load(0).into();
  let path = file_string as *mut ObjString;
  unsafe {
    let path = &(*path).contents;
    let mut file = fs::File::open(path).map_err(Error::other)?;
    let mut s = String::new();
    file.read_to_string(&mut s).map_err(Error::other)?;
    Ok(Some(heap.alloc_string(s)))
  }
}

fn read_to_bytes(_local: &mut Local, _heap: &mut Gc) -> NativeRet {
  todo!()
  // let file_string: Reference = local.load(0).into();
  // let ObjectType::String(ObjString { contents: path }) = &*heap.get(file_string).value else {
  //   panic!();
  // };
  // let mut file = fs::File::open(path).map_err(Error::other)?;
  // let mut buf = Vec::new();
  // file.read_to_end(&mut buf).map_err(Error::other)?;
  // Ok(Some(heap.new_bytes(buf)))
}

pub fn module() -> Module {
  ModuleBuilder::new()
    .with_name("file")
    .with_function(Function::native("read_to_string", 1, read_to_string))
    .with_function(Function::native("read_to_bytes", 1, read_to_bytes))
    .build()
}
