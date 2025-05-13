use chrono::Local;
use genpdf::Element;
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

pub fn add_note(title: &str, body: &str, tags: Option<Vec<String>>) {
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
        tags: tags.unwrap_or_default(),
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

pub fn export_notes_to_md() -> bool {
    let notes = read_notes_from_json();

    let mut content = String::new();

    for note in notes {
        content.push_str(&format!("## 🆔 ID: {}\n", note.id));
        content.push_str(&format!("### 📌 Title: {}\n", note.title));
        content.push_str(&format!("- 🕒 Created: {}\n", note.created_at));
        if let Some(updated) = note.updated_at {
            content.push_str(&format!("- 🔄 Updated: {}\n", updated));
        }
        if !note.tags.is_empty() {
            content.push_str(&format!("- 🏷️ Tags: {}\n", note.tags.join(", ")));
        }
        content.push_str(&format!("\n**Note:**\n{}\n", note.body));
        content.push_str("\n---\n\n");
    }

    let mut file = match std::fs::File::create("notes.md") {
        Ok(f) => f,
        Err(_) => return false,
    };

    use std::io::Write;
    file.write_all(content.as_bytes()).is_ok()
}

pub fn export_notes_to_csv() -> bool {
    let notes = read_notes_from_json();

    let path = std::env::current_dir().unwrap().join("notes.csv");

    let mut file = match std::fs::File::create(&path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    use std::io::Write;

    // add BOM (UTF-8)
    let bom = b"\xEF\xBB\xBF";
    if file.write_all(bom).is_err() {
        return false;
    }

    // write the header
    if writeln!(file, "id,title,body,created_at,updated_at,tags").is_err() {
        return false;
    }

    for note in notes {
        let updated = note.updated_at.unwrap_or_default();
        let tags = note.tags.join(", ");
        // write the line, with the escape if there is a comma
        let safe_title = note.title.replace(",", ";");
        let safe_body = note.body.replace(",", ";");

        let row = format!(
            "{},{},{},{},{},{}\n",
            note.id, safe_title, safe_body, note.created_at, updated, tags
        );

        if file.write_all(row.as_bytes()).is_err() {
            return false;
        }
    }

    true
}

pub fn export_notes_to_pdf() -> bool {
    let notes = read_notes_from_json();

    // ✅ حمل الخط من ملف ttf
    let font_family = genpdf::fonts::from_files("assets", "Amiri", None)
    .expect("❌ Failed to load font family from assets/");

    let mut doc = genpdf::Document::new(font_family);

    for note in notes {
        let mut section = genpdf::elements::LinearLayout::vertical();

        section.push(
            genpdf::elements::Paragraph::new(format!("🆔 ID: {}", note.id))
                .styled(genpdf::style::Style::new().bold()),
        );
        section.push(genpdf::elements::Paragraph::new(format!(
            "📌 {}",
            note.title
        )));
        section.push(genpdf::elements::Paragraph::new(format!(
            "📝 {}",
            note.body
        )));
        section.push(genpdf::elements::Paragraph::new(format!(
            "🕒 Created: {}",
            note.created_at
        )));

        if let Some(updated) = &note.updated_at {
            section.push(genpdf::elements::Paragraph::new(format!(
                "🔄 Updated: {}",
                updated
            )));
        }

        if !note.tags.is_empty() {
            section.push(genpdf::elements::Paragraph::new(format!(
                "🏷️ Tags: {}",
                note.tags.join(", ")
            )));
        }

        section.push(genpdf::elements::Break::new(1));
        section.push(genpdf::elements::Break::new(1));

        doc.push(section);
    }

    match doc.render_to_file("notes.pdf") {
        Ok(_) => true,
        Err(e) => {
            println!("❌ PDF generation error: {}", e);
            false
        }
    }
}
