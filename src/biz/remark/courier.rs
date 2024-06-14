use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RemarkCourier {
    pub parent: i64,
    pub content: String,
}