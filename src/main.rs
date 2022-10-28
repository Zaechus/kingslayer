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
        Game::load(&filename)?
    } else {
        include_str!("dev.ron").parse()?
        // include_str!("world.ron").parse()?
    };

    game.play()?;

    Ok(())
}
