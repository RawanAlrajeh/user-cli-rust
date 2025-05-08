use crate::models::{User, UserStatus};

pub fn get_users() -> Vec<User> {
    vec![
        User {
            name: "Rawan".into(),
            email: Some("rawan@email.com".into()),
            status: UserStatus::Active,
        },
        User {
            name: "Nada".into(),
            email: None,
            status: UserStatus::Inactive,
        },
        User {
            name: "Hadeel".into(),
            email: Some("hadeel@banned.com".into()),
            status: UserStatus::Banned("abuse".into()),
        },
    ]
}
