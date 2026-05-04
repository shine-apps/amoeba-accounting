use std::sync::Mutex;
use rusqlite::Connection;
use tauri::State;
use crate::models::amoeba::{Amoeba, AmoebaInput};
use crate::repository::amoeba_repo;

/// 获取所有阿米巴组织列表
#[tauri::command]
pub async fn list_amoebas(
    db: State<'_, Mutex<Connection>>,
) -> Result<Vec<Amoeba>, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    amoeba_repo::list(&conn).map_err(|e| format!("查询阿米巴列表失败: {}", e))
}

/// 创建阿米巴组织
#[tauri::command]
pub async fn create_amoeba(
    db: State<'_, Mutex<Connection>>,
    input: AmoebaInput,
) -> Result<Amoeba, String> {
    // 校验输入
    if input.name.trim().is_empty() {
        return Err("阿米巴组织名称不能为空".to_string());
    }
    let valid_types = ["生产型", "营销型", "研发型", "管理型"];
    if !valid_types.contains(&input.amoeba_type.as_str()) {
        return Err(format!(
            "组织类型必须是以下之一: {}",
            valid_types.join(", ")
        ));
    }

    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    amoeba_repo::insert(&conn, &input).map_err(|e| format!("创建阿米巴组织失败: {}", e))
}

/// 更新阿米巴组织
#[tauri::command]
pub async fn update_amoeba(
    db: State<'_, Mutex<Connection>>,
    id: i64,
    input: AmoebaInput,
) -> Result<Amoeba, String> {
    if input.name.trim().is_empty() {
        return Err("阿米巴组织名称不能为空".to_string());
    }
    let valid_types = ["生产型", "营销型", "研发型", "管理型"];
    if !valid_types.contains(&input.amoeba_type.as_str()) {
        return Err(format!(
            "组织类型必须是以下之一: {}",
            valid_types.join(", ")
        ));
    }

    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    amoeba_repo::update(&conn, id, &input).map_err(|e| format!("更新阿米巴组织失败: {}", e))
}

/// 删除阿米巴组织
#[tauri::command]
pub async fn delete_amoeba(
    db: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    amoeba_repo::delete(&conn, id).map_err(|e| format!("删除阿米巴组织失败: {}", e))
}
