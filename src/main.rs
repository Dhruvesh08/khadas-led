// use std::fs::write;

// fn main() -> std::io::Result<()> {
//     write("/sys/class/leds/sys_led/trigger", "none")?;
//     Ok(())
// }

use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = match File::open("/sys/class/leds/sys_led/trigger") {
        Ok(file) => file,
        Err(e) => panic!("Error opening file: {}", e),
    };
    file.write_all(b"none").unwrap();
}