# 🦀 Rust User CLI

A simple command-line app built in Rust for practicing core concepts like:

- `struct` and `enum`
- `Option` and `Result` handling
- Pattern matching
- Modular code structure (`models`, `database`, `services`)
- Command-line argument parsing with [`clap`](https://docs.rs/clap/latest/clap/)
- File operations with [`std::fs`](https://doc.rust-lang.org/std/fs/index.html)
- JSON serialization with [`serde`](https://docs.rs/serde/latest/serde/)

## 📦 How to Run

Make sure you have Rust installed, then run:

```bash
cargo run -- [OPTIONS]
```

### 👤 User Commands

```bash
cargo run -- --find Rawan
cargo run -- --status Hadeel
cargo run -- --email Nada
cargo run -- --list
```

### 📝 Notes Commands

```bash
# note in text file
cargo run -- --add-note "This is a note"
cargo run -- --show-raw-notes
cargo run -- --clear-notes

# note in json file
cargo run -- --note-title "Meeting" --note-body "Call with the team at 5pm"
cargo run -- --show-structured-notes
cargo run -- --delete-note 1
```

## 📚 Notes

This project was built for educational purposes to explore:
- Rust’s powerful type system
- File and JSON handling
- Command-line parsing with clap
- Clean module-based code organization
