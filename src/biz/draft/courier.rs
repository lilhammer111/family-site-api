use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct DraftCourier {
    pub text: String,
}