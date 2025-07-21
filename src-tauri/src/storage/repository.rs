// リポジトリ
// データベースとのCRUD操作を担当

use crate::models::*;
use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};

/// 設定リポジトリ
/// アプリケーション設定の保存と取得を担当
pub struct ConfigRepository {
    conn: Arc<Mutex<Connection>>,
}

impl ConfigRepository {
    /// 新しい設定リポジトリを作成
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// テーブルを初期化
    pub fn init_table(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    /// 設定値を保存
    pub fn save_config(&self, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO config (key, value) VALUES (?1, ?2)",
            [key, value],
        )?;
        Ok(())
    }

    /// 設定値を取得
    pub fn get_config(&self, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM config WHERE key = ?1")?;
        let mut rows = stmt.query([key])?;
        
        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

/// チケットリポジトリ
/// Backlogから取得したチケット情報のキャッシュを担当
pub struct TicketRepository {
    conn: Arc<Mutex<Connection>>,
}

impl TicketRepository {
    /// 新しいチケットリポジトリを作成
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// テーブルを初期化
    pub fn init_table(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                summary TEXT NOT NULL,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    // 実装は今後追加予定
}