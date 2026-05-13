use std::sync::Mutex;
use rusqlite::Connection;
use tauri::State;
use crate::models::accounting_record::{AccountingRecord, AccountingResult, RecordInput};
use crate::models::labor_time::LaborTime;
use crate::repository::{record_repo, expense_repo, income_repo, labor_repo};
use crate::services::{calculate, validate_record};

pub fn list_records_inner(conn: &Connection, amoeba_id: i64) -> Result<Vec<AccountingRecord>, String> {
    record_repo::list_by_amoeba(conn, amoeba_id)
        .map_err(|e| format!("查询核算记录失败: {}", e))
}

pub fn get_record_inner(conn: &Connection, id: i64) -> Result<Option<AccountingRecord>, String> {
    record_repo::get_with_details(conn, id).map_err(|e| format!("查询核算记录失败: {}", e))
}

pub fn save_record_inner(
    conn: &Connection,
    record_id: Option<i64>,
    input: &RecordInput,
) -> Result<AccountingResult, String> {
    validate_record(input)?;

    let result = calculate(&input.income_details, &input.expenses, &input.labor);

    match record_id {
        Some(id) if id > 0 => {
            let existing = record_repo::get_with_details(conn, id)
                .map_err(|e| format!("查询记录失败: {}", e))?
                .ok_or_else(|| format!("记录 ID {} 不存在", id))?;

            let record = AccountingRecord {
                id: Some(id),
                amoeba_id: input.amoeba_id,
                period_type: input.period_type.clone(),
                period_start: input.period_start.clone(),
                period_end: input.period_end.clone(),
                external_sales: 0.0,
                internal_sales: 0.0,
                remark: input.remark.clone(),
                created_at: existing.created_at,
                updated_at: String::new(),
                income_details: vec![],
                expenses: vec![],
                labor: LaborTime {
                    id: None,
                    record_id: Some(id),
                    normal_hours: input.labor.normal_hours,
                    overtime_hours: input.labor.overtime_hours,
                    public_hours: input.labor.public_hours,
                    headcount: input.labor.headcount,
                },
                result: Some(result.clone()),
            };

            record_repo::update(conn, &record, &result)
                .map_err(|e| format!("更新核算记录失败: {}", e))?;

            income_repo::delete_by_record(conn, id)
                .map_err(|e| format!("删除旧收入明细失败: {}", e))?;
            income_repo::insert_batch(conn, id, &input.income_details)
                .map_err(|e| format!("插入收入明细失败: {}", e))?;

            expense_repo::delete_by_record(conn, id)
                .map_err(|e| format!("删除旧费用明细失败: {}", e))?;
            expense_repo::insert_batch(conn, id, &input.expenses)
                .map_err(|e| format!("插入费用明细失败: {}", e))?;

            labor_repo::update(conn, id, &input.labor)
                .map_err(|e| format!("更新工时记录失败: {}", e))?;
        }
        _ => {
            let record = AccountingRecord {
                id: None,
                amoeba_id: input.amoeba_id,
                period_type: input.period_type.clone(),
                period_start: input.period_start.clone(),
                period_end: input.period_end.clone(),
                external_sales: 0.0,
                internal_sales: 0.0,
                remark: input.remark.clone(),
                created_at: String::new(),
                updated_at: String::new(),
                income_details: vec![],
                expenses: vec![],
                labor: LaborTime {
                    id: None,
                    record_id: None,
                    normal_hours: 0.0,
                    overtime_hours: 0.0,
                    public_hours: 0.0,
                    headcount: 1,
                },
                result: Some(result.clone()),
            };

            let new_id = record_repo::insert(conn, &record, &result)
                .map_err(|e| format!("插入核算记录失败: {}", e))?;

            income_repo::insert_batch(conn, new_id, &input.income_details)
                .map_err(|e| format!("插入收入明细失败: {}", e))?;

            expense_repo::insert_batch(conn, new_id, &input.expenses)
                .map_err(|e| format!("插入费用明细失败: {}", e))?;

            labor_repo::insert(conn, new_id, &input.labor)
                .map_err(|e| format!("插入工时记录失败: {}", e))?;
        }
    }

    Ok(result)
}

pub fn delete_record_inner(conn: &Connection, id: i64) -> Result<(), String> {
    record_repo::delete(conn, id).map_err(|e| format!("删除核算记录失败: {}", e))
}

/// 获取某个阿米巴的所有核算记录
#[tauri::command]
pub async fn list_records(
    db: State<'_, Mutex<Connection>>,
    amoeba_id: i64,
) -> Result<Vec<AccountingRecord>, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    list_records_inner(&conn, amoeba_id)
}

/// 获取单条核算记录（含完整关联数据）
#[tauri::command]
pub async fn get_record(
    db: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<Option<AccountingRecord>, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    get_record_inner(&conn, id)
}

/// 保存核算记录（新建或更新）
///
/// 如果 record_id 为 Some(>0) 则执行更新操作，否则执行新建操作。
#[tauri::command]
pub async fn save_record(
    db: State<'_, Mutex<Connection>>,
    record_id: Option<i64>,
    input: RecordInput,
) -> Result<AccountingResult, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    save_record_inner(&conn, record_id, &input)
}

/// 删除核算记录
#[tauri::command]
pub async fn delete_record(
    db: State<'_, Mutex<Connection>>,
    id: i64,
) -> Result<(), String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;
    delete_record_inner(&conn, id)
}
