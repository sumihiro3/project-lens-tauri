// データベーススキーマ定義
// SQLiteテーブル構造の定義

/// データベースのバージョン（技術仕様書準拠に更新）
pub const DB_VERSION: i32 = 2;

/// データベーススキーマの初期化SQL（技術仕様書完全準拠）
pub const INIT_SCHEMA: &str = r#"
-- チケットテーブル（技術仕様書準拠）
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL,
    assignee_id TEXT,
    reporter_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT,
    raw_data TEXT NOT NULL -- JSON形式でオリジナルデータを保存
);

-- ワークスペーステーブル（技術仕様書準拠）
CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    encryption_version TEXT NOT NULL DEFAULT 'v1',
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- プロジェクト重みテーブル（技術仕様書準拠）
CREATE TABLE IF NOT EXISTS project_weights (
    project_id TEXT PRIMARY KEY,
    project_name TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    weight_score INTEGER NOT NULL CHECK (weight_score BETWEEN 1 AND 10),
    updated_at TEXT NOT NULL,
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);

-- AI分析結果テーブル（技術仕様書準拠）
CREATE TABLE IF NOT EXISTS ai_analyses (
    ticket_id TEXT PRIMARY KEY,
    urgency_score REAL NOT NULL,
    complexity_score REAL NOT NULL,
    user_relevance_score REAL NOT NULL,
    project_weight_factor REAL NOT NULL,
    final_priority_score REAL NOT NULL,
    recommendation_reason TEXT NOT NULL,
    category TEXT NOT NULL,
    analyzed_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id)
);

-- 設定テーブル（汎用設定管理）
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- バージョン管理テーブル
CREATE TABLE IF NOT EXISTS db_version (
    version INTEGER PRIMARY KEY
);

-- インデックス作成（パフォーマンス最適化）
CREATE INDEX IF NOT EXISTS idx_tickets_workspace_id ON tickets(workspace_id);
CREATE INDEX IF NOT EXISTS idx_tickets_project_id ON tickets(project_id);
CREATE INDEX IF NOT EXISTS idx_tickets_assignee_id ON tickets(assignee_id);
CREATE INDEX IF NOT EXISTS idx_tickets_status ON tickets(status);
CREATE INDEX IF NOT EXISTS idx_tickets_priority ON tickets(priority);
CREATE INDEX IF NOT EXISTS idx_tickets_updated_at ON tickets(updated_at);
CREATE INDEX IF NOT EXISTS idx_project_weights_workspace_id ON project_weights(workspace_id);
CREATE INDEX IF NOT EXISTS idx_ai_analyses_final_priority_score ON ai_analyses(final_priority_score DESC);
CREATE INDEX IF NOT EXISTS idx_ai_analyses_analyzed_at ON ai_analyses(analyzed_at);

-- バージョン設定更新
INSERT OR REPLACE INTO db_version (version) VALUES (2);
"#;

/// マイグレーションSQL（v1からv2への移行）
pub const MIGRATION_V1_TO_V2: &str = r#"
-- 既存のtickets テーブルを一時テーブルに移動
ALTER TABLE tickets RENAME TO tickets_old;

-- 新しいtickets テーブルを作成（技術仕様書準拠）
CREATE TABLE tickets (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL DEFAULT 'default_workspace',
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL DEFAULT 2,
    assignee_id TEXT,
    reporter_id TEXT NOT NULL DEFAULT 'unknown',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT,
    raw_data TEXT NOT NULL DEFAULT '{}'
);

-- データ移行（可能な範囲で）
INSERT INTO tickets (
    id, project_id, workspace_id, title, description, status, 
    priority, assignee_id, reporter_id, created_at, updated_at, raw_data
)
SELECT 
    id, 
    project_id, 
    'default_workspace',  -- 既存データは全てdefault_workspaceに割り当て
    COALESCE(summary, title, '無題'),  -- summary または title を使用
    description,
    status,
    CASE 
        WHEN priority = 'Critical' THEN 4
        WHEN priority = 'High' THEN 3
        WHEN priority = 'Normal' THEN 2
        ELSE 1
    END,  -- 文字列の優先度を数値に変換
    assignee,
    'unknown',  -- reporter情報がないため
    created_at,
    updated_at,
    COALESCE(data, '{}')  -- data をraw_data として使用
FROM tickets_old;

-- 一時テーブルを削除
DROP TABLE tickets_old;

-- 新しいテーブル作成
CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    encryption_version TEXT NOT NULL DEFAULT 'v1',
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_weights (
    project_id TEXT PRIMARY KEY,
    project_name TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    weight_score INTEGER NOT NULL CHECK (weight_score BETWEEN 1 AND 10),
    updated_at TEXT NOT NULL,
    FOREIGN KEY (workspace_id) REFERENCES workspaces(id)
);

CREATE TABLE IF NOT EXISTS ai_analyses (
    ticket_id TEXT PRIMARY KEY,
    urgency_score REAL NOT NULL,
    complexity_score REAL NOT NULL,
    user_relevance_score REAL NOT NULL,
    project_weight_factor REAL NOT NULL,
    final_priority_score REAL NOT NULL,
    recommendation_reason TEXT NOT NULL,
    category TEXT NOT NULL,
    analyzed_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id)
);

-- インデックス作成
CREATE INDEX idx_tickets_workspace_id ON tickets(workspace_id);
CREATE INDEX idx_tickets_project_id ON tickets(project_id);
CREATE INDEX idx_tickets_assignee_id ON tickets(assignee_id);
CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_priority ON tickets(priority);
CREATE INDEX idx_tickets_updated_at ON tickets(updated_at);
CREATE INDEX idx_project_weights_workspace_id ON project_weights(workspace_id);
CREATE INDEX idx_ai_analyses_final_priority_score ON ai_analyses(final_priority_score DESC);
CREATE INDEX idx_ai_analyses_analyzed_at ON ai_analyses(analyzed_at);

-- バージョン更新
UPDATE db_version SET version = 2;
"#;

/// データベース初期化関数
pub fn get_schema_for_version(version: i32) -> &'static str {
    match version {
        1 => panic!("Version 1 is deprecated. Please migrate to version 2."),
        2 => INIT_SCHEMA,
        _ => panic!("Unsupported database version: {}", version),
    }
}

/// マイグレーション取得関数
pub fn get_migration_sql(from_version: i32, to_version: i32) -> Option<&'static str> {
    match (from_version, to_version) {
        (1, 2) => Some(MIGRATION_V1_TO_V2),
        _ => None,
    }
}