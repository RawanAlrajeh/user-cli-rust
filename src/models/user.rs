#[derive(Debug)]
pub enum UserStatus {
    Active,
    Inactive,
    #[allow(dead_code)]
    Banned(String),
}

pub struct User {
    pub name: String,
    pub email: Option<String>,
    pub status: UserStatus,
}
