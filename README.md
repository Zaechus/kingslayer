# 👑 Kingslayer ⚔️

[![CI](https://github.com/Zaechus/kingslayer/actions/workflows/ci.yml/badge.svg)](https://github.com/Zaechus/kingslayer/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/kingslayer)](https://crates.io/crates/kingslayer)
[![Kingslayer documentation](https://docs.rs/kingslayer/badge.svg)](https://docs.rs/kingslayer)

Kingslayer is a text-based adventure game and library written in Rust.

## Playing the game

You can play online here: [zaechus.github.io/kingslayer-web](https://zaechus.github.io/kingslayer-web/)

or install Kingslayer:
```sh
cargo install kingslayer
kingslayer
```
or clone the project and run it:
```sh
git clone https://github.com/Zaechus/kingslayer
cd kingslayer
cargo run --release
```

## Making a game

You can make a world like [world.ron](https://github.com/Zaechus/kingslayer/blob/main/src/world.ron) and run it directly with kingslayer:
```sh
kingslayer custom_world.ron
```
or in a separate program with the kingslayer library:
```rust
use kingslayer::*;

fn main() {
    let mut game: Game = include_str!("world.ron").parse().unwrap();

    game.play().unwrap();
}
```

Alternatively, you can manually handle input and output with the `ask` method ([kingslayer-web example](https://github.com/Zaechus/kingslayer-web/blob/main/docs/index.js)).
