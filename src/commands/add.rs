#![allow(warnings)]

use crate::db;
use crate::models::task::Task;
use nanoid::nanoid;
use chrono::{NaiveDate, Local};

pub fn add_task(name: String, priority: Option<&str>, date: Option<&str>) -> Result<String, String> {
    let default_date = Local::today().naive_local().to_string();
    let default_priority = "4";

    let parsed_date = date.unwrap_or(&default_date);

    let priority_str = priority.unwrap_or(&default_priority);
    let parsed_priority = priority_str.parse::<u8>()
        .map_err(|_| "Invalid priority")?;

    let new_task = Task {
        id: nanoid!(),
        name,
        date: NaiveDate::parse_from_str(parsed_date, "%Y-%m-%d").map_err(|_| "Invalid date format")?,
        is_checked: false,
        priority: parsed_priority
    };

    db::add_task_db(&new_task)
        .map(|_| new_task.id)
        .map_err(|e| format!("Error adding task to the database: {}", e))
}