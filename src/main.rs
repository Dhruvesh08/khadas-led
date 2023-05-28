use std::fs::File;
use std::io::Write;

fn main() {
    let mut file = File::open("/sys/class/leds/sys_led/trigger").unwrap();
    file.write_all(b"none").unwrap();
}