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

pub use monster_generated::my_game::sample::root_as_monster;

#[allow(dead_code, unused_imports)]
#[path = "../target/flatbuffers/Monster_generated.rs"]
mod monster_generated;

pub fn process_message(msg: zeromq::ZmqMessage, topic: &str) {
    if msg.len() < 2 {
        return;
    }
    println!("topic: {}", topic);

    if let (Some(envelope), Some(buf)) = (msg.get(0), msg.get(1)) {
        if envelope.eq(topic) {
            process_monster(buf);
        }
    }
}

fn process_monster(buf: &[u8]) {
    let monster = match root_as_monster(buf) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("process_monster: {}", e);
            return;
        }
    };
    // Get and test some scalar types from the FlatBuffer.
    let hp = monster.hp();
    let mana = monster.mana();
    let name = monster.name().unwrap_or("_");
    let color = monster.color();

    println!("name: {}, mana: {}, hp: {}", name, mana, hp);
    println!("color: {}", color.variant_name().unwrap_or("_"));

    match monster.pos() {
        Some(pos) => {
            let x = pos.x();
            let y = pos.y();
            let z = pos.z();
            println!("position: [{},{},{}]", x, y, z);
        }
        None => println!("no position"),
    }

    match monster.weapons() {
        Some(weapons) => {
            for w in weapons.iter() {
                println!("weapon: {}", w.name().unwrap_or("_"));
            }
        }
        None => println!("no weapons"),
    }
    println!("-------");
}
