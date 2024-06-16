use std::{collections::BTreeMap, rc::Rc};

use crate::{function::Function, pool_entry::PoolEntry};

use super::{Class, Field};

#[derive(Default)]
pub struct ClassBuilder {
  name: String,
  fields: BTreeMap<Rc<str>, Field>,
  constants: Vec<PoolEntry>,
  methods: Vec<Function>,
}

impl ClassBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn with_name(mut self, name: &str) -> Self {
    self.name = name.to_string();
    self.constants.push(PoolEntry::Class(name.to_string()));
    self
  }

  pub fn with_field(mut self, field_name: &str) -> Self {
    let offset = self.fields.len();
    let name = Rc::from(field_name);
    self.fields.entry(name).or_insert_with(|| Field { vis: Field::PUBLIC, offset: offset as u8 });
    self
  }

  pub fn with_constant(mut self, entry: PoolEntry) -> Self {
    self.constants.push(entry);
    self
  }

  pub fn with_method(mut self, method: Function) -> Self {
    self.methods.push(method);
    self
  }

  pub fn build(self) -> Class {
    let methods = self.methods.into_iter().map(|m| (m.name.clone(), m)).collect();
    Class { name: self.name.into(), constants: self.constants, fields: self.fields, methods }
  }
}
