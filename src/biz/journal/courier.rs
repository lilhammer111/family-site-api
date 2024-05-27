use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct JournalJson {
    pub title: String,
    pub content: String,
    pub images: Vec<String>,
}