use rusqlite::Connection;


pub fn get_task_list(conn: &Connection) -> Vec<String> {
    let mut stmt = conn.prepare("SELECT task_name FROM task_list").unwrap();
    let tasks = stmt.query_map([], |row| row.get::<_, String>(0)).unwrap();
    let task_list = tasks.map(|t| t.unwrap()).collect();
    println!("{:?}", task_list);
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
