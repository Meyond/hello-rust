use std::env;
use std::path::{Path, PathBuf};

fn main() {
  // 获取当前文件（通常是 main.rs）的相对路径
  let current_file = file!();
  let current_path = Path::new(&current_file);

  // 获取当前工作目录
  let current_dir = env::current_dir().expect("Failed to get current directory");

  // 将当前文件的相对路径转换为绝对路径
  let absolute_path = current_dir.join(current_path);

  println!("Absolute path of main.rs: {:?}", absolute_path);
}
