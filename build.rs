fn main() -> Result<(), Box<dyn std::error::Error>> {
    let led = "./proto/led.proto";

    tonic_build::configure()
        .build_server(true)
        .compile(&[led], &["./proto/led"])?;
    Ok(())
}
