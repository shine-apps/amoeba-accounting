use rusqlite::{params, Connection, Result, OptionalExtension};
use crate::models::amoeba::{Amoeba, AmoebaInput};

/// 获取所有阿米巴组织列表
pub fn list(conn: &Connection) -> Result<Vec<Amoeba>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amoeba_type, leader, parent_id, status, created_at, updated_at
         FROM amoeba
         ORDER BY id ASC",
    )?;

    let amoebas = stmt
        .query_map([], |row| {
            Ok(Amoeba {
                id: row.get(0)?,
                name: row.get(1)?,
                amoeba_type: row.get(2)?,
                leader: row.get(3)?,
                parent_id: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })?
        .collect::<Result<Vec<_>>>()?;

    Ok(amoebas)
}

/// 根据 ID 获取单个阿米巴组织
pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Amoeba>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, amoeba_type, leader, parent_id, status, created_at, updated_at
         FROM amoeba
         WHERE id = ?1",
    )?;

    let amoeba = stmt
        .query_row(params![id], |row| {
            Ok(Amoeba {
                id: row.get(0)?,
                name: row.get(1)?,
                amoeba_type: row.get(2)?,
                leader: row.get(3)?,
                parent_id: row.get(4)?,
                status: row.get(5)?,
                created_at: row.get(6)?,
                updated_at: row.get(7)?,
            })
        })
        .optional()?;

    Ok(amoeba)
}

/// 创建阿米巴组织
pub fn insert(conn: &Connection, input: &AmoebaInput) -> Result<Amoeba> {
    conn.execute(
        "INSERT INTO amoeba (name, amoeba_type, leader, parent_id, status)
         VALUES (?1, ?2, ?3, ?4, 'active')",
        params![input.name, input.amoeba_type, input.leader, input.parent_id],
    )?;

    let id = conn.last_insert_rowid();
    get_by_id(conn, id).map(|opt| opt.unwrap())
}

/// 更新阿米巴组织
pub fn update(conn: &Connection, id: i64, input: &AmoebaInput) -> Result<Amoeba> {
    conn.execute(
        "UPDATE amoeba
         SET name = ?1, amoeba_type = ?2, leader = ?3, parent_id = ?4,
             updated_at = datetime('now', 'localtime')
         WHERE id = ?5",
        params![input.name, input.amoeba_type, input.leader, input.parent_id, id],
    )?;

    get_by_id(conn, id).map(|opt| opt.unwrap())
}

/// 删除阿米巴组织
pub fn delete(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM amoeba WHERE id = ?1", params![id])?;
    Ok(())
}
