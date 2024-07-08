// use uuid::Uuid;
use chrono::NaiveDate;

pub struct Task {
    pub id: String,
    pub name: String,
    pub priority: u8,
    pub date: NaiveDate,
    pub is_checked: bool,
}