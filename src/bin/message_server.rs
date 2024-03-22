use std::{convert::TryInto, error::Error, process, thread};

use signal_hook::{consts::SIGABRT, consts::SIGINT, consts::SIGTERM, iterator::Signals};
use zeromq::{Socket, SocketRecv, SocketSend};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGABRT, SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            eprintln!(":message_server received signal {:?}", sig);
            process::exit(1);
        }
    });

    let mut socket = zeromq::RepSocket::new();
    socket
        .connect("tcp://127.0.0.1:5560")
        .await
        .expect("Failed to connect");

    loop {
        let mut repl: String = socket.recv().await?.try_into()?;
        dbg!(&repl);
        repl.push_str(" Reply");
        socket.send(repl.into()).await?;
    }
}