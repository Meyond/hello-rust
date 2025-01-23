fn main() {
  let x: i8 = 5;
  let y: Option<i8> = Some(5);
  let z: Option<i8> = None;

  // 使用 match 语句处理 Option 类型
  let sum = match y {
    Some(value) => x + value,
    None => x,
  };

  // 使用 map 处理 Option 类型
  let sum2 = z.map(|value| x + value).unwrap_or(x);

  println!("The sum is {}", sum);
  println!("The sum2 is {}", sum2);
}
