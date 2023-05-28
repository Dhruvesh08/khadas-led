use std::fs;

fn main() -> std::io::Result<()> {
    let path = "/sys/class/leds/sys_led/trigger";
    let trigger_value = "none";

    fs::write(path, trigger_value)?;

    Ok(())
}
