use clap::Parser;

#[derive(Debug)]
pub enum UserStatus {
    Active,
    Inactive,
    Banned(String),
}

pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub status: UserStatus,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long)]
    pub find: Option<String>,

    #[arg(long)]
    pub email: Option<String>,

    #[arg(long)]
    pub status: Option<String>,

    #[arg(long)]
    pub list: bool,
}
