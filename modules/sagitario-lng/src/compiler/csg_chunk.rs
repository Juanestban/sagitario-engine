use std::ptr;

use super::{
  memory::{free_array, grow_array, grow_capacity},
  value::{Value, ValueArray},
};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
  CONSTANT,
  RETURN,
}

impl From<OpCode> for u8 {
  fn from(op: OpCode) -> Self {
    op as u8
  }
}

pub struct Chunk {
  pub count: usize,
  pub capacity: usize,
  pub code: *mut u8,
  pub lines: *mut u8,
  pub constants: ValueArray,
}

impl Chunk {
  pub fn new() -> Self {
    Self {
      count: 0,
      capacity: 0,
      lines: ptr::null_mut(),
      code: ptr::null_mut(),
      constants: ValueArray::new(),
    }
  }

  pub fn write_chunk(&mut self, byte: u8, line: u8) {
    if self.capacity < self.count + 1 {
      let old_capacity = self.capacity;
      self.capacity = grow_capacity(old_capacity);
      self.code = grow_array::<u8>(self.code, old_capacity, self.capacity);

      if self.code.is_null() {
        panic!("Failed to allocate memory for code array");
      }

      self.lines = grow_array::<u8>(self.lines, old_capacity, self.capacity);

      if self.lines.is_null() {
        panic!("Failed to allocate memory for lines array");
      }
    }

    if !self.code.is_null() && !self.lines.is_null() {
      unsafe {
        *self.code.add(self.count) = byte;
        *self.lines.add(self.count) = line;
      }
    } else {
      panic!("Null pointer desreference")
    }

    self.count += 1;
  }

  pub fn add_constants(&mut self, value: Value) -> usize {
    self.constants.write_value_array(value);

    self.constants.count - 1
  }

  pub fn free_chunk(&mut self) {
    free_array::<u8>(self.code, self.capacity);
    free_array::<u8>(self.lines, self.capacity);
    self.constants.free_value_array();

    *self = Chunk::new();
  }
}

// fn recursive -> [Infinite] === BAD_OPERATION
// impl Drop for Chunk {
//   fn drop(&mut self) {
//     self.free_chunk();
//   }
// }
