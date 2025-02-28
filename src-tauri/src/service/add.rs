use rusqlite::{params, Connection, Result};

use crate::models::TaskList;

pub fn update_task(conn: &mut Connection, task_list: TaskList) -> Result<()> {
    let tx = conn.transaction()?;
    {
        // 准备两个语句：一个用于更新，一个用于插入
        let mut update_stmt = tx.prepare("UPDATE task_list SET task_name = ?1 WHERE id = ?2")?;
        let mut insert_stmt =
            tx.prepare("INSERT INTO task_list (id, task_name) VALUES (?1, ?2)")?;
        // 先尝试更新
        let rows_updated = update_stmt.execute(params![task_list.name, task_list.id])?;
        // 如果没有更新到任何行，说明记录不存在，执行插入
        if rows_updated == 0 {
            insert_stmt.execute(params![task_list.id, task_list.name])?;
        }
    }
    tx.commit()?;
    Ok(())
}
