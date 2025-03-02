mod models;
mod service;

use models::*;
use rusqlite::Connection;
use service::*;
use tauri::ipc::Response;
use std::sync::Mutex;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{fmt::Layer, layer::SubscriberExt, util::SubscriberInitExt, Layer as _};

#[derive(Debug)]
struct AppState {
    db: Mutex<Connection>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let layer = Layer::new().with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(layer).init();
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
        .invoke_handler(tauri::generate_handler![get_my_items, update_my_items])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//获取任务列表
#[tauri::command]
fn get_my_items(state: tauri::State<AppState>) -> Response {
    // 这里可以从数据库或文件读取数据
    let conn = state.db.lock().unwrap();
    let task_list = get_task_list(&conn);
    tauri::ipc::Response::new(serde_json::to_string(&task_list).unwrap())
}

//添加更新任务列表
#[tauri::command]
fn update_my_items(task: TaskList, state: tauri::State<AppState>) {
    info!("前端传来的任务列表 {:?}", task);
    let mut conn = state.db.lock().unwrap();
    update_task(&mut conn, task).unwrap();
}
