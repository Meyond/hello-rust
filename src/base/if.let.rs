fn main() {
    // 处理方式1 match some or none

    let config_max = Some(3u8);
    match config_max {
      Some(max) => println!("The maximum is configured to be {max}"),
      None => (),
    }

    // 处理方式2 if let

    let config_max: Option<u8> = None;
    // let config_max = Some(3u8);
    if let Some(max) = config_max {
      println!("The maximum is configured to be {max}");
    } else {
      println!("The maximum has not been set");
    }

    // 处理方式3 unwrap_or_else

    let result = config_max.unwrap_or_else(|| {
      println!("not exist.");
      0 // 返回0给result
    });

    if config_max.is_some() {
      println!("The element is some : {}", result);
    } else {
      println!("The element is: {}", result);
    }
  }
