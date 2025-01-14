
fn main() {
  let jack = build_user(
      String::from("someone@qq.com"),
      String::from("jack")
  );

  println!("rect1 is {jack:?}");

  let mut rose = User {
      email: "rose@qq.com".to_string(),
      ..jack
  };

  // println!("{}", rose.email);
  dbg!(&rose);

  let black = Color(0, 0, 0);

  let subU = Unit;
}

fn build_user(email: String, username: String) -> User {
  User {
      active: true,
      email,
      username,
      sign_in_count: 1,
  }
}

#[derive(Debug)]
struct User {
  username: String,
  email: String,
  active: bool,
  sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32);

// 类单元结构体(没有任何字段的结构体)
struct Unit;