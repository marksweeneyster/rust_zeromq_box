use std::error::Error;

use zeromq::prelude::*;
mod message_broker_defaults;
use message_broker_defaults::*;

static BROKER_PORT: u16 = 9999;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut frontend = zeromq::RouterSocket::new();
    frontend.bind(format!("tcp://127.0.0.1:{}", CLIENT_PORT).as_str()).await?;

    let mut backend = zeromq::DealerSocket::new();
    backend.bind(format!("tcp://127.0.0.1:{}", SERVER_PORT).as_str()).await?;

    let mut capture = zeromq::PubSocket::new();
    capture.bind( format!("tcp://127.0.0.1:{}", BROKER_PORT).as_str()).await?;

    zeromq::proxy(frontend, backend, Some(Box::new(capture))).await?;
    Ok(())
}
