run:
    cargo run --release

test:
    cargo clippy
    cargo bench
    cargo test

c:
    cargo fmt
    cargo update

    cargo clippy
    cargo c

    cargo doc
    cargo build

    cargo bench
    cargo test

docs:
    cargo doc --open
