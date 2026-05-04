use std::sync::Mutex;
use rusqlite::Connection;
use tauri::State;
use crate::models::accounting_record::AccountingRecord;
use crate::repository::{amoeba_repo, record_repo};
use crate::export::write_excel;

/// 导出 Excel 文件
///
/// 根据阿米巴组织 ID 和可选的周期筛选条件，查询核算记录并导出为 Excel 文件。
/// 返回导出文件的路径。
#[tauri::command]
pub async fn export_excel(
    db: State<'_, Mutex<Connection>>,
    amoeba_id: i64,
    period_type: Option<String>,
    period_start: Option<String>,
    period_end: Option<String>,
    output_path: String,
) -> Result<String, String> {
    let conn = db.lock().map_err(|e| format!("获取数据库锁失败: {}", e))?;

    // 查询阿米巴组织信息
    let amoeba = amoeba_repo::get_by_id(&conn, amoeba_id)
        .map_err(|e| format!("查询阿米巴组织失败: {}", e))?
        .ok_or_else(|| format!("阿米巴组织 ID {} 不存在", amoeba_id))?;

    // 查询核算记录
    let records: Vec<AccountingRecord> = match (&period_type, &period_start, &period_end) {
        (Some(pt), Some(ps), Some(pe)) => {
            record_repo::query_by_period(&conn, amoeba_id, pt, ps, pe)
                .map_err(|e| format!("查询核算记录失败: {}", e))?
        }
        _ => {
            record_repo::list_by_amoeba(&conn, amoeba_id)
                .map_err(|e| format!("查询核算记录失败: {}", e))?
        }
    };

    if records.is_empty() {
        return Err("没有可导出的核算记录".to_string());
    }

    // 确保输出路径以 .xlsx 结尾
    let final_path = if output_path.ends_with(".xlsx") {
        output_path
    } else {
        format!("{}.xlsx", output_path)
    };

    // 写入 Excel 文件
    write_excel(&records, &amoeba, &final_path)?;

    Ok(final_path)
}
