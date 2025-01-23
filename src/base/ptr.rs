use std::ptr;

fn main() {
  let mut array = [1, 2, 3, 4, 5];
  let ptr = array.as_mut_ptr();
  unsafe {
    *ptr.offset(1) = 10;
  }
  println!("{:?}", array);
}
