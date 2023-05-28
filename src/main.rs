use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let path = "/sys/class/leds/sys_led/trigger";
    let mut stream = UnixStream::connect(path)?;
    stream.write_all(b"none")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}