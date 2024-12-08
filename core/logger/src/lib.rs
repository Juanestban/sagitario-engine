use std::fmt::Debug;

fn single_print<T: Debug>(args: Vec<T>) {
  for arg in args {
    print!("{:?}", arg);
  }

  println!("\x1b[0m");
}

pub fn success<T: Debug>(args: Vec<T>) {
  print!("\x1b[1;32m");
  single_print(args);
}

pub fn info<T: Debug>(args: Vec<T>) {
  print!("\x1b[1;34m");
  single_print(args);
}

pub fn note<T: Debug>(args: Vec<T>) {
  print!("\x1b[1;35m");
  single_print(args);
}

pub fn warn<T: Debug>(args: Vec<T>) {
  print!("\x1b[1;33m");
  single_print(args);
}

pub fn danger<T: Debug>(args: Vec<T>) {
  print!("\x1b[1;31m");
  single_print(args);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_logger() {
    single_print(vec!["hola", "mundo"]);
  }

  #[test]
  fn test_success_logger() {
    success(vec!["success!"]);
    single_print(vec!["final!"]);
  }

  #[test]
  fn test_info_logger() {
    info(vec!["info!"]);
    single_print(vec!["final!"]);
  }

  #[test]
  fn test_note_logger() {
    note(vec!["note!"]);
    single_print(vec!["final!"]);
  }

  #[test]
  fn test_warn_logger() {
    warn(vec!["warn!"]);
    single_print(vec!["final!"]);
  }

  #[test]
  fn test_danger_logger() {
    danger(vec!["danger!"]);
    single_print(vec!["final!"]);
  }
}
