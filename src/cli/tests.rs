use super::*;
use item::Item;

#[test]
fn cli_take_drop() {
    let iron_sword = Box::new(Item::new(
        "iron sword",
        "There is an iron sword on the ground.",
        "The iron sword seems kind of rusty.",
    ));
    let capsule = Box::new(Item::new(
        "capsule",
        "There is a capsule here.",
        "The capsule is hollow and can hold things.",
    ));

    let mut sandbox_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    sandbox_room_objs.insert(iron_sword.name(), iron_sword);
    sandbox_room_objs.insert(capsule.name(), capsule);

    let sandbox_room = Box::new(Room::new(
        "Sandbox Room",
        "You stand in a large box filled with sand.",
        sandbox_room_objs,
    ));
    let mut rooms: HashMap<String, Box<Room>> = HashMap::new();
    rooms.insert(sandbox_room.name(), sandbox_room);

    let cli = Cli::new("Sandbox Room", rooms);

    assert_eq!(cli.inventory(), "You are empty-handed.");
    assert!(
        cli.world.borrow().look().contains("iron sword")
            && cli.world.borrow().look().contains("capsule")
    );

    cli.take("iron sword");
    assert_eq!(cli.inventory(), "You are carrying:\n  iron sword");
    assert_eq!(
        cli.world.borrow().look(),
        format!(
            "{}\n{}\n{}",
            "Sandbox Room",
            "You stand in a large box filled with sand.",
            "There is a capsule here."
        )
    );

    cli.take("capsule");
    assert!(cli.inventory().contains("iron sword") && cli.inventory().contains("capsule"));
    assert_eq!(
        cli.world.borrow().look(),
        format!(
            "{}\n{}",
            "Sandbox Room", "You stand in a large box filled with sand.",
        )
    );

    cli.drop("iron sword");
    assert_eq!(cli.inventory(), "You are carrying:\n  capsule");
    assert_eq!(
        cli.world.borrow().look(),
        format!(
            "{}\n{}\n{}",
            "Sandbox Room",
            "You stand in a large box filled with sand.",
            "There is an iron sword on the ground.",
        )
    );

    cli.drop("capsule");
    assert_eq!(cli.inventory(), "You are empty-handed.");
    assert!(
        cli.world.borrow().look().contains("iron sword")
            && cli.world.borrow().look().contains("capsule")
    );
}

#[test]
fn cli_take_all_drop_all() {
    let big_item = Box::new(Item::new(
        "big item",
        "There is a big item on the ground.",
        "Meh, it's just some random thing.",
    ));
    let thingy = Box::new(Item::new(
        "thingy",
        "There is a thingy here.",
        "What even is that??",
    ));

    let mut sandbox_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    sandbox_room_objs.insert(big_item.name(), big_item);
    sandbox_room_objs.insert(thingy.name(), thingy);

    let sandbox_room = Box::new(Room::new(
        "Sandbox Room",
        "You stand in a large box filled with sand.",
        sandbox_room_objs,
    ));
    let mut rooms: HashMap<String, Box<Room>> = HashMap::new();
    rooms.insert(sandbox_room.name(), sandbox_room);

    let cli = Cli::new("Sandbox Room", rooms);

    assert!(
        cli.world.borrow().look().contains("thingy")
            && cli.world.borrow().look().contains("big item")
    );
    cli.take_all();
    assert!(cli.inventory().contains("thingy") && cli.inventory().contains("big item"));

    cli.drop_all();
    assert!(
        cli.world.borrow().look().contains("thingy")
            && cli.world.borrow().look().contains("big item")
    );
    assert_eq!(cli.inventory(), "You are empty-handed.");
}

#[test]
fn cli_put_in() {
    let big_item_contents: HashMap<String, Box<Item>> = HashMap::new();
    let big_item = Box::new(Item::new_container(
        "big item",
        "There is a big item on the ground.",
        "Meh, it's just some random thing.",
        Some(big_item_contents),
    ));
    let thingy = Box::new(Item::new(
        "thingy",
        "There is a thingy here.",
        "What even is that??",
    ));

    let mut sandbox_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    sandbox_room_objs.insert(big_item.name(), big_item);
    sandbox_room_objs.insert(thingy.name(), thingy);

    let sandbox_room = Box::new(Room::new(
        "Sandbox Room",
        "You stand in a large box filled with sand.",
        sandbox_room_objs,
    ));
    let mut rooms: HashMap<String, Box<Room>> = HashMap::new();
    rooms.insert(sandbox_room.name(), sandbox_room);

    let cli = Cli::new("Sandbox Room", rooms);

    assert!(
        cli.world.borrow().look().contains("thingy")
            && cli.world.borrow().look().contains("big item")
    );
    cli.take("thingy");
    assert!(cli.inventory().contains("thingy"));

    cli.put_in("thingy", "big item");
    assert!(
        cli.world.borrow().look().contains("thingy")
            && cli.world.borrow().look().contains("big item")
            && cli.world.borrow().look().contains("contains")
    );
    assert_eq!(cli.inventory(), "You are empty-handed.");
}

#[test]
fn cli_inspect() {
    let thingy = Box::new(Item::new(
        "thingy",
        "There is a thingy here.",
        "What even is that??",
    ));

    let mut sandbox_room_objs: HashMap<String, Box<Item>> = HashMap::new();
    sandbox_room_objs.insert(thingy.name(), thingy);

    let sandbox_room = Box::new(Room::new(
        "Sandbox Room",
        "You stand in a large box filled with sand.",
        sandbox_room_objs,
    ));
    let mut rooms: HashMap<String, Box<Room>> = HashMap::new();
    rooms.insert(sandbox_room.name(), sandbox_room);

    let cli = Cli::new("Sandbox Room", rooms);

    assert!(cli.world.borrow().look().contains("thingy"));
    assert_eq!(cli.inspect("thingy"), "What even is that??");
    cli.take("thingy");
    assert!(cli.inventory().contains("thingy"));
    assert_eq!(cli.inspect("thingy"), "What even is that??");
}
