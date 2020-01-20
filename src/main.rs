use std::env;

use kingslayer::{Cli, Enemy, Item, Weapon};

fn main() {
    if let Some(path) = env::args().nth(1) {
        let cli = Cli::from_ron_file(&path);

        cli.start();
    } else {
        let cli = Cli::from_ron_file("worlds/world.ron");
        cli.spawn_enemy(
            "Dank Tunnel 1",
            Enemy::new_goblin(false).with_item(Item::Weapon(Weapon::new(
                "rusty knife",
                "The knife looks small and a little rusty.",
                3,
            ))),
        );
        for _ in 0..2 {
            cli.spawn_enemy("Dank Tunnel 2", Enemy::new_goblin(true));
        }

        cli.start();
    }
}
