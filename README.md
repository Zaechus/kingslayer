# Kingslayer

[![Build Status](https://travis-ci.com/Maxgy/kingslayer.svg?branch=master)](https://travis-ci.com/Maxgy/kingslayer)
[![Build status](https://ci.appveyor.com/api/projects/status/b5p7b1efiy9t0fm7/branch/master?svg=true)](https://ci.appveyor.com/project/Maxgy/kingslayer/branch/master)
[![Current Crates.io Version](https://img.shields.io/crates/v/kingslayer)](https://crates.io/crates/kingslayer)

Kingslayer is a text adventure dungeon crawler game written in Rust. It is a rewrite and continuation of [thekinggame](https://github.com/Maxgy/thekinggame).

You can find the WASM package at [github.com/Maxgy/kingslayer-wasm](https://github.com/Maxgy/kingslayer-wasm)

### Running the game

You can play the online WASM version here: [maxgy.github.io/server-kingslayer](https://maxgy.github.io/kingslayer-wasm/)

or clone the project and run:
```
$ cargo run --release
```

### Creating and Running your own World

Worlds are defined with JSON. An example can be found on the [wiki](https://github.com/Maxgy/kingslayer/wiki/Example-world-JSON-file). Deploying the world to the command line looks like this:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_json_file("data/world.json");

    cli.start();
}
```
or the loop can be managed manually like this:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_json_file("data/world.json");

    println!("{}", cli.ask("l"));
    loop {
        let s = cli.ask(&Cli::prompt());
        println!("{}", s);
        if s.contains("You died.") {
            break;
        }
    }
}
```
This method allows you to manage other forms of input and output such as within a website. The JSON content for the world can also be passed as a raw string with `Cli::from_json_str`.

### Dependencies
* Rustc and Cargo >= 1.37.0