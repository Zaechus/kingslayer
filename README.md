# Kingslayer

[![Build Status](https://travis-ci.com/Maxgy/kingslayer.svg?branch=master)](https://travis-ci.com/Maxgy/kingslayer)

Kingslayer is a text adventure dungeon crawler game written in Rust. It is a rewrite and continuation of [thekinggame](https://github.com/Maxgy/thekinggame).

### Running the game
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
This method allows you to manage other forms of input and output such as within a website.

### Dependencies
* Rustc and Cargo >= 1.37.0