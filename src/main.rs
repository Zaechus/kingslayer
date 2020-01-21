use std::env;

use kingslayer::{Armor, Cli, Enemy, Gold, Item, Weapon};

fn main() {
    if let Some(path) = env::args().nth(1) {
        let cli = Cli::from_file(&path);

        cli.start();
    } else {
        let cli = Cli::from_file("worlds/world.ron");
        cli.add_item(
            "Brig",
            Item::Weapon(Weapon::new("stick", "It's short but stout.", 4)),
        );

        // Hold 2
        cli.spawn_enemy("Hold 2", Enemy::new_rats(true));
        cli.add_item(
            "Hold 2",
            Item::Weapon(Weapon::new(
                "sword",
                "It's a basic short sword with a few knicks.",
                7,
            )),
        );
        cli.add_item(
            "Hold 2",
            Item::Armor(Armor::new(
                "leather armor",
                "It's well worn but reliable.",
                11,
            )),
        );

        // Crew Deck
        cli.spawn_enemy(
            "Crew Deck",
            Enemy::new_pirate(false).with_item(Item::Gold(Gold::new(10))),
        );
        cli.spawn_enemy(
            "Crew Deck",
            Enemy::new_pirate(false)
                .with_item(Item::Weapon(Weapon::new(
                    "cutlass",
                    "It's thick steel cutlass.",
                    8,
                )))
                .with_item(Item::Gold(Gold::new(10))),
        );

        cli.start();
    }
}
