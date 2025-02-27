use rusqlite::{params, Connection};



pub fn update_task(conn: &Connection,items: Vec<String>)  {
    conn.execute(
        "INSERT INTO task (id, list_id, my_day, im_task, my_task, task, state, delete_state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![1, 0, 0, 0, 0, "我的一天", 0, 0],
    )?;
}