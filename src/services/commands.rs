use crate::models::args::Args;
use crate::services::notes::add_note;
use crate::services::{
    notes::{
        clear_notes, delete_note_by_id, edit_note_by_id, export_notes_to_csv, export_notes_to_md,
        export_notes_to_pdf, read_notes, read_notes_from_json, search_note, write_note,
    },
    user::{find_user_by_name, get_all_users, get_email_by_name, get_status_by_name},
};

pub fn handle_user_commands(args: &Args) {
    if let Some(name) = &args.find {
        match find_user_by_name(name) {
            Ok(user) => println!("👤 User found: {}", user.name),
            Err(e) => println!("❌ Error: {}", e),
        }
    }

    if let Some(name) = &args.status {
        match get_status_by_name(name) {
            Some(status) => println!("📌 Status: {:?}", status),
            None => println!("⚠️ Status not found"),
        }
    }

    if let Some(name) = &args.email {
        match get_email_by_name(name) {
            Some(email) => println!("📧 Email: {}", email),
            None => println!("⚠️ Email not found"),
        }
    }

    if args.list {
        let users = get_all_users();
        for user in users {
            let email = user.email.unwrap_or_else(|| "no email".to_string());
            println!("👥 {} | {:?} | {}", user.name, user.status, email);
        }
    }
}

pub fn handle_note_commands(args: &Args) {
    // add note with content with text file

    // ✅ Simple mode: just content
    if let Some(note) = &args.add_note {
        write_note(note);
        println!("📝 Simple note added!");
    }

    if args.show_raw_notes {
        match read_notes() {
            Ok(content) => println!("📄 Raw Notes:\n{}", content),
            Err(e) => println!("❌ Error reading raw notes: {}", e),
        }
    }

    // ✅ JSON mode: title + body

    //  add note with title and body with json file
    if let (Some(title), Some(body)) = (&args.note_title, &args.note_body) {
        add_note(title, body, args.tags.clone());
        println!("📝 Structured note added!");
        return;
    }

    if args.show_structured_notes {
        let notes = read_notes_from_json();

        if notes.is_empty() {
            println!("📭 No structured notes found.");
        } else {
            for note in notes {
                println!("🆔 ID: {}", note.id);
                println!("📌 {} - {}", note.title, note.created_at);
                println!("📝 {}", note.body);
                if let Some(updated) = &note.updated_at {
                    println!("🔄 Last updated: {}", updated);
                }
                if !note.tags.is_empty() {
                    println!("🏷️ Tags: {}", note.tags.join(", "));
                }
                println!("-------------------------------------");
            }
        }
    }

    if args.clear_notes {
        match clear_notes() {
            Ok(_) => println!("🧹 Notes cleared!"),
            Err(e) => println!("❌ {}", e),
        }
    }

    if let Some(keyword) = &args.search_note {
        println!("🔍 Search results for '{}':", keyword);
        match search_note(keyword) {
            Ok(content) => println!("{}", content),
            Err(e) => println!("❌ {}", e),
        }
    }
    if let Some(id) = &args.delete_note {
        let deleted = delete_note_by_id(*id);
        if deleted {
            println!("🗑️ Note deleted!");
        } else {
            println!("❌ Note not found");
        }
    }
    if let Some(id) = args.edit_note {
        let updated = edit_note_by_id(id, args.new_title.clone(), args.new_body.clone());
        if updated {
            println!("✏️ Note with ID {} updated!", id);
        } else {
            println!("⚠️ No note found with ID {}", id);
        }
    }

    if let Some(tag) = &args.filter_by_tag {
        let notes = read_notes_from_json();
        let filtered: Vec<_> = notes
            .into_iter()
            .filter(|note| {
                note.tags
                    .iter()
                    .any(|t| t.to_lowercase() == tag.to_lowercase())
            })
            .collect();

        if filtered.is_empty() {
            println!("📭 No notes found with tag '{}'.", tag);
        } else {
            println!("🔎 Notes tagged with '{}':", tag);
            for note in filtered {
                println!("🆔 ID: {}", note.id);
                println!("📌 {} - {}", note.title, note.created_at);
                println!("📝 {}", note.body);

                if let Some(updated) = &note.updated_at {
                    println!("🔄 Last updated: {}", updated);
                }

                if !note.tags.is_empty() {
                    println!("🏷️ Tags: {}", note.tags.join(", "));
                }

                println!("-------------------------------------");
            }
        }
    }

    if args.export_md {
        if export_notes_to_md() {
            println!("📤 Notes exported to notes.md!");
        } else {
            println!("❌ Failed to export notes.");
        }
    }

    if args.export_csv {
        if export_notes_to_csv() {
            println!("📤 Notes exported to notes.csv!");
        } else {
            println!("❌ Failed to export notes.");
        }
    }

    if args.export_pdf {
        if export_notes_to_pdf() {
            println!("📄 Notes exported to notes.pdf!");
        } else {
            println!("❌ Failed to export PDF.");
        }
    }

    if args.export_pdf {
        let success = export_notes_to_pdf();
        if success {
            println!("✅ PDF exported to notes.pdf");

            if args.open_pdf {
                #[cfg(target_os = "macos")]
                {
                    std::process::Command::new("open")
                        .arg("notes.pdf")
                        .spawn()
                        .expect("❌ Failed to open PDF file");
                }

                #[cfg(target_os = "windows")]
                {
                    std::process::Command::new("cmd")
                        .args(["/C", "start", "notes.pdf"])
                        .spawn()
                        .expect("❌ Failed to open PDF file");
                }

                #[cfg(target_os = "linux")]
                {
                    std::process::Command::new("xdg-open")
                        .arg("notes.pdf")
                        .spawn()
                        .expect("❌ Failed to open PDF file");
                }
            }
        } else {
            println!("❌ Failed to export PDF");
        }
    }
}

// pub fn run_user() {
//     let user = find_user_by_name("Hadeel");

//     match user {
//         Ok(user) => println!("User found: {}", user.name),
//         Err(e) => println!("Error: {}", e),
//     }

//     let status = get_status_by_name("Rawan");
//     match status {
//         Some(status) => println!("Status: {:?}", status),
//         None => println!("Status not found"),
//     }

//     let email = get_email_by_name("Hadeel");
//     match email {
//         Some(email) => println!("Email: {}", email),
//         None => println!("Email not found"),
//     }
// }
