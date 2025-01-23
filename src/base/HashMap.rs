use std::collections::HashMap;
fn main() {
  let mut scores = HashMap::new();

  scores.insert("Blue", 10);
  scores.insert("Yellow", 50);

  let team_name = "Blue";
  let score = scores.get(&team_name).copied().unwrap_or(0);

  println!("Score of Blue is: {}", score);

  scores.insert("Blue", 11);
  let score = scores.get(&team_name).copied().unwrap_or(0);
  println!("Score of Blue is: {}", score);

  // 遍历 HashMap，使用引用避免转移所有权
  // for (key, value) in &scores {
  //     println!("{:?}: {:?}", key, value);
  // }
  dbg!(&scores); // 内置类型不需要#[derive(Debug)]
}
