enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String),
}

fn main() {

  let home = IpAddrKind::V4(127, 0, 0, 1);

  let loopback = IpAddrKind::V6(String::from("::1"));

  // let home = IpAddr {
  //     kind: IpAddrKind::V4,
  //     address: String::from("127.0.0.1"),
  // };
}

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }
