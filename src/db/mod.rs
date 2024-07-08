#![allow(warnings)]

use rusqlite::{params, Connection, Result, NO_PARAMS, Statement};
use crate::models::task::Task;
use chrono::{NaiveDate, Local, Datelike, Duration, Weekday};

pub fn create_connection() -> Result<Connection> {
    let connection = Connection::open("rustodo.db")?;
    Ok(connection)
}

pub fn setup_database() -> Result<()>{
    let connection = create_connection()?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS tasks(
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            date TEXT NOT NULL,
            is_checked BOOLEAN NOT NULL,
            priority INTEGER
        )", [],
    )?;
    Ok(())
}

pub fn add_task_db(task: &Task) -> Result<()> {
    let connection = create_connection()?;
    connection.execute(
        "INSERT INTO tasks (id, name, date, is_checked, priority)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![&task.id, &task.name, &task.date.format("%Y-%m-%d").to_string(), &task.is_checked, &task.priority],
    )?;
    Ok(())
}

pub fn list_tasks_db(filter: &str) -> Result<Vec<Task>, rusqlite::Error> {
    let connection = create_connection()?;
    
    let today = Local::today().naive_local();
    let formatted_today = today.format("%Y-%m-%d").to_string();
    
    let start_of_week_date = (today - Duration::days(today.weekday().num_days_from_sunday() as i64));
    let end_of_week_date = start_of_week_date + Duration::days(6);
    let start_of_week = start_of_week_date.format("%Y-%m-%d").to_string();
    let end_of_week = end_of_week_date.format("%Y-%m-%d").to_string();

    let start_month_date = NaiveDate::from_ymd(today.year(), today.month(), 1);
    let end_month_date = NaiveDate::from_ymd_opt(today.year(), today.month() + 1, 1)
        .unwrap_or(NaiveDate::from_ymd(today.year() + 1, 1, 1))
        .pred();
    let start_month = start_month_date.format("%Y-%m-%d").to_string();
    let end_month = end_month_date.format("%Y-%m-%d").to_string();

    let sql_query = match filter {
        "checked" => "SELECT * FROM tasks WHERE is_checked = 1",
        "week" | "month" => "SELECT * FROM tasks WHERE date BETWEEN ?1 AND ?2 AND is_checked = 0",
        //"month" => "SELECT * FROM tasks WHERE date BETWEEN ?1 AND ?2 AND is_checked = 0",
        _ => "SELECT * FROM tasks WHERE date <= ?1 AND is_checked = 0",
    };

    let mut stmt: Statement = connection.prepare(sql_query)?;

    let params = match filter {
        "week" => vec![&start_of_week as &dyn rusqlite::ToSql, &end_of_week as &dyn rusqlite::ToSql],
        "month" => vec![&start_month as &dyn rusqlite::ToSql, &end_month as &dyn rusqlite::ToSql],
        "checked" => vec![],
        _ => vec![&formatted_today as &dyn rusqlite::ToSql],
    };

    let task_iter = stmt.query_map(&*params, |row| {
        Ok(Task {
            id: row.get(0)?,
            name: row.get(1)?,
            date: NaiveDate::parse_from_str(&row.get::<_, String>(2)?, "%Y-%m-%d").map_err(|_| rusqlite::Error::InvalidQuery)?,
            is_checked: row.get(3)?,
            priority: row.get(4)?,
        })
    })?;

    let mut tasks: Vec<Task> = task_iter.collect::<Result<Vec<_>, _>>()?;
    tasks.sort_by(|a, b| a.date.cmp(&b.date));
    Ok(tasks)
}

pub fn delete_tasks_db(ids: &[&str]) -> Result<Vec<String>> {
    let connection = create_connection()?;
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let sql = format!("DELETE FROM tasks WHERE id IN ({})", placeholders);

    let params = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect::<Vec<_>>();

    connection.execute(&sql, params.as_slice())?;

    Ok(ids.iter().map(|&id| id.to_string()).collect::<Vec<String>>())
}

pub fn check_task_db(ids: &[&str]) -> Result<Vec<String>> {
    let connection = create_connection()?;
    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let sql = format!("UPDATE tasks SET is_checked = 1 WHERE id IN ({})", placeholders);

    let params = ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect::<Vec<_>>();

    let changes = connection.execute(&sql, params.as_slice())?;
    if changes > 0 {
        Ok(ids.iter().map(|&id| id.to_string()).collect::<Vec<String>>())
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }

}

pub fn update_date_db(task_id: &str, new_date: &str) -> Result<usize> {
    let connection = create_connection()?;
    let sql = "UPDATE tasks SET date = ? WHERE id =?";
    let updated_row = connection.execute(sql, &[new_date, task_id])?;
    Ok(updated_row)
}