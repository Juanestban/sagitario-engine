use std::ptr;

use super::memory::{free_array, grow_array, grow_capacity};
use sagitario_logger::info;

pub type Value = f64;

pub struct ValueArray {
  pub capacity: usize,
  pub count: usize,
  pub values: *mut Value,
}

impl ValueArray {
  pub fn new() -> Self {
    Self {
      capacity: 0,
      count: 0,
      values: ptr::null_mut(),
    }
  }

  pub fn write_value_array(&mut self, value: Value) {
    if self.capacity < self.count + 1 {
      let old_capacity = self.capacity;
      self.capacity = grow_capacity(old_capacity);
      self.values = grow_array::<Value>(self.values, old_capacity, self.capacity);
    }

    unsafe {
      *self.values.add(self.count) = value;
    }

    self.count += 1;
  }

  pub fn free_value_array(&mut self) {
    free_array::<Value>(self.values, self.capacity);

    *self = ValueArray::new();
  }
}

pub fn print_value(value: f64) {
  info(vec![format!("{}", value)]);
}
