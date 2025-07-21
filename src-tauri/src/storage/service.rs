// ストレージサービス
// データベース操作の高レベルインターフェースを提供

use crate::models::*;
use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

/// ストレージサービス
/// データベースへのアクセスを管理する
pub struct StorageService {
    conn: Arc<Mutex<Connection>>,
}

impl StorageService {
    /// 新しいストレージサービスを作成
    pub fn new(db_path: &Path) -> Result<Self, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// インメモリデータベースを使用したストレージサービスを作成（テスト用）
    #[cfg(test)]
    pub fn new_in_memory() -> Result<Self, rusqlite::Error> {
        let conn = Connection::open_in_memory()?;
        Ok(Self {
            conn: Arc::new(Mutex::new(conn)),
        })
    }

    /// データベース接続を取得
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}