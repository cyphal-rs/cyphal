mod get_info;
mod heartbeat;
mod router;

use async_std::sync::Mutex;
use cyphal::Transport;
use cyphal_can::CanTransport;
use cyphal_socketcan::CanFdSocket;
use heartbeat::HeartbeatMessage;
use router::Router;
use std::{sync::Arc, time::Duration};

#[async_std::main]
async fn main() {
    let socket = CanFdSocket::new("vcan1").unwrap();
    let transport = Arc::new(Mutex::new(CanTransport::new(socket).unwrap()));
    let router = Router {};

    async_std::task::spawn(send_heartbeat(transport.clone()));

    if let Err(e) = transport.lock().await.serve(router).await {
        print!(
            "An error occured while trying to serve incoming requests: {}",
            e
        )
    };
}

async fn send_heartbeat<T: Transport>(transport: Arc<Mutex<T>>) {
    loop {
        let message = HeartbeatMessage::new(1, [0; 65]).unwrap();
        let _r = &transport.lock().await.publish(&message).await;
        async_std::task::sleep(Duration::from_secs(1)).await;
    }
}
