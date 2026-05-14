pub mod models;
pub mod repository;
pub mod services;
pub mod export;
pub mod commands;

use std::sync::Mutex;
use repository::db::init_db;
use tauri::Manager;

/// Tauri 应用入口
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            // 初始化数据库
            let conn = init_db(&app.handle())
                .expect("数据库初始化失败，请检查应用数据目录权限");

            // 注册数据库连接为 Tauri State
            app.manage(Mutex::new(conn));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // 阿米巴组织管理
            commands::list_amoebas,
            commands::create_amoeba,
            commands::update_amoeba,
            commands::delete_amoeba,
            // 类别设置
            commands::get_categories,
            commands::save_categories,
            commands::reset_categories,
            // 核算记录管理
            commands::list_records,
            commands::get_record,
            commands::save_record,
            commands::delete_record,
            // Excel 导出
            commands::export_excel,
        ])
        .run(tauri::generate_context!())
        .expect("启动应用时发生错误");
}
