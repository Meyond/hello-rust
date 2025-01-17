use std::convert::TryInto;

fn main() -> std::io::Result<()> {
    let num: i32 = 42;
    let num_as_u32: u32 = num.try_into().unwrap();
    println!("{}", num_as_u32);
    Ok(())
}