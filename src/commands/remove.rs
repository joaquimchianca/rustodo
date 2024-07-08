use crate::db;

pub fn delete_tasks(ids: Vec<&str>) -> Result<Vec<String>, String> {
    match db::delete_tasks_db(&ids) {
        Ok(removed_ids) => Ok(removed_ids),
        Err(e) => Err(e.to_string())
    }
}