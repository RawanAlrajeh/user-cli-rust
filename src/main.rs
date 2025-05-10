mod mock;
mod models;
mod services;
use crate::models::args::Args;
use crate::services::commands::{handle_note_commands, handle_user_commands};
use clap::Parser;

fn main() {
    // CLI
    let args = Args::try_parse().expect("Failed to parse arguments");
    handle_user_commands(&args);
    handle_note_commands(&args);

    // normal way
    // run_user();
}
