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

use bytes::{Buf, Bytes};
use std::env;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use zeromq::{Socket, SocketSend, ZmqMessage};

use crate::monster_generated::my_game::sample::{
    Color, Equipment, Monster, MonsterArgs, Vec3, Weapon, WeaponArgs,
};

#[allow(dead_code, unused_imports)]
#[path = "../../target/flatbuffers/Monster_generated.rs"]
mod monster_generated;

fn make_monster() -> ZmqMessage {
    let mut builder = flatbuffers::FlatBufferBuilder::with_capacity(1024);

    let weapon_one_name = builder.create_string("Club");
    let weapon_two_name = builder.create_string("Other Club");

    let club = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_one_name),
            damage: 3,
        },
    );
    let other_club = Weapon::create(
        &mut builder,
        &WeaponArgs {
            name: Some(weapon_two_name),
            damage: 5,
        },
    );

    let name = builder.create_string("Troll");

    let inventory = builder.create_vector(&[0u8, 9, 8, 7, 6, 5, 4, 3, 2, 1]);

    let weapons = builder.create_vector(&[club, other_club]);

    let x = Vec3::new(11.1, 22.2, 33.3);
    let y = Vec3::new(-4.0, -5.0, -6.0);
    let path = builder.create_vector(&[x, y]);
    let ogre = Monster::create(
        &mut builder,
        &MonsterArgs {
            pos: Some(&Vec3::new(21.0f32, 22.0f32, 13.0f32)),
            mana: 199,
            hp: 150,
            name: Some(name),
            inventory: Some(inventory),
            color: Color::Red,
            weapons: Some(weapons),
            equipped_type: Equipment::Weapon,
            equipped: Some(club.as_union_value()),
            path: Some(path),
            ..Default::default()
        },
    );
    builder.finish(ogre, None);
    let mut buf = builder.finished_data();
    println!("bytes len {}", buf.len());

    let topic_bytes = Bytes::from("monster");
    let ogre_bytes = buf.copy_to_bytes(buf.len());

    let frames = vec![topic_bytes, ogre_bytes];
    ZmqMessage::try_from(frames).expect("ZMQ message error")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let port = if args.len() > 1 { &args[1] } else { "8883" };
    let endpoint = format!("tcp://127.0.0.1:{}", port);

    let mut publisher = zeromq::PubSocket::new();
    publisher.bind(endpoint.as_str()).await?;

    let message = make_monster();

    loop {
        publisher.send(message.clone()).await?;
        sleep(Duration::from_millis(1000)).await;
    }
    Ok(())
}
