use std::fs::write;

fn main() -> std::io::Result<()> {
    write("/sys/class/leds/sys_led/trigger", "none")?;
    Ok(())
}