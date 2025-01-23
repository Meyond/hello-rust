use std::cell::RefCell;
use std::rc::Rc;

fn main() {
  let shared = Rc::new(RefCell::new(5));
  let clone1 = Rc::clone(&shared);
  let clone2 = Rc::clone(&shared);
  *shared.borrow_mut() = 10;
  println!("{}", *clone1.borrow());
}
