use std::fs::{read_to_string, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};

pub fn write_note(note: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("notes.txt")
        .expect("Failed to open or create notes file");

    writeln!(file, "{}", note.trim()).expect("Failed to write note");
}

pub fn read_notes() -> Result<String, String> {
    read_to_string("notes.txt").map_err(|e| e.to_string())
}

pub fn clear_notes() -> Result<(), String> {
    File::create("notes.txt")
        .map(|_| ())
        .map_err(|_| "Failed to clear notes".to_string())
}

pub fn search_note(query: &str) -> Result<String, String> {
    let file = File::open("notes.txt").map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let matching_lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.contains(query))
        .collect();

    if matching_lines.is_empty() {
        Ok("⚠️ No matching notes found".to_string())
    } else {
        Ok(matching_lines.join("\n"))
    }
}
