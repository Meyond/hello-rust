enum MixedType {
  Int(i32),
  String(String),
  Float(f32),
  Bool(bool),
}

fn main() {
  // 也可使用Vec::new(); 和 push 创建和添加
  let mut vec: Vec<MixedType> = vec![
    MixedType::Int(5),
    MixedType::String("hello".to_string()),
    MixedType::Float(3.14),
    MixedType::Bool(false),
  ];

  // dbg!(&vec);

  for item in vec {
    match item {
      MixedType::Int(i) => println!("Int: {}", i),
      MixedType::String(s) => println!("String: {}", s),
      MixedType::Float(f) => println!("Float: {}", f),
      MixedType::Bool(b) => println!("Bool: {}", b),
    }
  }
}
