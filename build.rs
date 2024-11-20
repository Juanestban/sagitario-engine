use std::{env, fs, path::Path};

fn main() {
  let source_path = Path::new("src/assets/icon.png");

  let out_dir = env::var("OUT_DIR").unwrap();
  let target_dir = Path::new(&out_dir).join("../../../");

  if !source_path.exists() {
    panic!("File not found: {:?}", source_path);
  }

  let dest_path = target_dir.join("icon.png");
  fs::copy(source_path, dest_path).unwrap();

  println!("[-] cargo:rerun-if-changed=assets/icon.png");
}
