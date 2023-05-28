use std::process::Command;

fn main() {
    let mut output = Command::new("sudo")
        .arg("tee")
        .arg("/sys/class/leds/sys_led/trigger")
        .arg("none")
        .spawn()
        .expect("Failed to execute command.");

    let status = output.wait().expect("Failed to wait for command execution.");
    if !status.success() {
        eprintln!("Command failed with exit code: {:?}", status.code());
    }
}
