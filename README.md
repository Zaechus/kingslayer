# Kingslayer

[![Build Status](https://travis-ci.com/Maxgy/kingslayer.svg?branch=master)](https://travis-ci.com/Maxgy/kingslayer)

A text adventure dungeon crawler game written in Rust.

### Creating and Running a World

Worlds are defined with JSON. An example can be found on the ![wiki](https://github.com/Maxgy/kingslayer/wiki/Example-world-JSON-file).  Deploying the world file in Rust looks like this:
```
use kingslayer::get_world;

fn main() {
    let cli = kingslayer::get_world("data/world.json");

    println!("{}", cli.ask("l"));
    loop {
        match cli.ask(&cli.prompt()) {
            s => {
                println!("{}", s);
                if s.contains("You died.") {
                    break;
                }
            }
        }
    }
}
```