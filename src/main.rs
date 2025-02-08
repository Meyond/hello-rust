// 生命周期的主要目标是避免悬垂引用（dangling references）
fn main() {
  let r;

  {
    let x = 5;
    r = &x;
    println!("r: {r}");
  }
}