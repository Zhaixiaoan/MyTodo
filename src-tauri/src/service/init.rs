use rusqlite::{params, Connection, Result};

pub fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            list_id INTEGER,
            my_day INTEGER,
            im_task INTEGER,
            my_task INTEGER,
            task TEXT,
            state INTEGER,
            delete_state INTEGER
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task_list (
            id INTEGER PRIMARY KEY,
            task_name TEXT 
        )",
        [],
    )?;

    Ok(())
}

pub fn insert_user(conn: &Connection) -> Result<()> {
    //查询如果不存在则创建
    let mut stmt = conn.prepare("select * from  task WHERE id in (1,2,3)")?;
    let rows = stmt.query_map([], |row| row.get::<_, i32>(0))?;
    if rows.count() == 0 {
        conn.execute(
            "INSERT INTO task (id, list_id, my_day, im_task, my_task, task, state, delete_state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![1, 0, 0, 0, 0, "我的一天", 0, 0],
        )?;
        conn.execute(
            "INSERT INTO task (id, list_id, my_day, im_task, my_task, task, state, delete_state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![2, 0, 0, 0, 0, "重要", 0, 0],
        )?;
        conn.execute(
            "INSERT INTO task (id, list_id, my_day, im_task, my_task, task, state, delete_state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![3, 0, 0, 0, 0, "已分配给我的", 0, 0],
        )?;
    }
    Ok(())
}
