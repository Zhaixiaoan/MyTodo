use rusqlite::Connection;
use tracing::info;

use crate::models::TaskList;

pub fn get_task_list(conn: &Connection) -> Vec<TaskList> {
    let mut stmt = conn.prepare("SELECT id, task_name FROM task_list").unwrap();
    let tasks = stmt
        .query_map([], |row| {
            Ok(TaskList {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .unwrap();
    let task_list: Vec<TaskList> = tasks.map(|t| t.unwrap()).collect();
    info!("查询task_list {:?}", task_list);
    return task_list;
}
#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_get_task_list() {
        let conn: Connection =
            Connection::open("my_database.db").expect("Failed to connect to database");
        get_task_list(&conn);
    }
}
