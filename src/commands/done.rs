use crate::db;

pub fn check_task(task_ids: Vec<&str>) -> Result<Vec<String>, String> {
    match db::check_task_db(&task_ids) {
        Ok(ids) => Ok(ids),
        Err(e) => Err(e.to_string())
    }


}