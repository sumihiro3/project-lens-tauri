// リポジトリ
// データベースとのCRUD操作を担当

use rusqlite::{Connection, Result};
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use crate::storage::schema::{INIT_SCHEMA, DB_VERSION, get_migration_sql};
use crate::models::{
    Ticket, BacklogWorkspaceConfig, ProjectWeight, AIAnalysis,
    TicketStatus, Priority
};

/// データベース接続エラー
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("SQLite error: {0}")]
    SqliteError(#[from] rusqlite::Error),
    
    #[error("Database version mismatch: expected {expected}, found {found}")]
    VersionMismatch { expected: i32, found: i32 },
    
    #[error("Migration failed from version {from} to {to}: {reason}")]
    MigrationFailed { from: i32, to: i32, reason: String },
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
}

/// データベース接続管理
/// SQLiteデータベースへの接続とスキーマ管理を担当
pub struct DatabaseConnection {
    conn: Arc<Mutex<Connection>>,
    db_path: PathBuf,
}

impl DatabaseConnection {
    /// 新しいデータベース接続を作成
    /// 
    /// # 引数
    /// * `db_path` - データベースファイルのパス
    /// 
    /// # 戻り値
    /// 初期化されたデータベース接続
    /// 
    /// # エラー
    /// データベース接続またはスキーマ初期化に失敗した場合
    pub fn new(db_path: PathBuf) -> Result<Self, DatabaseError> {
        let conn = Connection::open(&db_path)?;
        let arc_conn = Arc::new(Mutex::new(conn));
        
        let db_connection = Self {
            conn: arc_conn,
            db_path,
        };
        
        // スキーマ初期化とマイグレーション実行
        db_connection.initialize_schema()?;
        
        Ok(db_connection)
    }
    
    /// データベーススキーマの初期化
    /// 新規データベースの場合は最新スキーマを適用、既存の場合はマイグレーション実行
    fn initialize_schema(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        
        // 現在のバージョンを確認
        let current_version = self.get_db_version_internal(&conn)?;
        
        if current_version == 0 {
            // 新規データベース: 最新スキーマを適用
            conn.execute_batch(INIT_SCHEMA)?;
        } else if current_version < DB_VERSION {
            // マイグレーション実行
            self.execute_migration(&conn, current_version, DB_VERSION)?;
        } else if current_version > DB_VERSION {
            return Err(DatabaseError::VersionMismatch {
                expected: DB_VERSION,
                found: current_version,
            });
        }
        
        Ok(())
    }
    
    /// データベースバージョンの取得（内部用）
    fn get_db_version_internal(&self, conn: &Connection) -> Result<i32, DatabaseError> {
        // db_versionテーブルが存在するかチェック
        let table_exists: bool = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='db_version'"
        )?.exists([])?;
        
        if !table_exists {
            return Ok(0); // 新規データベース
        }
        
        // バージョンを取得
        let version: i32 = conn.query_row(
            "SELECT version FROM db_version ORDER BY version DESC LIMIT 1",
            [],
            |row| row.get(0)
        ).unwrap_or(0);
        
        Ok(version)
    }
    
    /// マイグレーション実行
    fn execute_migration(&self, conn: &Connection, from_version: i32, to_version: i32) -> Result<(), DatabaseError> {
        if let Some(migration_sql) = get_migration_sql(from_version, to_version) {
            conn.execute_batch(migration_sql).map_err(|e| {
                DatabaseError::MigrationFailed {
                    from: from_version,
                    to: to_version,
                    reason: e.to_string(),
                }
            })?;
        } else {
            return Err(DatabaseError::MigrationFailed {
                from: from_version,
                to: to_version,
                reason: "No migration path available".to_string(),
            });
        }
        
        Ok(())
    }
    
    /// データベースバージョンの取得（公開API）
    pub fn get_db_version(&self) -> Result<i32, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        self.get_db_version_internal(&conn)
    }
    
    /// データベース接続の取得
    /// Repository実装で使用
    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        Arc::clone(&self.conn)
    }
    
    /// トランザクション開始
    /// 
    /// # 戻り値
    /// トランザクション制御用のTransactionWrapper
    pub fn begin_transaction(&self) -> Result<TransactionWrapper, DatabaseError> {
        let _conn = self.conn.lock().unwrap();
        // Note: 実際のトランザクション管理はPhase 3で詳細実装
        // 現在は基本的な構造のみ提供
        Ok(TransactionWrapper::new())
    }
    
    /// データベースファイルパスの取得
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }
}

/// トランザクション管理ラッパー
/// Phase 3で詳細実装予定
pub struct TransactionWrapper {
    // 実装はPhase 3で追加
}

impl TransactionWrapper {
    fn new() -> Self {
        Self {}
    }
    
    /// トランザクションをコミット
    pub fn commit(self) -> Result<(), DatabaseError> {
        // Phase 3で実装
        Ok(())
    }
    
    /// トランザクションをロールバック
    pub fn rollback(self) -> Result<(), DatabaseError> {
        // Phase 3で実装
        Ok(())
    }
}

/// 設定リポジトリ
/// アプリケーション設定の保存と取得を担当（スキーマv2準拠）
pub struct ConfigRepository {
    conn: Arc<Mutex<Connection>>,
}

impl ConfigRepository {
    /// 新しい設定リポジトリを作成
    /// 
    /// # 引数
    /// * `conn` - データベース接続
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// 設定値を保存
    /// 
    /// # 引数
    /// * `key` - 設定キー
    /// * `value` - 設定値
    /// 
    /// # エラー
    /// データベース操作に失敗した場合
    pub fn save_config(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let now = Utc::now().to_rfc3339();
        
        conn.execute(
            "INSERT OR REPLACE INTO config (key, value, updated_at) VALUES (?1, ?2, ?3)",
            [key, value, &now],
        )?;
        
        Ok(())
    }

    /// 設定値を取得
    /// 
    /// # 引数
    /// * `key` - 設定キー
    /// 
    /// # 戻り値
    /// 設定値（存在しない場合はNone）
    pub fn get_config(&self, key: &str) -> Result<Option<String>, DatabaseError> {
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
    
    /// すべての設定を取得
    /// 
    /// # 戻り値
    /// (key, value)のペアのベクタ
    pub fn get_all_configs(&self) -> Result<Vec<(String, String)>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM config ORDER BY key")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })?;
        
        let mut configs = Vec::new();
        for row in rows {
            configs.push(row?);
        }
        
        Ok(configs)
    }
    
    /// 設定を削除
    /// 
    /// # 引数
    /// * `key` - 削除する設定キー
    pub fn delete_config(&self, key: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM config WHERE key = ?1", [key])?;
        Ok(())
    }
}

/// チケットリポジトリ
/// Backlogから取得したチケット情報のキャッシュを担当（スキーマv2準拠）
pub struct TicketRepository {
    conn: Arc<Mutex<Connection>>,
}

impl TicketRepository {
    /// 新しいチケットリポジトリを作成
    /// 
    /// # 引数
    /// * `conn` - データベース接続
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    /// チケットを保存
    /// 
    /// # 引数
    /// * `ticket` - 保存するチケット
    pub fn save_ticket(&self, ticket: &Ticket) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        
        let status_str = match ticket.status {
            TicketStatus::Open => "Open",
            TicketStatus::InProgress => "InProgress",
            TicketStatus::Resolved => "Resolved",
            TicketStatus::Closed => "Closed",
            TicketStatus::Pending => "Pending",
        };
        
        let priority_int = ticket.priority.clone() as i32;
        
        conn.execute(
            "INSERT OR REPLACE INTO tickets (
                id, project_id, workspace_id, title, description, status, priority,
                assignee_id, reporter_id, created_at, updated_at, due_date, raw_data
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            [
                &ticket.id,
                &ticket.project_id,
                &ticket.workspace_id,
                &ticket.title,
                ticket.description.as_deref().unwrap_or(""),
                status_str,
                &priority_int.to_string(),
                ticket.assignee_id.as_deref().unwrap_or(""),
                &ticket.reporter_id,
                &ticket.created_at.to_rfc3339(),
                &ticket.updated_at.to_rfc3339(),
                ticket.due_date.map(|d| d.to_rfc3339()).as_deref().unwrap_or(""),
                &ticket.raw_data,
            ],
        )?;
        
        Ok(())
    }
    
    /// チケットをIDで取得
    /// 
    /// # 引数
    /// * `ticket_id` - チケットID
    /// 
    /// # 戻り値
    /// チケット（存在しない場合はNone）
    pub fn get_ticket_by_id(&self, ticket_id: &str) -> Result<Option<Ticket>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, workspace_id, title, description, status, priority,
                    assignee_id, reporter_id, created_at, updated_at, due_date, raw_data
             FROM tickets WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query([ticket_id])?;
        
        if let Some(row) = rows.next()? {
            let ticket = self.row_to_ticket(row)?;
            Ok(Some(ticket))
        } else {
            Ok(None)
        }
    }
    
    /// ワークスペースIDでチケット一覧を取得
    /// 
    /// # 引数
    /// * `workspace_id` - ワークスペースID
    /// 
    /// # 戻り値
    /// チケット一覧
    pub fn get_tickets_by_workspace(&self, workspace_id: &str) -> Result<Vec<Ticket>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, project_id, workspace_id, title, description, status, priority,
                    assignee_id, reporter_id, created_at, updated_at, due_date, raw_data
             FROM tickets WHERE workspace_id = ?1 ORDER BY updated_at DESC"
        )?;
        
        let mut tickets = Vec::new();
        let mut rows = stmt.query([workspace_id])?;
        
        while let Some(row) = rows.next()? {
            tickets.push(self.row_to_ticket(row)?);
        }
        
        Ok(tickets)
    }
    
    /// 複数チケットの一括保存
    /// 
    /// # 引数
    /// * `tickets` - 保存するチケット一覧
    pub fn save_tickets(&self, tickets: &[Ticket]) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let tx = conn.unchecked_transaction()?;
        
        for ticket in tickets {
            // save_ticketのロジックを展開（トランザクション内で実行）
            let status_str = match ticket.status {
                TicketStatus::Open => "Open",
                TicketStatus::InProgress => "InProgress",
                TicketStatus::Resolved => "Resolved",
                TicketStatus::Closed => "Closed",
                TicketStatus::Pending => "Pending",
            };
            
            let priority_int = ticket.priority.clone() as i32;
            
            tx.execute(
                "INSERT OR REPLACE INTO tickets (
                    id, project_id, workspace_id, title, description, status, priority,
                    assignee_id, reporter_id, created_at, updated_at, due_date, raw_data
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
                [
                    &ticket.id,
                    &ticket.project_id,
                    &ticket.workspace_id,
                    &ticket.title,
                    ticket.description.as_deref().unwrap_or(""),
                    status_str,
                    &priority_int.to_string(),
                    ticket.assignee_id.as_deref().unwrap_or(""),
                    &ticket.reporter_id,
                    &ticket.created_at.to_rfc3339(),
                    &ticket.updated_at.to_rfc3339(),
                    ticket.due_date.map(|d| d.to_rfc3339()).as_deref().unwrap_or(""),
                    &ticket.raw_data,
                ],
            )?;
        }
        
        tx.commit()?;
        Ok(())
    }
    
    /// SQLiteの行をTicket構造体に変換
    fn row_to_ticket(&self, row: &rusqlite::Row) -> Result<Ticket, DatabaseError> {
        let status_str: String = row.get(5)?;
        let status = match status_str.as_str() {
            "Open" => TicketStatus::Open,
            "InProgress" => TicketStatus::InProgress,
            "Resolved" => TicketStatus::Resolved,
            "Closed" => TicketStatus::Closed,
            "Pending" => TicketStatus::Pending,
            _ => TicketStatus::Open, // デフォルト
        };
        
        let priority_int: i32 = row.get::<_, String>(6)?.parse().unwrap_or(2);
        let priority = match priority_int {
            1 => Priority::Low,
            2 => Priority::Normal,
            3 => Priority::High,
            4 => Priority::Critical,
            _ => Priority::Normal,
        };
        
        let created_at_str: String = row.get(9)?;
        let updated_at_str: String = row.get(10)?;
        let due_date_str: Option<String> = row.get(11)?;
        
        Ok(Ticket {
            id: row.get(0)?,
            project_id: row.get(1)?,
            workspace_id: row.get(2)?,
            title: row.get(3)?,
            description: row.get(4)?,
            status,
            priority,
            assignee_id: row.get(7)?,
            reporter_id: row.get(8)?,
            created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap().with_timezone(&Utc),
            due_date: due_date_str.map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
            raw_data: row.get(12)?,
        })
    }
}

/// ワークスペース設定リポジトリ
/// Backlogワークスペース設定の保存と取得を担当（スキーマv2準拠）
pub struct WorkspaceRepository {
    conn: Arc<Mutex<Connection>>,
}

impl WorkspaceRepository {
    /// 新しいワークスペースリポジトリを作成
    /// 
    /// # 引数
    /// * `conn` - データベース接続
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
    
    /// ワークスペース設定を保存
    /// 
    /// # 引数
    /// * `workspace` - 保存するワークスペース設定
    pub fn save_workspace(&self, workspace: &BacklogWorkspaceConfig) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO workspaces (
                id, name, domain, api_key_encrypted, encryption_version, enabled, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            [
                &workspace.id,
                &workspace.name,
                &workspace.domain,
                &workspace.api_key_encrypted,
                &workspace.encryption_version,
                &workspace.enabled.to_string(),
                &workspace.created_at.to_rfc3339(),
                &workspace.updated_at.to_rfc3339(),
            ],
        )?;
        
        Ok(())
    }
    
    /// ワークスペース設定をIDで取得
    /// 
    /// # 引数
    /// * `workspace_id` - ワークスペースID
    /// 
    /// # 戻り値
    /// ワークスペース設定（存在しない場合はNone）
    pub fn get_workspace_by_id(&self, workspace_id: &str) -> Result<Option<BacklogWorkspaceConfig>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, domain, api_key_encrypted, encryption_version, enabled, created_at, updated_at
             FROM workspaces WHERE id = ?1"
        )?;
        
        let mut rows = stmt.query([workspace_id])?;
        
        if let Some(row) = rows.next()? {
            let workspace = self.row_to_workspace(row)?;
            Ok(Some(workspace))
        } else {
            Ok(None)
        }
    }
    
    /// 有効なワークスペース一覧を取得
    /// 
    /// # 戻り値
    /// 有効なワークスペース設定一覧
    pub fn get_enabled_workspaces(&self) -> Result<Vec<BacklogWorkspaceConfig>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, name, domain, api_key_encrypted, encryption_version, enabled, created_at, updated_at
             FROM workspaces WHERE enabled = 'true' ORDER BY name"
        )?;
        
        let mut workspaces = Vec::new();
        let mut rows = stmt.query([])?;
        
        while let Some(row) = rows.next()? {
            workspaces.push(self.row_to_workspace(row)?);
        }
        
        Ok(workspaces)
    }
    
    /// ワークスペースを削除
    /// 
    /// # 引数
    /// * `workspace_id` - 削除するワークスペースID
    pub fn delete_workspace(&self, workspace_id: &str) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM workspaces WHERE id = ?1", [workspace_id])?;
        Ok(())
    }
    
    /// SQLiteの行をBacklogWorkspaceConfig構造体に変換
    fn row_to_workspace(&self, row: &rusqlite::Row) -> Result<BacklogWorkspaceConfig, DatabaseError> {
        let enabled_str: String = row.get(5)?;
        let enabled = enabled_str == "true";
        
        let created_at_str: String = row.get(6)?;
        let updated_at_str: String = row.get(7)?;
        
        Ok(BacklogWorkspaceConfig {
            id: row.get(0)?,
            name: row.get(1)?,
            domain: row.get(2)?,
            api_key_encrypted: row.get(3)?,
            encryption_version: row.get(4)?,
            enabled,
            created_at: DateTime::parse_from_rfc3339(&created_at_str).unwrap().with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap().with_timezone(&Utc),
        })
    }
}

/// プロジェクト重み設定リポジトリ
/// プロジェクト重み設定の保存と取得を担当（スキーマv2準拠）
pub struct ProjectWeightRepository {
    conn: Arc<Mutex<Connection>>,
}

impl ProjectWeightRepository {
    /// 新しいプロジェクト重みリポジトリを作成
    /// 
    /// # 引数
    /// * `conn` - データベース接続
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
    
    /// プロジェクト重み設定を保存
    /// 
    /// # 引数
    /// * `project_weight` - 保存するプロジェクト重み設定
    pub fn save_project_weight(&self, project_weight: &ProjectWeight) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO project_weights (
                project_id, project_name, workspace_id, weight_score, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5)",
            [
                &project_weight.project_id,
                &project_weight.project_name,
                &project_weight.workspace_id,
                &project_weight.weight_score.to_string(),
                &project_weight.updated_at.to_rfc3339(),
            ],
        )?;
        
        Ok(())
    }
    
    /// プロジェクト重み設定をIDで取得
    /// 
    /// # 引数
    /// * `project_id` - プロジェクトID
    /// 
    /// # 戻り値
    /// プロジェクト重み設定（存在しない場合はNone）
    pub fn get_project_weight_by_id(&self, project_id: &str) -> Result<Option<ProjectWeight>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT project_id, project_name, workspace_id, weight_score, updated_at
             FROM project_weights WHERE project_id = ?1"
        )?;
        
        let mut rows = stmt.query([project_id])?;
        
        if let Some(row) = rows.next()? {
            let project_weight = self.row_to_project_weight(row)?;
            Ok(Some(project_weight))
        } else {
            Ok(None)
        }
    }
    
    /// ワークスペースのプロジェクト重み一覧を取得
    /// 
    /// # 引数
    /// * `workspace_id` - ワークスペースID
    /// 
    /// # 戻り値
    /// プロジェクト重み設定一覧
    pub fn get_project_weights_by_workspace(&self, workspace_id: &str) -> Result<Vec<ProjectWeight>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT project_id, project_name, workspace_id, weight_score, updated_at
             FROM project_weights WHERE workspace_id = ?1 ORDER BY project_name"
        )?;
        
        let mut project_weights = Vec::new();
        let mut rows = stmt.query([workspace_id])?;
        
        while let Some(row) = rows.next()? {
            project_weights.push(self.row_to_project_weight(row)?);
        }
        
        Ok(project_weights)
    }
    
    /// SQLiteの行をProjectWeight構造体に変換
    fn row_to_project_weight(&self, row: &rusqlite::Row) -> Result<ProjectWeight, DatabaseError> {
        let weight_score_str: String = row.get(3)?;
        let weight_score: u8 = weight_score_str.parse().unwrap_or(5);
        
        let updated_at_str: String = row.get(4)?;
        
        Ok(ProjectWeight {
            project_id: row.get(0)?,
            project_name: row.get(1)?,
            workspace_id: row.get(2)?,
            weight_score,
            updated_at: DateTime::parse_from_rfc3339(&updated_at_str).unwrap().with_timezone(&Utc),
        })
    }
}

/// AI分析結果リポジトリ
/// AI分析結果の保存と取得を担当（スキーマv2準拠）
pub struct AIAnalysisRepository {
    conn: Arc<Mutex<Connection>>,
}

impl AIAnalysisRepository {
    /// 新しいAI分析結果リポジトリを作成
    /// 
    /// # 引数
    /// * `conn` - データベース接続
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
    
    /// AI分析結果を保存
    /// 
    /// # 引数
    /// * `analysis` - 保存するAI分析結果
    pub fn save_ai_analysis(&self, analysis: &AIAnalysis) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO ai_analyses (
                ticket_id, urgency_score, complexity_score, user_relevance_score,
                project_weight_factor, final_priority_score, recommendation_reason,
                category, analyzed_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            [
                &analysis.ticket_id,
                &analysis.urgency_score.to_string(),
                &analysis.complexity_score.to_string(),
                &analysis.user_relevance_score.to_string(),
                &analysis.project_weight_factor.to_string(),
                &analysis.final_priority_score.to_string(),
                &analysis.recommendation_reason,
                &analysis.category,
                &analysis.analyzed_at.to_rfc3339(),
            ],
        )?;
        
        Ok(())
    }
    
    /// AI分析結果をチケットIDで取得
    /// 
    /// # 引数
    /// * `ticket_id` - チケットID
    /// 
    /// # 戻り値
    /// AI分析結果（存在しない場合はNone）
    pub fn get_ai_analysis_by_ticket_id(&self, ticket_id: &str) -> Result<Option<AIAnalysis>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT ticket_id, urgency_score, complexity_score, user_relevance_score,
                    project_weight_factor, final_priority_score, recommendation_reason,
                    category, analyzed_at
             FROM ai_analyses WHERE ticket_id = ?1"
        )?;
        
        let mut rows = stmt.query([ticket_id])?;
        
        if let Some(row) = rows.next()? {
            let analysis = self.row_to_ai_analysis(row)?;
            Ok(Some(analysis))
        } else {
            Ok(None)
        }
    }
    
    /// SQLiteの行をAIAnalysis構造体に変換
    fn row_to_ai_analysis(&self, row: &rusqlite::Row) -> Result<AIAnalysis, DatabaseError> {
        let urgency_score: String = row.get(1)?;
        let complexity_score: String = row.get(2)?;
        let user_relevance_score: String = row.get(3)?;
        let project_weight_factor: String = row.get(4)?;
        let final_priority_score: String = row.get(5)?;
        let analyzed_at_str: String = row.get(8)?;
        
        Ok(AIAnalysis {
            ticket_id: row.get(0)?,
            urgency_score: urgency_score.parse().unwrap_or(0.0),
            complexity_score: complexity_score.parse().unwrap_or(0.0),
            user_relevance_score: user_relevance_score.parse().unwrap_or(0.0),
            project_weight_factor: project_weight_factor.parse().unwrap_or(1.0),
            final_priority_score: final_priority_score.parse().unwrap_or(0.0),
            recommendation_reason: row.get(6)?,
            category: row.get(7)?,
            analyzed_at: DateTime::parse_from_rfc3339(&analyzed_at_str).unwrap().with_timezone(&Utc),
        })
    }
}