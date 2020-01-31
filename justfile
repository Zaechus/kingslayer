run:
    cargo run --release

test:
    cargo clippy
    cargo test

c:
    cargo fmt
    cargo update

    cargo clippy
    cargo c

    cargo doc
    cargo build

    cargo test