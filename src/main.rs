use std::{collections::HashMap, env, error, process};

use kingslayer::*;

fn main() {
    if let Err(e) = try_main() {
        println!("{}", e);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn error::Error>> {
    let mut game = if let Some(filename) = env::args().nth(1) {
        Game::restore(&filename)?
    } else {
        tmp()
    };

    game.play()?;

    Ok(())
}

fn tmp() -> Game {
    let mut rooms = HashMap::new();
    rooms.insert(
        "1,1,1".to_owned(),
        Room::new("Green Room", "You are in a green room."),
    );

    Game::new(Player::new("1,1,1"), rooms)
}
