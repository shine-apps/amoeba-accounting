use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::models::labor_time::{LaborTime, LaborTimeInput};

/// 插入工时记录
pub fn insert(conn: &Connection, record_id: i64, labor: &LaborTimeInput) -> Result<()> {
    conn.execute(
        "INSERT INTO labor_time (record_id, normal_hours, overtime_hours, public_hours, headcount)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            record_id,
            labor.normal_hours,
            labor.overtime_hours,
            labor.public_hours,
            labor.headcount
        ],
    )?;
    Ok(())
}

/// 获取某条核算记录的工时数据
pub fn get_by_record(conn: &Connection, record_id: i64) -> Result<Option<LaborTime>> {
    let mut stmt = conn.prepare(
        "SELECT id, record_id, normal_hours, overtime_hours, public_hours, headcount
         FROM labor_time
         WHERE record_id = ?1",
    )?;

    let labor = stmt
        .query_row(params![record_id], |row| {
            Ok(LaborTime {
                id: row.get(0)?,
                record_id: row.get(1)?,
                normal_hours: row.get(2)?,
                overtime_hours: row.get(3)?,
                public_hours: row.get(4)?,
                headcount: row.get(5)?,
            })
        })
        .optional()?;

    Ok(labor)
}

/// 更新工时记录
pub fn update(conn: &Connection, record_id: i64, labor: &LaborTimeInput) -> Result<()> {
    conn.execute(
        "UPDATE labor_time
         SET normal_hours = ?1, overtime_hours = ?2, public_hours = ?3, headcount = ?4
         WHERE record_id = ?5",
        params![
            labor.normal_hours,
            labor.overtime_hours,
            labor.public_hours,
            labor.headcount,
            record_id
        ],
    )?;
    Ok(())
}
