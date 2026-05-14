use std::sync::Mutex;
use rusqlite::Connection;
use tauri::State;
use crate::models::amoeba_category::{CategoryList, SaveCategoriesInput};
use crate::repository::category_repo;

pub fn get_categories_inner(conn: &Connection, amoeba_id: i64) -> Result<CategoryList, String> {
    category_repo::list_by_amoeba(conn, amoeba_id)
        .map_err(|e| format!("查询类别列表失败: {}", e))
}

pub fn save_categories_inner(
    conn: &Connection,
    amoeba_id: i64,
    input: &SaveCategoriesInput,
) -> Result<(), String> {
    if input.income.is_empty() {
        return Err("至少需要一个收入类别".to_string());
    }
    if input.expense.is_empty() {
        return Err("至少需要一个费用类别".to_string());
    }
    for cat in &input.income {
        if cat.name.trim().is_empty() {
            return Err("收入类别名称不能为空".to_string());
        }
    }
    for cat in &input.expense {
        if cat.name.trim().is_empty() {
            return Err("费用类别名称不能为空".to_string());
        }
    }
    category_repo::save_all(conn, amoeba_id, input)
        .map_err(|e| format!("保存类别失败: {}", e))
}

pub fn reset_categories_inner(conn: &Connection, amoeba_id: i64) -> Result<CategoryList, String> {
    category_repo::reset_to_defaults(conn, amoeba_id)
        .map_err(|e| format!("恢复默认类别失败: {}", e))
}

/// 获取指定阿米巴的类别列表
#[tauri::command]
pub async fn get_categories(
    db: State<'_, Mutex<Connection>>,
    amoeba_id: i64,
) -> Result<CategoryList, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    get_categories_inner(&conn, amoeba_id)
}

/// 保存指定阿米巴的类别列表
#[tauri::command]
pub async fn save_categories(
    db: State<'_, Mutex<Connection>>,
    amoeba_id: i64,
    input: SaveCategoriesInput,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    save_categories_inner(&conn, amoeba_id, &input)
}

/// 恢复指定阿米巴的默认类别
#[tauri::command]
pub async fn reset_categories(
    db: State<'_, Mutex<Connection>>,
    amoeba_id: i64,
) -> Result<CategoryList, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    reset_categories_inner(&conn, amoeba_id)
}
