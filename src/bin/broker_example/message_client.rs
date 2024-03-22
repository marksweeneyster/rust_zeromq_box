use std::error::Error;

use zeromq::Socket;
use zeromq::{SocketRecv, SocketSend};
mod message_broker_defaults;
use message_broker_defaults::CLIENT_PORT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut socket = zeromq::ReqSocket::new();
    socket
        .connect(format!("tcp://127.0.0.1:{}", CLIENT_PORT).as_str())
        .await
        .expect("Failed to connect");

    socket.send("hello".into()).await?;
    let repl = socket.recv().await?;
    dbg!(repl);

    socket.send("hola".into()).await?;
    let repl = socket.recv().await?;
    dbg!(repl);

    socket.send("moshi mosh".into()).await?;
    let repl = socket.recv().await?;
    dbg!(repl);

    Ok(())
}
