use rusqlite::{params, Connection, Result};
use crate::models::income_detail::{IncomeDetail, IncomeDetailInput};

/// 批量插入收入明细
pub fn insert_batch(conn: &Connection, record_id: i64, incomes: &[IncomeDetailInput]) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT INTO income_detail (record_id, category, amount, description)
         VALUES (?1, ?2, ?3, ?4)",
    )?;

    for income in incomes {
        stmt.execute(params![record_id, income.category, income.amount, income.description])?;
    }

    Ok(())
}

/// 获取某条核算记录的所有收入明细
pub fn list_by_record(conn: &Connection, record_id: i64) -> Result<Vec<IncomeDetail>> {
    let mut stmt = conn.prepare(
        "SELECT id, record_id, category, amount, description
         FROM income_detail
         WHERE record_id = ?1
         ORDER BY id ASC",
    )?;

    let incomes = stmt
        .query_map(params![record_id], |row| {
            Ok(IncomeDetail {
                id: row.get(0)?,
                record_id: row.get(1)?,
                category: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(incomes)
}

/// 删除某条核算记录的所有收入明细
pub fn delete_by_record(conn: &Connection, record_id: i64) -> Result<()> {
    conn.execute("DELETE FROM income_detail WHERE record_id = ?1", params![record_id])?;
    Ok(())
}
