mod service;

use rusqlite::Connection;
use service::*;
use std::sync::Mutex;

#[derive(Debug)]
struct AppState {
    db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 创建数据库连接
    let conn = Connection::open("my_database.db").expect("Failed to connect to database");

    // 初始化数据库表和数据
    let _ = create_table(&conn);
    let _ = insert_user(&conn);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            db: Mutex::new(conn),
        })
        .invoke_handler(tauri::generate_handler![get_my_items,update_my_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//获取任务列表
#[tauri::command]
fn get_my_items(state: tauri::State<AppState>) -> Vec<String> {
    // 这里可以从数据库或文件读取数据
    let conn = state.db.lock().unwrap();
    get_task_list(&conn)
}

//添加更新任务列表
#[tauri::command]
fn update_my_items(items: Vec<String>, state: tauri::State<AppState>) {
    let conn = state.db.lock().unwrap();
    update_task(&conn,items);
}
