use rusqlite::{params, Connection, Result};
use crate::models::expense_detail::{ExpenseDetail, ExpenseDetailInput};

/// 批量插入费用明细
pub fn insert_batch(conn: &Connection, record_id: i64, expenses: &[ExpenseDetailInput]) -> Result<()> {
    let mut stmt = conn.prepare(
        "INSERT INTO expense_detail (record_id, category, amount, description)
         VALUES (?1, ?2, ?3, ?4)",
    )?;

    for expense in expenses {
        stmt.execute(params![record_id, expense.category, expense.amount, expense.description])?;
    }

    Ok(())
}

/// 获取某条核算记录的所有费用明细
pub fn list_by_record(conn: &Connection, record_id: i64) -> Result<Vec<ExpenseDetail>> {
    let mut stmt = conn.prepare(
        "SELECT id, record_id, category, amount, description
         FROM expense_detail
         WHERE record_id = ?1
         ORDER BY id ASC",
    )?;

    let expenses = stmt
        .query_map(params![record_id], |row| {
            Ok(ExpenseDetail {
                id: row.get(0)?,
                record_id: row.get(1)?,
                category: row.get(2)?,
                amount: row.get(3)?,
                description: row.get(4)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(expenses)
}

/// 删除某条核算记录的所有费用明细
pub fn delete_by_record(conn: &Connection, record_id: i64) -> Result<()> {
    conn.execute("DELETE FROM expense_detail WHERE record_id = ?1", params![record_id])?;
    Ok(())
}
