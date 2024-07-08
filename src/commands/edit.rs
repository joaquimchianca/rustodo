use crate::db;

pub fn change_task_date(task_id: &str, new_date: &str) -> Result<(), String> {
    match db::update_date_db(task_id, new_date) {
        Ok(row) => {
            if row > 0 {
                Ok(())
            } else {
                Err("No task was updated. Please check task ID".to_string())
            }
        },
        Err(e) => Err(format!("Failed to update task date: {}", e))
    }
}