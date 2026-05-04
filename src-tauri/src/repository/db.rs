use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::Manager;

/// 初始化数据库：获取应用数据目录，打开/创建 SQLite 数据库
pub fn init_db(app_handle: &tauri::AppHandle) -> Result<Connection> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| rusqlite::Error::InvalidPath(PathBuf::from(e.to_string())))?;

    // 确保目录存在
    std::fs::create_dir_all(&app_dir)
        .map_err(|e| rusqlite::Error::InvalidPath(PathBuf::from(e.to_string())))?;

    let db_path = app_dir.join("amoeba.db");
    let conn = Connection::open(&db_path)?;

    // 启用外键约束
    conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;

    // 运行数据库迁移
    run_migrations(&conn)?;

    Ok(conn)
}

/// 运行数据库迁移：创建所有表
pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS amoeba (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            name        TEXT NOT NULL,
            amoeba_type TEXT NOT NULL DEFAULT '生产型',
            leader      TEXT NOT NULL DEFAULT '',
            parent_id   INTEGER,
            status      TEXT NOT NULL DEFAULT 'active',
            created_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at  TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (parent_id) REFERENCES amoeba(id) ON DELETE SET NULL
        );

        CREATE TABLE IF NOT EXISTS accounting_record (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            amoeba_id       INTEGER NOT NULL,
            period_type     TEXT NOT NULL DEFAULT 'month',
            period_start    TEXT NOT NULL,
            period_end      TEXT NOT NULL,
            external_sales  REAL NOT NULL DEFAULT 0,
            internal_sales  REAL NOT NULL DEFAULT 0,
            remark          TEXT NOT NULL DEFAULT '',
            total_sales     REAL NOT NULL DEFAULT 0,
            total_expense   REAL NOT NULL DEFAULT 0,
            added_value     REAL NOT NULL DEFAULT 0,
            total_hours     REAL NOT NULL DEFAULT 0,
            unit_value      REAL NOT NULL DEFAULT 0,
            sales_per_person REAL NOT NULL DEFAULT 0,
            value_per_person REAL NOT NULL DEFAULT 0,
            value_rate      REAL NOT NULL DEFAULT 0,
            expense_rate    REAL NOT NULL DEFAULT 0,
            created_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            updated_at      TEXT NOT NULL DEFAULT (datetime('now', 'localtime')),
            FOREIGN KEY (amoeba_id) REFERENCES amoeba(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS expense_detail (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            record_id   INTEGER NOT NULL,
            category    TEXT NOT NULL,
            amount      REAL NOT NULL DEFAULT 0,
            description TEXT NOT NULL DEFAULT '',
            FOREIGN KEY (record_id) REFERENCES accounting_record(id) ON DELETE CASCADE
        );

        CREATE TABLE IF NOT EXISTS labor_time (
            id              INTEGER PRIMARY KEY AUTOINCREMENT,
            record_id       INTEGER NOT NULL UNIQUE,
            normal_hours    REAL NOT NULL DEFAULT 0,
            overtime_hours  REAL NOT NULL DEFAULT 0,
            public_hours    REAL NOT NULL DEFAULT 0,
            headcount       INTEGER NOT NULL DEFAULT 1,
            FOREIGN KEY (record_id) REFERENCES accounting_record(id) ON DELETE CASCADE
        );

        CREATE INDEX IF NOT EXISTS idx_accounting_record_amoeba ON accounting_record(amoeba_id);
        CREATE INDEX IF NOT EXISTS idx_accounting_record_period ON accounting_record(period_type, period_start, period_end);
        CREATE INDEX IF NOT EXISTS idx_expense_detail_record ON expense_detail(record_id);
        CREATE INDEX IF NOT EXISTS idx_labor_time_record ON labor_time(record_id);
        ",
    )?;

    Ok(())
}
