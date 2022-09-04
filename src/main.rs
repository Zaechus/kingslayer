use std::{env, io, process};

use kingslayer::Game;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("error: {}", e);
        process::exit(e.raw_os_error().unwrap_or(1));
    }
}

fn try_main() -> io::Result<()> {
    let mut game = if let Some(filename) = env::args().nth(1) {
        Game::load(&filename)?
    } else {
        Game::from_ron_str(include_str!("worlds/world.ron"))
    };

    game.play();
    Ok(())
}
