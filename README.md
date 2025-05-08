# 🦀 Rust User CLI

A simple command-line app built in Rust for practicing core concepts like:

- `struct` and `enum`
- `Option` and `Result` handling
- Pattern matching
- Modular code structure (`models`, `database`, `services`)
- Command-line argument parsing with [`clap`](https://docs.rs/clap/latest/clap/)
- File operations with [`std::fs`](https://doc.rust-lang.org/std/fs/index.html)

## 📦 How to Run

Make sure you have Rust installed, then run:

### 👤 User Commands

```bash
cargo run -- --find Rawan
cargo run -- --status Hadeel
cargo run -- --email Nada
cargo run -- --list
```

### 📝 Notes Commands

```bash
cargo run -- --add_note "This is a note"
cargo run -- --show_notes
cargo run -- --clear_notes
cargo run -- --search_note "note"
```

## 📚 Notes

- This project was built for educational purposes to explore Rust’s CLI capabilities, modular code structure, and file handling using safe and idiomatic Rust.
