use rusqlite::{params, Connection};
use crate::models::amoeba_category::{
    AmoebaCategory, CategoryList, SaveCategoriesInput,
    DEFAULT_INCOME_CATEGORIES, DEFAULT_EXPENSE_CATEGORIES,
};

/// 为指定阿米巴播种默认类别
pub fn seed_defaults(conn: &Connection, amoeba_id: i64) -> Result<(), String> {
    let mut stmt = conn
        .prepare(
            "INSERT INTO amoeba_category (amoeba_id, category_type, name, desc, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .map_err(|e| e.to_string())?;

    for (name, desc, sort) in DEFAULT_INCOME_CATEGORIES {
        stmt.execute(params![amoeba_id, "income", name, desc, sort])
            .map_err(|e| e.to_string())?;
    }
    for (name, desc, sort) in DEFAULT_EXPENSE_CATEGORIES {
        stmt.execute(params![amoeba_id, "expense", name, desc, sort])
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// 获取指定阿米巴的所有类别，若为空则自动播种默认值
pub fn list_by_amoeba(conn: &Connection, amoeba_id: i64) -> Result<CategoryList, String> {
    let mut stmt = conn
        .prepare(
            "SELECT id, amoeba_id, category_type, name, desc, sort_order
             FROM amoeba_category WHERE amoeba_id = ?1
             ORDER BY category_type, sort_order ASC",
        )
        .map_err(|e| e.to_string())?;

    let categories: Vec<AmoebaCategory> = stmt
        .query_map(params![amoeba_id], |row| {
            Ok(AmoebaCategory {
                id: row.get(0)?,
                amoeba_id: row.get(1)?,
                category_type: row.get(2)?,
                name: row.get(3)?,
                desc: row.get(4)?,
                sort_order: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let income: Vec<AmoebaCategory> = categories
        .iter()
        .filter(|c| c.category_type == "income")
        .cloned()
        .collect();
    let expense: Vec<AmoebaCategory> = categories
        .iter()
        .filter(|c| c.category_type == "expense")
        .cloned()
        .collect();

    Ok(CategoryList { income, expense })
}

/// 保存类别（全量替换）
pub fn save_all(
    conn: &Connection,
    amoeba_id: i64,
    input: &SaveCategoriesInput,
) -> Result<(), String> {
    conn.execute("BEGIN", []).map_err(|e| e.to_string())?;

    conn.execute(
        "DELETE FROM amoeba_category WHERE amoeba_id = ?1",
        params![amoeba_id],
    )
    .map_err(|e| {
        let _ = conn.execute("ROLLBACK", []);
        e.to_string()
    })?;

    let mut stmt = conn
        .prepare(
            "INSERT INTO amoeba_category (amoeba_id, category_type, name, desc, sort_order)
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .map_err(|e| {
            let _ = conn.execute("ROLLBACK", []);
            e.to_string()
        })?;

    for (i, cat) in input.income.iter().enumerate() {
        stmt.execute(params![
            amoeba_id,
            "income",
            cat.name,
            cat.desc,
            i as i32 + 1,
        ])
        .map_err(|e| {
            let _ = conn.execute("ROLLBACK", []);
            e.to_string()
        })?;
    }

    for (i, cat) in input.expense.iter().enumerate() {
        stmt.execute(params![
            amoeba_id,
            "expense",
            cat.name,
            cat.desc,
            i as i32 + 1,
        ])
        .map_err(|e| {
            let _ = conn.execute("ROLLBACK", []);
            e.to_string()
        })?;
    }

    conn.execute("COMMIT", []).map_err(|e| e.to_string())?;

    Ok(())
}

/// 恢复默认类别
pub fn reset_to_defaults(conn: &Connection, amoeba_id: i64) -> Result<CategoryList, String> {
    conn.execute(
        "DELETE FROM amoeba_category WHERE amoeba_id = ?1",
        params![amoeba_id],
    )
    .map_err(|e| e.to_string())?;

    seed_defaults(conn, amoeba_id)?;

    list_by_amoeba(conn, amoeba_id)
}
