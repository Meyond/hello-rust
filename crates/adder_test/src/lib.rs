/**
 * assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。
 * 当断言失败时，这些宏会使用调试格式打印出其参数，
 * 这意味着被比较的值必须实现了 PartialEq 和 Debug trait。
 * 所有的基本类型和大部分标准库类型都实现了这些 trait。
 * 对于自定义的结构体和枚举，需要实现 PartialEq 才能断言它们的值是否相等。
 * 需要实现 Debug 才能在断言失败时打印它们的值。
 */

#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width > other.width && self.height > other.height
  }
}

pub fn add_two(a: usize) -> usize {
  a + 3
}

pub fn greeting(name: &str) -> String {
  String::from("Hello!")
  // format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn larger_can_hold_smaller() {
    let larger = Rectangle {
      width: 8,
      height: 7,
    };
    let smaller = Rectangle {
      width: 5,
      height: 1,
    };

    assert!(larger.can_hold(&smaller));
  }

  #[test]
  fn it_adds_two() {
    let result = add_two(2);
    assert_ne!(result, 4);
  }

  // 自定义测试失败信息
  #[test]
  fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
      result.contains("Carol"),
      "Greeting did not contain name, value was `{result}`"
    );
  }
}
