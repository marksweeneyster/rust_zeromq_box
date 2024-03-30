#[allow(dead_code, unused_imports)]
#[path = "../monster_generated.rs"]
mod monster_generated;

pub use monster_generated::my_game::sample::root_as_monster;

pub fn process_message(msg: zeromq::ZmqMessage, topic: &str) {
    println!("topic: {}", topic);
    for w in msg.iter() {
        if w.eq(topic) {
            continue;
        }
        process_monster(w);
    }
}

fn process_monster(buf: &[u8]) {
    let monster = root_as_monster(buf).unwrap();
    // Get and test some scalar types from the FlatBuffer.
    let hp = monster.hp();
    let mana = monster.mana();
    let name = monster.name().unwrap();
    let color = monster.color();

    let pos = monster.pos().unwrap();
    let x = pos.x();
    let y = pos.y();
    let z = pos.z();

    println!("name: {}, mana: {}, hp: {}", name, mana, hp);
    println!("color: {}", color.variant_name().unwrap());
    println!("position: [{},{},{}]", x, y, z);

    let weps = monster.weapons().unwrap();

    for w in weps.iter() {
        println!("weapon: {}", w.name().unwrap());
    }
    println!("-------");
}
