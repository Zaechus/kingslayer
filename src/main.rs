use std::{env, error, process};

use kingslayer::*;

fn main() {
    if let Err(err) = try_main() {
        println!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn error::Error>> {
    let mut game = if let Some(filename) = env::args().nth(1) {
        Game::restore(&filename)?
    } else {
        dev_game()
    };

    game.play()?;

    Ok(())
}

// TODO
fn dev_game() -> Game {
    Game::new("field")
        .with("field", Thing::new("You are in a field."))
        .with("cave", Thing::new("You are in a cave."))
        .with(
            "field hole",
            Thing::default()
                .with_name("hole")
                .with_name("n")
                .with_location("field")
                .with_dest("cave"),
        )
        .with(
            "cave hole",
            Thing::default()
                .with_name("hole")
                .with_name("s")
                .with_location("cave")
                .with_dest("field"),
        )
        .with(
            "iron sword",
            Thing::new("There is an iron sword on the ground.")
                .with_name("iron sword")
                .with_location("cave"),
        )
}
