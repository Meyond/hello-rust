use std::process::{Command, Stdio};

fn main() -> std::io::Result<()> {
    let output = Command::new("ls")
        .arg("-l")
        .stdout(Stdio::piped())
        .spawn()?
        .wait_with_output()?;
    println!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
