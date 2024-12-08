use std::ptr;

use super::memory::{free_array, grow_array, grow_capacity};

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum OpCode {
  RETURN,
  FALSE,
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
}

impl Chunk {
  pub fn new() -> Self {
    Self {
      count: 0,
      capacity: 0,
      code: ptr::null_mut(),
    }
  }

  pub fn write_chunk(&mut self, byte: OpCode) {
    if self.capacity < self.count + 1 {
      let old_capacity = self.capacity;
      self.capacity = grow_capacity(old_capacity);
      self.code = grow_array::<u8>(self.code, old_capacity, self.capacity);
    }

    unsafe {
      *self.code.add(self.count) = byte as u8;
    }

    self.count += 1;
  }

  pub fn free_chunk(&mut self) {
    free_array::<u8>(self.code, self.capacity);

    *self = Chunk::new();
  }
}

// fn recursive -> [Infinite] === BAD_OPERATION
// impl Drop for Chunk {
//   fn drop(&mut self) {
//     self.free_chunk();
//   }
// }
