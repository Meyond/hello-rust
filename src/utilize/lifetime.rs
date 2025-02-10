/**
 * 生命周期说明：
 * 1、生命周期的主要目标是避免悬垂引用（dangling references）
 * 2、生命周期注解不改变任何引用的生命周期的长短,它描述了多个引用生命周期相互的关系
 * 3、函数返回的引用的生命周期与传入参数的生命周期中较短那个保持一致
 *
 */

// 生命周期规则：

// 1、编译器为每一个引用参数都分配一个生命周期参数
fn foo<'a, 'b>(x: &'a i32, y: &'b i32) {}

// 2、如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
fn foo<'a>(x: &'a i32) -> &'a i32 {}

// 3、如果有多个输入生命周期参数，但其中一个是 &self 或 &mut self，那么 self 的生命周期被赋予所有输出生命周期参数

struct Example {
  value: i64,
}

impl Example {
  // fn get_value<'a, 'b>(&'a self, _other: &'b i32) -> &'a i32 {
  // 等价于
  fn get_value(&self, _other: &i32) -> &i64 {
    &self.value
  }
}

fn main() {
  let example = Example { value: 10 };
  print!("{}", example.get_value(&10));
}

// =====================================================================

fn longest<'b>(x: &'b str, y: &'b str) -> &'b str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

fn main() {
  let string1 = String::from("long string is long");
  let result;
  {
    let string2 = String::from("xyz");
    // longest 函数返回的引用的生命周期与传入参数的生命周期中较短那个保持一致,所以result的生命周期与string2的生命周期一致,所以这里会报错
    result = longest(string1.as_str(), string2.as_str());
  }
  println!("The longest string is {result}");
}
