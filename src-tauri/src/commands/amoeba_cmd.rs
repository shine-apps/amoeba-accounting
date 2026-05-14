use std::sync::Mutex;
use rusqlite::Connection;
use tauri::State;
use crate::models::amoeba::{Amoeba, AmoebaInput};
use crate::repository::amoeba_repo;
use crate::repository::category_repo;

pub fn list_amoebas_inner(conn: &Connection) -> Result<Vec<Amoeba>, String> {
    amoeba_repo::list(conn).map_err(|e| format!("查询阿米巴列表失败: {}", e))
}

pub fn create_amoeba_inner(conn: &Connection, input: &AmoebaInput) -> Result<Amoeba, String> {
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
    let amoeba = amoeba_repo::insert(conn, input).map_err(|e| format!("创建阿米巴组织失败: {}", e))?;
    let amoeba_id = amoeba.id.unwrap();
    category_repo::seed_defaults(conn, amoeba_id)
        .map_err(|e| format!("播种默认类别失败: {}", e))?;
    Ok(amoeba)
}

pub fn update_amoeba_inner(conn: &Connection, id: i64, input: &AmoebaInput) -> Result<Amoeba, String> {
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
    amoeba_repo::update(conn, id, input).map_err(|e| format!("更新阿米巴组织失败: {}", e))
}

pub fn delete_amoeba_inner(conn: &Connection, id: i64) -> Result<(), String> {
    amoeba_repo::delete(conn, id).map_err(|e| format!("删除阿米巴组织失败: {}", e))
}

/// 获取所有阿米巴组织列表
#[tauri::command]
pub async fn list_amoebas(
    db: State<'_, Mutex<Connection>>,
) -> Result<Vec<Amoeba>, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    list_amoebas_inner(&conn)
}

/// 创建阿米巴组织
#[tauri::command]
pub async fn create_amoeba(
    db: State<'_, Mutex<Connection>>,
    input: AmoebaInput,
) -> Result<Amoeba, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    create_amoeba_inner(&conn, &input)
}

/// 更新阿米巴组织
#[tauri::command]
pub async fn update_amoeba(
    db: State<'_, Mutex<Connection>>,
    id: i64,
    input: AmoebaInput,
) -> Result<Amoeba, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    update_amoeba_inner(&conn, id, &input)
}

/// 删除阿米巴组织
#[tauri::command]
pub async fn delete_amoeba(
    db: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    delete_amoeba_inner(&conn, id)
}
