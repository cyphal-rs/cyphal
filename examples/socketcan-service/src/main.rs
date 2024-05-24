mod get_info;
mod heartbeat;
mod router;

use cyphal::Transport;
use cyphal_can::CanTransport;
use cyphal_socketcan::CanSocket;
use router::Router;

#[async_std::main]
async fn main() {
    let socket = CanSocket::new("vcan0").unwrap();
    let mut transport = CanTransport::new(socket).unwrap();
    let router = Router {};

    if let Err(e) = transport.serve(router).await {
        print!(
            "An error occured while trying to serve incoming requests: {}",
            e
        )
    };
}
