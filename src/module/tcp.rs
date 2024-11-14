// use std::{
//   io::{Read, Write},
//   net::{TcpListener, TcpStream},
// };

use crate::{
  function::{Function, NativeRet},
  heap::{Heap, ObjString},
  local::Local,
  runtime::Error,
  value::Reference,
};

use super::{builder::ModuleBuilder, Module};

fn new_listener(local: &mut Local, _heap: &mut Heap) -> NativeRet {
  let addr: Reference = local.load(0).into();

  unsafe {
    let addr = addr as *mut ObjString;
    let addr = &(*addr).contents;
    // let ObjectType::String(ObjString { contents: addr }) = &*heap.get(addr).value else {
    //   panic!();
    // };

    let listener = std::net::TcpListener::bind(addr).map_err(Error::other)?;
    // let listener = heap.alloc(Object::new(ObjectType::Native(Box::new(listener))));
    let listener = todo!();

    Ok(Some(listener))
  }
}

fn destroy(local: &mut Local, _heap: &mut Heap) -> NativeRet {
  // let listener: Reference = local.load(0).into();
  // heap.free(listener);
  // Ok(None)
  todo!()
}

fn accept(local: &mut Local, _heap: &mut Heap) -> NativeRet {
  let listener: Reference = local.load(0).into();

  todo!()
  // let ObjectType::Native(nat) = &*heap.get(listener).value else { panic!() };
  // match nat.downcast_ref::<TcpListener>() {
  //   Some(listener) => {
  //     let (tcp_stream, _) = listener.accept().map_err(Error::other)?;
  //     // let tcp_stream = heap.alloc(Object::new(ObjectType::Native(Box::new(tcp_stream))));
  //     let tcp_stream = todo!();
  //     Ok(Some(tcp_stream))
  //   }
  //   None => todo!(),
  // }
}

fn recv_string(local: &mut Local, _heap: &mut Heap) -> NativeRet {
  let stream: Reference = local.load(0).into();

  todo!()
  // let ObjectType::Native(nat) = &*heap.get(stream).value else { panic!() };
  // match nat.downcast_ref::<TcpStream>() {
  //   Some(mut stream) => {
  //     let mut buf = [0; 2048];
  //     stream.read(&mut buf).map_err(Error::other)?;
  //     let s = String::from_utf8(buf.to_vec()).map_err(Error::other)?;
  //     Ok(Some(heap.alloc_string(s)))
  //   }
  //   None => todo!(),
  // }
}

fn send_string(local: &mut Local, _heap: &mut Heap) -> NativeRet {
  let stream: Reference = local.load(0).into();
  let string: Reference = local.load(1).into();

  todo!()
  // let ObjectType::Native(stream) = &*heap.get(stream).value else { panic!() };
  // let ObjectType::String(ObjString { contents: send }) = &*heap.get(string).value else { panic!() };

  // match stream.downcast_ref::<TcpStream>() {
  //   Some(mut stream) => {
  //     stream.write_all(send.as_bytes()).map_err(Error::other)?;
  //     stream.flush().map_err(Error::other)?;
  //     Ok(None)
  //   }
  //   None => todo!(),
  // }
}

pub fn module() -> Module {
  ModuleBuilder::new()
    .with_name("tcp")
    .with_function(Function::native("new_listener", 1, new_listener))
    .with_function(Function::native("destroy", 1, destroy))
    .with_function(Function::native("accept", 1, accept))
    .with_function(Function::native("recv_string", 1, recv_string))
    .with_function(Function::native("send_string", 2, send_string))
    .build()
}
