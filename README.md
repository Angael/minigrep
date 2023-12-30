# Minigrep

A simple grep-like program written in Rust for learning purposes.

## Usage

```bash
cargo run -- the poem.txt
```

To search for a case-insensitive string:

```bash
IGNORE_CASE=1 cargo run -- the poem.txt
```

## Building

To build this app for production:

```bash
cargo build --release
```
