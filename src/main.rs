mod database;
mod models;
mod services;
use crate::models::Args;
use crate::services::{find_user_by_name, get_all_users, get_email_by_name, get_status_by_name};
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
