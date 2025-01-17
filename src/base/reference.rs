fn main() {
  let s_str: &str = "Hello";
  let mut s_string = s_str.to_string();

  change(&mut s_string);
  println!("{}", s_string)
}

fn change(some_string: &mut String) {
  some_string.push_str(", world");
}
