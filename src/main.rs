use tokio::net::UnixDatagram;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path = "/sys/class/leds/sys_led/trigger";
    let trigger_value = "none";

    let socket = UnixDatagram::unbound()?;
    socket.connect(path)?;

    socket.send(trigger_value.as_bytes()).await?;
    Ok(())
}