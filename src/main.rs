extern crate kinggame1d;

use kinggame1d::cli::Cli;
use kinggame1d::room::Room;

fn main() {
    let rooms: Vec<Room> = Vec::new();
    let cli = Cli::new(rooms);
    cli.start();
}
