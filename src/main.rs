use std::env;

use kingslayer::Game;

fn main() {
    let game = if let Some(filename) = env::args().nth(1) {
        Game::load(&filename)
    } else {
        Game::from_ron_str(include_str!("worlds/world.ron"))
    };

    game.play();
}
