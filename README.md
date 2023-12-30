# Minigrep

A simple grep-like program written in Rust for learning purposes.

## Usage

```bash
cargo run -- --insensitive-case --file poem.txt --query the
or
cargo run -- -i -f poem.txt -q the
```

## Building

To build this app for prod:

```bash
cargo build --release
```
