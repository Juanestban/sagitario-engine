use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;

pub fn grow_capacity(capacity: usize) -> usize {
  if capacity < 8 {
    8
  } else {
    capacity * 2
  }
}

pub fn free_array<T>(pointer: *mut T, old_capacity: usize) {
  let old_size = old_capacity * std::mem::size_of::<T>();

  unsafe {
    reallocate(pointer as *mut u8, old_size, 0);
  }
}

pub fn grow_array<T>(pointer: *mut T, old_capacity: usize, new_capacity: usize) -> *mut T {
  let old_size = old_capacity * std::mem::size_of::<T>();
  let new_size = new_capacity * std::mem::size_of::<T>();

  unsafe { reallocate(pointer as *mut u8, old_size, new_size) as *mut T }
}

unsafe fn reallocate(pointer: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
  if new_size == 0 {
    if !pointer.is_null() {
      dealloc(pointer, Layout::from_size_align_unchecked(old_size, 1));
    }

    return ptr::null_mut();
  }

  let new_ptr = if pointer.is_null() {
    alloc(Layout::from_size_align_unchecked(new_size, 1))
  } else {
    realloc(pointer, Layout::from_size_align_unchecked(old_size, 1), new_size)
  };

  if new_ptr.is_null() {
    panic!("Failed rellocate memory");
  }

  new_ptr
}
