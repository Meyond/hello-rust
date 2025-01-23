fn main() {
  let maybe_value: Option<i32> = Some(42);

  match maybe_value {
      Some(value) => println!("Value: {}", value),
      None => println!("No value"),
  }

  let result: Result<i32, &str> = Ok(42);

  match result {
      Ok(value) => println!("Result: {}", value),
      Err(error) => println!("Error: {}", error),
  }
}