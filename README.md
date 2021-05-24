# 👑 Kingslayer ⚔️

[![Linux](https://github.com/Zaechus/kingslayer/workflows/Linux/badge.svg)](https://github.com/Zaechus/kingslayer/actions?query=workflow%3ALinux)
[![Windows](https://github.com/Zaechus/kingslayer/workflows/Windows/badge.svg)](https://github.com/Zaechus/kingslayer/actions?query=workflow%3AWindows)
[![Mac](https://github.com/Zaechus/kingslayer/workflows/Mac/badge.svg)](https://github.com/Zaechus/kingslayer/actions?query=workflow%3AMac)
[![Crates.io](https://img.shields.io/crates/v/kingslayer)](https://crates.io/crates/kingslayer)
[![Kingslayer documentation](https://docs.rs/kingslayer/badge.svg)](https://docs.rs/kingslayer)
[![Run on Repl.it](https://repl.it/badge/github/Zaechus/kingslayer)](https://repl.it/github/Zaechus/kingslayer)

Kingslayer is a text-based dungeon crawler written in Rust. It is a continuation of [thekinggame](https://github.com/Zaechus/thekinggame).

### Playing the game

You can play the online WASM version here: [zaechus.github.io/kingslayer-web](https://zaechus.github.io/kingslayer-web/)

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
* Rust ^1.52.0

### Crates
* rand = "0.8"
* rayon = "1.5"
* serde = "1.0"
* ron = "0.6"
