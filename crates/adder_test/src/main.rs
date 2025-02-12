pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 {
      panic!("Guess value must be greater than or equal to 1, got {value}.");
    } else if value > 100 {
      panic!("Guess value must be less than or equal to 100, got {value}.");
    }

    Guess { value } // 结构体实例化
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  // 触发panic并且输出消息包含指定内容才被判定为测试通过
  #[test]
  #[should_panic(expected = "less than or equal to 100")]
  fn greater_than_100() {
    Guess::new(200);
  }

  // ANCHOR: here
  #[test]
  fn it_works() -> Result<(), String> {
    let result = add(2, 2);

    if result == 4 {
      Ok(())
    } else {
      Err(String::from("two plus two does not equal four"))
    }
  }
  // ANCHOR_END: here
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}
