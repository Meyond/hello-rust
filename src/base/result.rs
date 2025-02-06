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

  let greet_file_result = File::open("hello.txt");

  let greet_file = match greet_file_result {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {e:?}"),
      },
      other_error => {
        panic!("Problem opening the file: {other_error:?}");
      }
    },
  };

}
