use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Note {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub updated_at: Option<String>,
}
