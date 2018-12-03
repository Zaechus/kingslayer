use super::*;
use item::Item;

#[test]
fn cli_take_drop() {
    let iron_sword = Box::new(Item::new(
        "iron sword",
        "There is an iron sword on the ground.",
    ));
    let capsule = Box::new(Item::new("capsule", "There is a capsule here."));

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
