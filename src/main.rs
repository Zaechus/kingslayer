use std::env;

use kingslayer::{Armor, Cli, Element, Enemy, Gold, Item, Weapon};

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

        let root_beer = Element::new(
            "root beer barrel barrels",
            "You can detect the slight scent of root beer.",
            "You can find no way to open the barrels, but there is definitely root beer inside.",
        );
        cli.add_element("Hold 1", root_beer.clone());
        cli.add_element("Hold 2", root_beer);

        // Hold 2
        cli.spawn_enemy("Hold 2", Enemy::new_rats(true));
        cli.add_item(
            "Hold 2",
            Item::Weapon(Weapon::new(
                "sword",
                "It's a basic short sword with a few knicks.",
                6,
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
            "Crew Deck 1",
            Enemy::new_pirate(false).with_item(Item::Gold(Gold::new(10))),
        );
        cli.spawn_enemy(
            "Crew Deck 1",
            Enemy::new_pirate(false)
                .with_item(Item::Weapon(Weapon::new(
                    "cutlass",
                    "It has thick steel and many notches.",
                    8,
                )))
                .with_item(Item::Gold(Gold::new(10))),
        );

        // Cannon Deck
        cli.spawn_enemy(
            "Cannon Deck 1",
            Enemy::new_pirate(true).with_item(Item::Gold(Gold::new(10))),
        );

        // Crows Nest
        cli.spawn_enemy(
            "Crows Nest",
            Enemy::new_pirate(false).with_item(Item::Gold(Gold::new(10))),
        );

        cli.start();
    }
}
