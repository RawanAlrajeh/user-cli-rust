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

