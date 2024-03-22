use std::{error::Error, process, thread};

use signal_hook::{consts::SIGABRT, consts::SIGINT, consts::SIGTERM, iterator::Signals};
use zeromq::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut signals = Signals::new(&[SIGINT, SIGABRT, SIGTERM])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            eprintln!(":message_broker received signal {:?}", sig);
            // interesting that this is not unsafe
            process::exit(1);
        }
    });

    let mut frontend = zeromq::RouterSocket::new();
    frontend.bind("tcp://127.0.0.1:5559").await?;

    let mut backend = zeromq::DealerSocket::new();
    backend.bind("tcp://127.0.0.1:5560").await?;

    let mut capture = zeromq::PubSocket::new();
    capture.bind("tcp://127.0.0.1:9999").await?;

    zeromq::proxy(frontend, backend, Some(Box::new(capture))).await?;
    Ok(())
}