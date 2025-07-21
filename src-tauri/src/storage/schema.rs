// データベーススキーマ定義
// SQLiteテーブル構造の定義

/// データベースのバージョン
pub const DB_VERSION: i32 = 1;

/// データベーススキーマの初期化SQL
pub const INIT_SCHEMA: &str = r#"
-- 設定テーブル
CREATE TABLE IF NOT EXISTS config (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

-- プロジェクト設定テーブル
CREATE TABLE IF NOT EXISTS project_settings (
    project_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    weight INTEGER NOT NULL DEFAULT 5,
    enabled BOOLEAN NOT NULL DEFAULT 1
);

-- チケットキャッシュテーブル
CREATE TABLE IF NOT EXISTS tickets (
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
);

-- AIプロバイダー設定テーブル
CREATE TABLE IF NOT EXISTS ai_providers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    api_key TEXT NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT 1
);

-- バージョン管理テーブル
CREATE TABLE IF NOT EXISTS db_version (
    version INTEGER PRIMARY KEY
);

-- 初期バージョン設定
INSERT OR REPLACE INTO db_version (version) VALUES (1);
"#;