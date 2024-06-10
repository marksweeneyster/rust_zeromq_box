use std::{convert::TryInto, error::Error};

use zeromq::{Socket, SocketRecv, SocketSend};
mod message_broker_defaults;
use message_broker_defaults::SERVER_PORT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::RepSocket::new();
    socket
        .connect(format!("tcp://127.0.0.1:{}", SERVER_PORT).as_str())
        .await
        .expect("Failed to connect");

    loop {
        let mut repl: String = socket.recv().await?.try_into()?;
        dbg!(&repl);
        repl.push_str(" Reply");
        socket.send(repl.into()).await?;
    }
}
