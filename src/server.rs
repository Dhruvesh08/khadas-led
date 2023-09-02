use khadas_led_sdk::LedController as LedModule;
use tonic::{transport::Server, Request, Response, Status};

use led_controller::{
    led_controller_server::LedController, led_controller_server::LedControllerServer,
};
use led_controller::{SetModeRequest, SetModeResponse};

pub mod led_controller {
    tonic::include_proto!("led_controller");
}

struct LedControllerService;

#[tonic::async_trait]
impl LedController for LedControllerService {
    async fn set_mode(
        &self,
        request: Request<SetModeRequest>,
    ) -> Result<Response<SetModeResponse>, Status> {
        let mode = request.into_inner().mode;

        // Implement your LED control logic here.
        // You can use the LedController struct we defined earlier.
        let led_controller = LedModule::new("/sys/class/leds/sys_led/trigger");
        led_controller.set_mode(&mode).unwrap();

        let response = SetModeResponse { success: true };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:55551".parse().unwrap();
    print!("Listening on: {}", addr);
    // let trigger_path = "/sys/class/leds/sys_led/trigger";
    let led_controller_service = LedControllerService {};
    // let led_controller_service = LedModule::new(trigger_path);

    tonic::transport::Server::builder()
        .add_service(LedControllerServer::new(led_controller_service))
        .serve(addr)
        .await?;

    Ok(())
}
