use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize)]
pub struct HealthJson {
    pub height: f64,
    pub weight: f64,
    pub teeth: i32,
    pub head_circumference: f64,
    pub record_date: NaiveDate,
}