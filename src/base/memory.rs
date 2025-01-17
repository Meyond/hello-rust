use std::mem;

fn main() {
    let mut a = 5;
    let mut b = 10;
    mem::swap(&mut a, &mut b);
    println!("a = {}, b = {}", a, b);
}