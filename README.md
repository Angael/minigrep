# Minigrep

A simple grep-like program written in Rust for learning purposes.

## Usage

```bash
cargo run -- the poem.txt
```

### To search for a case-insensitive string:

Linux:

```bash
IGNORE_CASE=1 cargo run -- to poem.txt
```

Windows:

```powershell
$Env:IGNORE_CASE=1; cargo run -- to poem.txt
```

## Building

To build this app for prod:

```bash
cargo build --release
```
