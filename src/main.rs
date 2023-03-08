use std::{env, error, fs, process};

use kingslayer::*;

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", err);
        process::exit(1);
    }
}

fn try_main() -> Result<(), Box<dyn error::Error>> {
    let mut game: Game = if let Some(filename) = env::args().nth(1) {
        fs::read_to_string(filename)?.parse()?
    } else {
        include_str!("world.ron").parse()?
    };

    game.play()?;

    Ok(())
}
