use std::fmt;

struct Point {
  x: i32,
  y: i32,
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main() {
  let p = Point { x: 3, y: 4 };
  println!("{}", p);
}
