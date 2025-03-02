use rusqlite::Connection;
use tracing::info;

use crate::models::TaskList;

pub fn get_task_list(conn: &Connection) -> Vec<TaskList> {
    let mut stmt = conn
        .prepare("SELECT id, task_name ,is_have FROM task_list where is_have =0")
        .expect("连接数据库是错误");
    let tasks = stmt
        .query_map([], |row| {
            Ok(TaskList {
                id: row.get(0)?,
                task_name: row.get(1)?,
                is_have: row.get(2)?,
            })
        })
        .expect("获取数据错误");
    let task_list: Vec<TaskList> = tasks.map(|t| t.expect("获取值错误")).collect();
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
