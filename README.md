<h1 align="center">🦀 Rust Notes CLI</h1>
<p align="center">
A powerful command-line tool to manage notes with tags, search, and export to PDF/CSV/Markdown – built with Rust 💪
</p>

<p align="center">
    <img src="https://img.shields.io/badge/made%20with-Rust-orange?style=for-the-badge" alt="Made with Rust" />
    <img src="https://img.shields.io/badge/cli-app-blueviolet?style=for-the-badge" alt="CLI App" />
    <img src="https://img.shields.io/badge/pdf-csv-md-green?style=for-the-badge" alt="Exports" />
</p>

---

## 📌 About the Project

This is a Rust CLI application that lets you manage personal notes efficiently.

### ✨ Features

- 🏷️ Add notes with **tags**
- 🔍 Search by **content or tag**
- 🕒 Tracks both **created** and **updated** timestamps
- 📄 Export to **PDF**, **CSV**, and **Markdown**
- 📂 Option to **open the PDF file automatically** after export

### 🧠 Built to Practice Rust Concepts

- `struct`, `enum`, and pattern matching
- CLI parsing with [`clap`](https://docs.rs/clap)
- File handling with [`std::fs`]
- JSON serialization with [`serde`]
- Date/time with [`chrono`]
- Modular codebase (`services`, `models`, etc.)

---

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
# note in text file
cargo run -- --add-note "This is a note"
cargo run -- --show-raw-notes
cargo run -- --clear-notes

# note in json file
cargo run -- --note-title "Meeting" --note-body "Call with the team at 5pm"
cargo run -- --show-structured-notes
cargo run -- --delete-note 1
cargo run -- --edit-note 1 --new-title "New Title" --new-body "New Body"

# Add note with tag
cargo run -- --add-note "Meeting" "Discuss project" --tags work

# Search by tag
cargo run -- --search-tag work

# Export
cargo run -- --export-pdf --open-pdf
cargo run -- --export-csv
cargo run -- --export-md
```

### 🚀 Ready to Run

```bash
cargo build
cargo run -- --help
```

### 📁 Project Structure

<pre>
src/
├── models/
│   └── note.rs
├── services/
│   └── notes.rs
├── main.rs
├── args.rs
</pre>

## 📚 Notes

This project was built for educational purposes to explore:

- Rust’s powerful type system
- File and JSON handling
- Command-line parsing with clap
- Clean module-based code organization
