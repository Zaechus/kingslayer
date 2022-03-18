run:
    cargo run --release

t:
    cargo test
    cargo clippy

c:
    cargo fmt
    cargo update

    cargo doc
    cargo build

    cargo test
    cargo clippy
