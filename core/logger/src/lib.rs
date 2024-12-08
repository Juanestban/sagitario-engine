use std::{any::Any, fmt::Debug};

fn single_print<T: Debug + 'static + Any>(args: Vec<T>) {
  let message = args
    .iter()
    .map(|arg| {
      if let Some(text) = (arg as &dyn Any).downcast_ref::<&str>() {
        return text.to_string();
      }

      if let Some(text) = (arg as &dyn Any).downcast_ref::<String>() {
        return text.clone();
      }

      format!("{:?}", arg) // Usar Debug para cualquier otro tipo
    })
    .collect::<Vec<String>>()
    .join(" ");

  println!("{}\x1b[0m", message);
}

pub fn success<T: Debug + 'static + Any>(args: Vec<T>) {
  print!("\x1b[1;32m");
  single_print(args);
}

pub fn info<T: Debug + 'static + Any>(args: Vec<T>) {
  print!("\x1b[1;34m");
  single_print(args);
}

pub fn note<T: Debug + 'static + Any>(args: Vec<T>) {
  print!("\x1b[1;35m");
  single_print(args);
}

pub fn warn<T: Debug + 'static + Any>(args: Vec<T>) {
  print!("\x1b[1;33m");
  single_print(args);
}

pub fn danger<T: Debug + 'static + Any>(args: Vec<T>) {
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
