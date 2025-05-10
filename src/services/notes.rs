use chrono::Local;
use std::fs::{read_to_string, File, OpenOptions};
use std::io::Read;
use std::io::{BufRead, BufReader, Write};

use crate::models::note::Note;

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
pub fn read_notes_from_json() -> Vec<Note> {
    let full_path = std::env::current_dir().unwrap().join("notes.json");

    let file = std::fs::OpenOptions::new().read(true).open(&full_path);

    if let Ok(mut file) = file {
        let mut content = String::new();
        use std::io::Read;
        file.read_to_string(&mut content)
            .expect("❌ Failed to read notes file");

        if content.trim().is_empty() {
            return vec![];
        }

        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
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

pub fn add_note(title: &str, body: &str) {
    let path = std::env::current_dir().unwrap().join("notes.json");

    // read notes from json file
    let mut notes: Vec<Note> = if let Ok(mut file) = OpenOptions::new().read(true).open(&path) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("❌ Failed to read file");

        if content.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str(&content).unwrap_or_else(|_| vec![])
        }
    } else {
        vec![]
    };

    println!("📦 Existing notes: {}", notes.len());

    // calculate the new id
    let next_id = if let Some(last) = notes.last() {
        last.id + 1
    } else {
        1
    };

    let new_note = Note {
        id: next_id,
        title: title.to_string(),
        body: body.to_string(),
        created_at: Local::now().to_string(),
        updated_at: None,
    };

    notes.push(new_note);

    let json = serde_json::to_string_pretty(&notes).expect("Failed to serialize notes");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&path)
        .expect("Failed to open notes.json");

    file.write_all(json.as_bytes())
        .expect("Failed to write notes");
}

pub fn delete_note_by_id(target_id: u32) -> bool {
    let path = std::env::current_dir().unwrap().join("notes.json");

    let notes: Vec<Note> = if let Ok(mut file) = OpenOptions::new().read(true).open(&path) {
        let mut content = String::new();
        file.read_to_string(&mut content)
            .expect("❌ Failed to read file");

        if content.trim().is_empty() {
            return false;
        }

        serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    } else {
        return false;
    };

    let filtered_notes: Vec<Note> = notes
        .clone()
        .into_iter()
        .filter(|note| note.id != target_id)
        .collect();

    if filtered_notes.len() == notes.len() {
        return false; // nothing deleted
    }

    let json = serde_json::to_string_pretty(&filtered_notes).expect("Failed to serialize notes");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
        .expect("Failed to open file for deletion");

    file.write_all(json.as_bytes())
        .expect("Failed to write file");
    true
}

pub fn edit_note_by_id(id: u32, new_title: Option<String>, new_body: Option<String>) -> bool {
    let path = std::env::current_dir().unwrap().join("notes.json");

    let mut notes: Vec<Note> =
        if let Ok(mut file) = std::fs::OpenOptions::new().read(true).open(&path) {
            let mut content = String::new();
            use std::io::Read;
            file.read_to_string(&mut content)
                .expect("Failed to read file");

            if content.trim().is_empty() {
                return false;
            }

            serde_json::from_str(&content).unwrap_or_else(|_| vec![])
        } else {
            return false;
        };

    let mut updated = false;

    for note in notes.iter_mut() {
        if note.id == id {
            if let Some(title) = new_title {
                note.title = title;
            }
            if let Some(body) = new_body {
                note.body = body;
            }
            note.updated_at = Some(Local::now().to_string());
            updated = true;
            break;
        }
    }

    if updated {
        let json = serde_json::to_string_pretty(&notes).expect("Failed to serialize");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
            .expect("Failed to open notes.json");
        file.write_all(json.as_bytes())
            .expect("Failed to write notes");
    }

    updated
}
