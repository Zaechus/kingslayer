# 👑 Kingslayer ⚔️

[![Rust](https://github.com/Maxgy/kingslayer/workflows/Rust/badge.svg)](https://github.com/Maxgy/kingslayer/actions)
[![Crates.io](https://img.shields.io/crates/v/kingslayer)](https://crates.io/crates/kingslayer)
[![Kingslayer documentation](https://docs.rs/kingslayer/badge.svg)](https://docs.rs/kingslayer)
![license/MIT](https://img.shields.io/github/license/Maxgy/kingslayer)

Kingslayer is a text-based dungeon crawler written in Rust. It is a continuation of [thekinggame](https://github.com/Maxgy/thekinggame).

### Playing the game

You can play the online WASM version here: [maxgy.github.io/kingslayer-web](https://maxgy.github.io/kingslayer-web/)

You can also install Kingslayer:
```
cargo install kingslayer
kingslayer
```

or clone the project and run:
```
cargo run --release
```

### Creating and Running your own World

Worlds can be created with RON and Rust helper functions. Running the world on the command line looks like this:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_file("worlds/world.ron");

    cli.start();
}
```
or the loop can be managed manually like this:
```rust
use kingslayer::Cli;

fn main() {
    let cli = Cli::from_file("worlds/world.ron");

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
This method allows for other forms of input and output such as within a website. The content for the world can also be passed as a raw string with `Cli::from_ron_str`.

### Dependencies
* Rust ^1.41.0

### Crates
* rand = "0.7"
* rayon = "1.0"
* serde = "1.0"
* ron = "0.5"
