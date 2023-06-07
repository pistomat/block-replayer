set dotenv-load := true

fmt: 
    cargo +nightly fmt --all

clippy: 
    cargo clippy --all --all-features

clippy-fix:
    cargo clippy --all --all-features --fix
