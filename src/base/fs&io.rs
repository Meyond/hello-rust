use std::fs;
// use std::io;
// use std::io::{Read, Write, Seek, SeekFrom};
use std::io::{self, Read, Seek, SeekFrom, Write};

fn main() -> io::Result<()> {
  let mut file = fs::File::create("test.txt")?;
  file.write_all(b"Hello, Rust!")?;

  file.seek(SeekFrom::Start(0))?; // 将文件指针重置到文件开头
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;
  println!("{}", contents);

  Ok(())
}
