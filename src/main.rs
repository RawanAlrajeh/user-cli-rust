mod database;
mod models;
mod services;
use crate::models::args::Args;
use crate::services::notes::{clear_notes, read_notes, search_note, write_note};
use crate::services::user::{
    find_user_by_name, get_all_users, get_email_by_name, get_status_by_name,
};
use clap::Parser;

fn main() {
    // CLI
    let args = Args::try_parse().expect("Failed to parse arguments");

    if let Some(name) = args.find {
        let user = find_user_by_name(&name);
        match user {
            Ok(user) => println!("User found: {}", user.name),
            Err(e) => println!("Error: {}", e),
        }
    }

    if let Some(name) = args.status {
        let status = get_status_by_name(&name);
        match status {
            Some(status) => println!("Status: {:?}", status),
            None => println!("Status not found"),
        }
    }

    if let Some(name) = args.email {
        let email = get_email_by_name(&name);
        match email {
            Some(email) => println!("Email: {}", email),
            None => println!("Email not found"),
        }
    }

    if args.list {
        let users = get_all_users();
        for user in users {
            let email = user.email.unwrap_or_else(|| "no email".to_string());
            println!("👤 {} | {:?} | {}", user.name, user.status, email);
        }
    }
    //  cli notes
    if let Some(note) = args.add_note {
        write_note(&note);
        println!("📝 Note added!");
    }

    if args.show_notes {
        match read_notes() {
            Ok(content) => println!("📚 Notes:\n{}", content),
            Err(e) => println!("Error reading notes: {}", e),
        }
    }

    if args.clear_notes {
        match clear_notes() {
            Ok(_) => println!("🧹 Notes cleared!"),
            Err(e) => println!("❌ {}", e),
        }
    }

    if let Some(keyword) = args.search_note {
        println!("🔍 نتائج البحث عن '{}':", keyword);
        let results = search_note(&keyword);
        match results {
            Ok(content) => println!("{}", content),
            Err(e) => println!("❌ {}", e),
        }
    }

    // normal way
    // let user = find_user_by_name("Hadeel");

    // match user {
    //     Ok(user) => println!("User found: {}", user.name),
    //     Err(e) => println!("Error: {}", e),
    // }

    // let status = get_status_by_name("Rawan");
    // match status {
    //     Some(status) => println!("Status: {:?}", status),
    //     None => println!("Status not found"),
    // }

    // let email = get_email_by_name("Hadeel");
    // match email {
    //     Some(email) => println!("Email: {}", email),
    //     None => println!("Email not found"),
    // }
}
