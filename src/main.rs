// use std::fs::write;

// fn main() -> std::io::Result<()> {
//     write("/sys/class/leds/sys_led/trigger", "none")?;
//     Ok(())
// }

use std::fs::{File};
use std::io::{self, prelude::*};

fn main() -> io::Result<()> {
    let  file = File::create("/sys/class/leds/sys_led/trigger")?;
    turn_led_on(file)?;
    Ok(())
}


pub fn turn_led_on(mut file:File) -> io::Result<()> {
    file.write(b"default-on")?;
    Ok(())
}

pub fn turn_led_off(mut file:File) -> io::Result<()> {  
    file.write(b"none")?;
    Ok(())
}
pub fn turn_led_blink(mut file:File) -> io::Result<()> {  
    file.write(b"heartbeat")?;
    Ok(())
}