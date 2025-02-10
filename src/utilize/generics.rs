/**
 * T type
 * U 代表额外类型
 * S 另一个类型，区别于T
 *
 * K key
 * V value
 *
 * E error / element
 */

fn largest<T: PartialOrd>(list: &[T]) -> &T {
  let mut largest = &list[0];

  for item in list {
    if item > largest {
      largest = item;
    }
  }

  largest
}

fn main() {
  let number_list = vec![34, 50, 25, 100, 65];
  let result = largest(&number_list);
  println!("The largest number is {result}");

  let char_list = vec!['y', 'm', 'a', 'q'];
  let result = largest(&char_list);
  println!("The largest char is {result}");
}

// 泛型约束

// Copy 浅拷贝
// Clone 深拷贝

// PartialEq 部分相等
// Eq 完全相等

// PartialOrd 部分排序
// Ord 完全排序

// Serialize 序列化
// Deserialize 反序列化

// Iterator 迭代器