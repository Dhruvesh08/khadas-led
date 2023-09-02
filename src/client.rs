use tonic::Request;

pub mod led_controller {
    tonic::include_proto!("led_controller");
}

use led_controller::led_controller_client::LedControllerClient;
use led_controller::SetModeRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let channel = tonic::transport::Channel::from_static("http://[::1]:55551")
    //     .connect()
    //     .await?;

    let mut client = LedControllerClient::connect("http://[::1]:55551").await?;
    

    // Delay for 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Send the first request
    let request1 = tonic::Request::new(SetModeRequest {
        mode: "none".to_string(),
    });
    let response1 = client.set_mode(request1).await?;
    println!("Response1: {:?}", response1);

    // Delay for 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Send the second request
    let request2 = tonic::Request::new(SetModeRequest {
        mode: "default-on".to_string(),
    });
    let response2 = client.set_mode(request2).await?;
    println!("Response2: {:?}", response2);

    // Delay for 5 seconds
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Send the third request
    let request3 = tonic::Request::new(SetModeRequest {
        mode: "heartbeat".to_string(),
    });
    let response3 = client.set_mode(request3).await?;
    println!("Response3: {:?}", response3);

    Ok(())
}
