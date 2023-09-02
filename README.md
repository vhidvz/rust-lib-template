# rust-lib-template

Rust library template

```sh
rustup component add clippy
rustup component add rustfmt

# .cargo/config.toml [alias]
cargo lint
cargo format

# Debugging & Code-Coverage
rustup component add llvm-tools
```

```sh
CARGO_INCREMENTAL=0 RUSTFLAGS='-Cinstrument-coverage' LLVM_PROFILE_FILE='target/cargo-test.profraw' cargo test
# https://blog.rng0.io/how-to-do-code-coverage-in-rust
grcov . --binary-path ./target/debug/deps/ -s . -t html --branch --ignore-not-existing --ignore '../*' --ignore "/*" -o docs/coverage
```
