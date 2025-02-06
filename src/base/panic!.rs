// 设置Cargo.toml 将panic!由展开切换为终止
// [profile.release]
// panic = 'abort'

fn main() {

  // 用 panic! 处理不可恢复的错误
  // 用 Result 处理可恢复的错误

  let v = vec![1, 2, 3];
  print!("{}", v[99]);
  panic!("crash and burn");


  // 简写：unwrap 和 expect
  let greeting_file = File::open("hello.txt").unwrap();

  let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");

  // 使用?向调用者返回错误
  let greeting_file = File::open("hello.txt")?;
}