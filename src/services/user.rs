use crate::database::get_users;
use crate::models::user::{User, UserStatus};

pub fn find_user_by_name(name: &str) -> Result<User, String> {
    get_users()
        .into_iter()
        .find(|user| user.name == name)
        .ok_or_else(|| "User not found".to_string())
}

pub fn get_status_by_name(name: &str) -> Option<UserStatus> {
    find_user_by_name(name)
        .ok()
        .and_then(|user| Some(user.status))
}

pub fn get_email_by_name(name: &str) -> Option<String> {
    find_user_by_name(name).ok().and_then(|user| user.email)
}

pub fn get_all_users() -> Vec<User> {
    get_users()
}
