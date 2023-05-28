// use std::fs::write;

// fn main() -> std::io::Result<()> {
//     write("/sys/class/leds/sys_led/trigger", "none")?;
//     Ok(())
// }

use std::fs::{File};
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let mut file = File::create("/sys/class/leds/sys_led/trigger")?;
    file.write(b"none")?;
    Ok(())
}