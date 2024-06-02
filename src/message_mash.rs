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