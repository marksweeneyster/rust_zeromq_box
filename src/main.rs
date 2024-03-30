//This file is part of rust_zeromq_box, a pub-sub demo project.
// Copyright (C) 2024 Mark Sweeney, marksweeneyster@gmail.com
//
// rust_zeromq_box is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::env;
use std::error::Error;

use zeromq::{Socket, SocketRecv};

mod message_mash;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let ip_addr = if args.len() > 1 {
        &args[1]
    } else {
        "127.0.0.1"
    };
    let port = if args.len() > 2 { &args[2] } else { "8883" };
    let topic = if args.len() > 3 { &args[3] } else { "monster" };

    let endpoint = format!("tcp://{}:{}", ip_addr, port);

    let mut socket = zeromq::SubSocket::new();
    socket
        .connect(endpoint.as_str())
        .await
        .expect("Failed to connect");

    socket.subscribe(topic).await?;

    println!("\nSubscribing to \"{}\", on {}:{}\n", topic, ip_addr, port);

    for i in 0..10 {
        println!("Message {}", i);
        let repl = socket.recv().await?;
        message_mash::process_message(repl, topic);
    }
    Ok(())
}
