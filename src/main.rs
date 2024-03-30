use std::env;
use std::error::Error;
use zeromq::{Socket, SocketRecv};

//extern crate flatbuffers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let port  = if args.len() > 1 { &args[1] } else { "8883" };
    let topic = if args.len() > 2 { &args[2] } else { "hotdogs" };

    let endpoint = "tcp://127.0.0.1:".to_owned() + port;

    let mut socket = zeromq::SubSocket::new();
    socket
        .connect(endpoint.as_str())
        .await
        .expect("Failed to connect");

    socket.subscribe(topic).await?;

    let mut _builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    for i in 0..10 {
        println!("Message {}", i);
        let repl = socket.recv().await?;
        for w in repl.iter() {
            dbg!(w);
        }
        //dbg!(repl);
    }
    Ok(())
}