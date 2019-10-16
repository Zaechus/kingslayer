# Kingslayer

[![Current Crates.io Version](https://img.shields.io/crates/v/kingslayer)](https://crates.io/crates/kingslayer)
[![Kingslayer documentation](https://docs.rs/kingslayer/badge.svg)](https://docs.rs/kingslayer)
[![Build Status](https://travis-ci.com/Maxgy/kingslayer.svg?branch=master)](https://travis-ci.com/Maxgy/kingslayer)
[![Build status](https://ci.appveyor.com/api/projects/status/b5p7b1efiy9t0fm7/branch/master?svg=true)](https://ci.appveyor.com/project/Maxgy/kingslayer/branch/master)

Kingslayer is a text-based dungeon crawler written in Rust. It is a continuation of [thekinggame](https://github.com/Maxgy/thekinggame).

You can find the WASM package at [github.com/Maxgy/kingslayer-wasm](https://github.com/Maxgy/kingslayer-wasm)

### Running the game

You can play the online WASM version here: [maxgy.github.io/kingslayer-wasm](https://maxgy.github.io/kingslayer-wasm/)

or clone the project and run:
```
$ cargo run --release
```

### Creating and Running your own World

Worlds can be created with RON or JSON. Running the world on the command line looks like this:
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
This method allows you to manage other forms of input and output such as within a website. The content for the world can also be passed as a raw string with `Cli::from_ron_str` or `Cli::from_json_str`.

### Dependencies
* Rust ^1.38.0

### Crates
* rand = "0.7"
* rayon = "1.0"
* serde = "1.0"
* ron = "0.5"
* serde_json = "1.0"