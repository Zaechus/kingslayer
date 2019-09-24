run:
    cargo run --release

build:
    cargo build

c:
    cargo fmt
    cargo update

    cargo clippy
    cargo c

    cargo doc
    cargo build

    cargo bench
    cargo test
