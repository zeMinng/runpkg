set windows-shell := ["powershell.exe", "-NoProfile", "-Command"]

test_path := justfile_directory() + "/../test"

# Development
dev *args="":
    cargo watch -c -x "run -- {{args}}"

test-run cmd="":
    cargo watch -c -x "run -- -p {{test_path}} {{cmd}}"

scripts:
    just test-run scripts

deps:
    just test-run deps

doctor:
    just test-run doctor

# Help
help:
    cargo run -- --help

help-cmd cmd:
    cargo run -- {{cmd}} --help

# Rust
check:
    cargo check

test:
    cargo test

fmt:
    cargo fmt

fmt-check:
    cargo fmt -- --check

lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Build
build:
    cargo build

release:
    cargo build --release