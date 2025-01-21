fn main() {

  // let config_max = Some(3u8);
  // match config_max {
  //     Some(max) => println!("The maximum is configured to be {max}"),
  //     _ => (),
  // }

  // let config_max = Some(3u8);
  let config_max: Option<u8> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    } else {
        println!("The maximum has not been set");
    }

}