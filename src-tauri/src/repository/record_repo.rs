use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::models::accounting_record::{AccountingRecord, AccountingResult};
use crate::models::labor_time::LaborTime;
use super::{expense_repo, income_repo, labor_repo};

/// 获取某个阿米巴的所有核算记录
pub fn list_by_amoeba(conn: &Connection, amoeba_id: i64) -> Result<Vec<AccountingRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, amoeba_id, period_type, period_start, period_end,
                external_sales, internal_sales, remark, created_at, updated_at,
                total_sales, total_expense, added_value, total_hours,
                unit_value, sales_per_person, value_per_person, value_rate, expense_rate
         FROM accounting_record
         WHERE amoeba_id = ?1
         ORDER BY period_start DESC",
    )?;

    let records = stmt
        .query_map(params![amoeba_id], |row| {
            Ok(AccountingRecord {
                id: row.get(0)?,
                amoeba_id: row.get(1)?,
                period_type: row.get(2)?,
                period_start: row.get(3)?,
                period_end: row.get(4)?,
                external_sales: row.get(5)?,
                internal_sales: row.get(6)?,
                remark: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
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
                result: Some(AccountingResult {
                    total_sales: row.get(10)?,
                    total_expense: row.get(11)?,
                    added_value: row.get(12)?,
                    total_hours: row.get(13)?,
                    unit_value: row.get(14)?,
                    sales_per_person: row.get(15)?,
                    value_per_person: row.get(16)?,
                    value_rate: row.get(17)?,
                    expense_rate: row.get(18)?,
                }),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    // 为每条记录加载关联数据
    let mut result = Vec::with_capacity(records.len());
    for mut record in records {
        let record_id = record.id.unwrap();
        record.income_details = income_repo::list_by_record(conn, record_id)?;
        record.expenses = expense_repo::list_by_record(conn, record_id)?;
        record.labor = labor_repo::get_by_record(conn, record_id)?.unwrap_or(LaborTime {
            id: None,
            record_id: Some(record_id),
            normal_hours: 0.0,
            overtime_hours: 0.0,
            public_hours: 0.0,
            headcount: 1,
        });
        result.push(record);
    }

    Ok(result)
}

/// 获取单条核算记录（含完整关联数据）
pub fn get_with_details(conn: &Connection, id: i64) -> Result<Option<AccountingRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, amoeba_id, period_type, period_start, period_end,
                external_sales, internal_sales, remark, created_at, updated_at,
                total_sales, total_expense, added_value, total_hours,
                unit_value, sales_per_person, value_per_person, value_rate, expense_rate
         FROM accounting_record
         WHERE id = ?1",
    )?;

    let record = stmt
        .query_row(params![id], |row| {
            Ok(AccountingRecord {
                id: row.get(0)?,
                amoeba_id: row.get(1)?,
                period_type: row.get(2)?,
                period_start: row.get(3)?,
                period_end: row.get(4)?,
                external_sales: row.get(5)?,
                internal_sales: row.get(6)?,
                remark: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
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
                result: Some(AccountingResult {
                    total_sales: row.get(10)?,
                    total_expense: row.get(11)?,
                    added_value: row.get(12)?,
                    total_hours: row.get(13)?,
                    unit_value: row.get(14)?,
                    sales_per_person: row.get(15)?,
                    value_per_person: row.get(16)?,
                    value_rate: row.get(17)?,
                    expense_rate: row.get(18)?,
                }),
            })
        })
        .optional()?;

    let mut record = match record {
        Some(r) => r,
        None => return Ok(None),
    };

    let record_id = record.id.unwrap();
    record.income_details = income_repo::list_by_record(conn, record_id)?;
    record.expenses = expense_repo::list_by_record(conn, record_id)?;
    record.labor = labor_repo::get_by_record(conn, record_id)?.unwrap_or(LaborTime {
        id: None,
        record_id: Some(record_id),
        normal_hours: 0.0,
        overtime_hours: 0.0,
        public_hours: 0.0,
        headcount: 1,
    });

    Ok(Some(record))
}

/// 插入核算记录（含计算结果）
pub fn insert(conn: &Connection, record: &AccountingRecord, result: &AccountingResult) -> Result<i64> {
    conn.execute(
        "INSERT INTO accounting_record
         (amoeba_id, period_type, period_start, period_end,
          external_sales, internal_sales, remark,
          total_sales, total_expense, added_value, total_hours,
          unit_value, sales_per_person, value_per_person, value_rate, expense_rate)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![
            record.amoeba_id,
            record.period_type,
            record.period_start,
            record.period_end,
            record.external_sales,
            record.internal_sales,
            record.remark,
            result.total_sales,
            result.total_expense,
            result.added_value,
            result.total_hours,
            result.unit_value,
            result.sales_per_person,
            result.value_per_person,
            result.value_rate,
            result.expense_rate,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// 更新核算记录（含计算结果）
pub fn update(conn: &Connection, record: &AccountingRecord, result: &AccountingResult) -> Result<()> {
    let id = record.id.unwrap();
    conn.execute(
        "UPDATE accounting_record
         SET amoeba_id = ?1, period_type = ?2, period_start = ?3, period_end = ?4,
             external_sales = ?5, internal_sales = ?6, remark = ?7,
             total_sales = ?8, total_expense = ?9, added_value = ?10, total_hours = ?11,
             unit_value = ?12, sales_per_person = ?13, value_per_person = ?14,
             value_rate = ?15, expense_rate = ?16,
             updated_at = datetime('now', 'localtime')
         WHERE id = ?17",
        params![
            record.amoeba_id,
            record.period_type,
            record.period_start,
            record.period_end,
            record.external_sales,
            record.internal_sales,
            record.remark,
            result.total_sales,
            result.total_expense,
            result.added_value,
            result.total_hours,
            result.unit_value,
            result.sales_per_person,
            result.value_per_person,
            result.value_rate,
            result.expense_rate,
            id,
        ],
    )?;
    Ok(())
}

/// 删除核算记录
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM accounting_record WHERE id = ?1", params![id])?;
    Ok(())
}

/// 按周期查询核算记录
pub fn query_by_period(
    conn: &Connection,
    amoeba_id: i64,
    period_type: &str,
    start: &str,
    end: &str,
) -> Result<Vec<AccountingRecord>> {
    let mut stmt = conn.prepare(
        "SELECT id, amoeba_id, period_type, period_start, period_end,
                external_sales, internal_sales, remark, created_at, updated_at,
                total_sales, total_expense, added_value, total_hours,
                unit_value, sales_per_person, value_per_person, value_rate, expense_rate
         FROM accounting_record
         WHERE amoeba_id = ?1 AND period_type = ?2
               AND period_start >= ?3 AND period_end <= ?4
         ORDER BY period_start ASC",
    )?;

    let records = stmt
        .query_map(params![amoeba_id, period_type, start, end], |row| {
            Ok(AccountingRecord {
                id: row.get(0)?,
                amoeba_id: row.get(1)?,
                period_type: row.get(2)?,
                period_start: row.get(3)?,
                period_end: row.get(4)?,
                external_sales: row.get(5)?,
                internal_sales: row.get(6)?,
                remark: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
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
                result: Some(AccountingResult {
                    total_sales: row.get(10)?,
                    total_expense: row.get(11)?,
                    added_value: row.get(12)?,
                    total_hours: row.get(13)?,
                    unit_value: row.get(14)?,
                    sales_per_person: row.get(15)?,
                    value_per_person: row.get(16)?,
                    value_rate: row.get(17)?,
                    expense_rate: row.get(18)?,
                }),
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    // 为每条记录加载关联数据
    let mut result = Vec::with_capacity(records.len());
    for mut record in records {
        let record_id = record.id.unwrap();
        record.income_details = income_repo::list_by_record(conn, record_id)?;
        record.expenses = expense_repo::list_by_record(conn, record_id)?;
        record.labor = labor_repo::get_by_record(conn, record_id)?.unwrap_or(LaborTime {
            id: None,
            record_id: Some(record_id),
            normal_hours: 0.0,
            overtime_hours: 0.0,
            public_hours: 0.0,
            headcount: 1,
        });
        result.push(record);
    }

    Ok(result)
}
