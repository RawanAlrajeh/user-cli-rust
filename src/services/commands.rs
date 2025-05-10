use crate::models::args::Args;
use crate::services::notes::add_note;
use crate::services::{
    notes::{
        clear_notes, delete_note_by_id, read_notes, read_notes_from_json, search_note, write_note,
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

    //  add note with title and body with json file

    // ✅ JSON mode: title + body
    if let (Some(title), Some(body)) = (&args.note_title, &args.note_body) {
        add_note(title, body);
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
