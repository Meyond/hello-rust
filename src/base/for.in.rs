fn main() {
  let a = [10, 20, 30, 40, 50];

  for ele in a {
    println!("the value is: {ele}");
  }

  for number in (1..4).rev() {
    println!("{number}!");
  }
  println!("LIFTOFF!!!");
}
