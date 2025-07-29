//! データベーススキーマのテスト
//! レビューフィードバック適用：マイグレーション機能の完全性テスト

#[cfg(test)]
mod tests {
    use rusqlite::{Connection, Result};
    use tempfile::NamedTempFile;
    use super::super::schema::{DB_VERSION, INIT_SCHEMA, MIGRATION_V1_TO_V2, get_schema_for_version, get_migration_sql};

    /// テスト用のインメモリデータベース接続を作成
    fn create_test_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok(conn)
    }

    /// テスト用の一時ファイルデータベース接続を作成
    fn create_temp_file_db() -> Result<(Connection, NamedTempFile)> {
        let temp_file = NamedTempFile::new().expect("一時ファイルの作成に失敗");
        let conn = Connection::open(temp_file.path())?;
        conn.execute("PRAGMA foreign_keys = ON;", [])?;
        Ok((conn, temp_file))
    }

    /// v1スキーマのテストデータ設定（マイグレーション前）
    fn setup_v1_schema(conn: &Connection) -> Result<()> {
        // v1 スキーマの模擬（旧形式）
        conn.execute_batch(r#"
            CREATE TABLE tickets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT,
                summary TEXT,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,  -- 文字列形式の優先度
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data TEXT  -- 旧形式のデータフィールド
            );

            CREATE TABLE db_version (
                version INTEGER PRIMARY KEY
            );

            INSERT INTO db_version (version) VALUES (1);
        "#)?;

        // テストデータ挿入
        conn.execute(r#"
            INSERT INTO tickets (
                id, project_id, title, summary, description, status, priority, 
                assignee, created_at, updated_at, data
            ) VALUES (
                'ticket-1', 'project-1', 'タイトル1', 'サマリー1', '説明1', 
                'open', 'High', 'user1', '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z',
                '{"original": "data"}'
            )
        "#, [])?;

        conn.execute(r#"
            INSERT INTO tickets (
                id, project_id, title, status, priority, 
                created_at, updated_at
            ) VALUES (
                'ticket-2', 'project-2', 'タイトル2', 'closed', 'Critical',
                '2025-01-02T00:00:00Z', '2025-01-02T00:00:00Z'
            )
        "#, [])?;

        Ok(())
    }

    #[test]
    fn test_db_version_constant() {
        assert_eq!(DB_VERSION, 2, "DBバージョンは2である必要があります");
    }

    #[test]
    fn test_init_schema_execution() -> Result<()> {
        let conn = create_test_db()?;
        
        // スキーマを実行
        conn.execute_batch(INIT_SCHEMA)?;
        
        // バージョンチェック
        let version: i32 = conn.query_row("SELECT version FROM db_version", [], |row| {
            row.get(0)
        })?;
        assert_eq!(version, 2);
        
        Ok(())
    }

    #[test]
    fn test_all_tables_created() -> Result<()> {
        let conn = create_test_db()?;
        conn.execute_batch(INIT_SCHEMA)?;
        
        // 全テーブルの存在確認
        let tables = vec![
            "tickets", "workspaces", "project_weights", 
            "ai_analyses", "config", "db_version"
        ];
        
        for table in tables {
            let count: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
                [table],
                |row| row.get(0)
            )?;
            assert_eq!(count, 1, "テーブル '{}' が作成されていません", table);
        }
        
        Ok(())
    }

    #[test]
    fn test_all_indexes_created() -> Result<()> {
        let conn = create_test_db()?;
        conn.execute_batch(INIT_SCHEMA)?;
        
        // インデックスの存在確認
        let expected_indexes = vec![
            "idx_tickets_workspace_id",
            "idx_tickets_project_id", 
            "idx_tickets_assignee_id",
            "idx_tickets_status",
            "idx_tickets_priority",
            "idx_tickets_updated_at",
            "idx_project_weights_workspace_id",
            "idx_ai_analyses_final_priority_score",
            "idx_ai_analyses_analyzed_at"
        ];
        
        for index in expected_indexes {
            let count: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name=?",
                [index],
                |row| row.get(0)
            )?;
            assert_eq!(count, 1, "インデックス '{}' が作成されていません", index);
        }
        
        Ok(())
    }

    #[test]
    fn test_foreign_key_constraints() -> Result<()> {
        let conn = create_test_db()?;
        conn.execute_batch(INIT_SCHEMA)?;
        
        // ワークスペースデータ挿入
        conn.execute(r#"
            INSERT INTO workspaces (
                id, name, domain, api_key_encrypted, created_at, updated_at
            ) VALUES (
                'ws1', 'テストワークスペース', 'test.backlog.jp', 
                'encrypted_key', '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z'
            )
        "#, [])?;

        // チケットデータ挿入
        conn.execute(r#"
            INSERT INTO tickets (
                id, project_id, workspace_id, title, status, priority,
                reporter_id, created_at, updated_at, raw_data
            ) VALUES (
                'ticket1', 'proj1', 'ws1', 'テストチケット', 'open', 2,
                'reporter1', '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z', '{}'
            )
        "#, [])?;

        // プロジェクト重みデータ挿入（外部キー制約テスト）
        let result = conn.execute(r#"
            INSERT INTO project_weights (
                project_id, project_name, workspace_id, weight_score, updated_at
            ) VALUES (
                'proj1', 'テストプロジェクト', 'ws1', 5, '2025-01-01T00:00:00Z'
            )
        "#, []);
        assert!(result.is_ok(), "有効な外部キーでの挿入が失敗しました");

        // 無効な外部キーでの挿入テスト
        let invalid_result = conn.execute(r#"
            INSERT INTO project_weights (
                project_id, project_name, workspace_id, weight_score, updated_at
            ) VALUES (
                'proj2', 'テストプロジェクト2', 'invalid_ws', 5, '2025-01-01T00:00:00Z'
            )
        "#, []);
        assert!(invalid_result.is_err(), "無効な外部キーでの挿入が成功してしまいました");

        Ok(())
    }

    #[test]
    fn test_check_constraints() -> Result<()> {
        let conn = create_test_db()?;
        conn.execute_batch(INIT_SCHEMA)?;

        // ワークスペースデータ挿入
        conn.execute(r#"
            INSERT INTO workspaces (
                id, name, domain, api_key_encrypted, created_at, updated_at
            ) VALUES (
                'ws1', 'テストワークスペース', 'test.backlog.jp', 
                'encrypted_key', '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z'
            )
        "#, [])?;

        // 有効な重み値（1-10）のテスト
        let valid_weights = vec![1, 5, 10];
        for weight in valid_weights {
            let result = conn.execute(r#"
                INSERT INTO project_weights (
                    project_id, project_name, workspace_id, weight_score, updated_at
                ) VALUES (?, 'テストプロジェクト', 'ws1', ?, '2025-01-01T00:00:00Z')
            "#, [&format!("proj_{}", weight), &weight.to_string()]);
            assert!(result.is_ok(), "有効な重み値 {} での挿入が失敗しました", weight);
        }

        // 無効な重み値のテスト
        let invalid_weights = vec![0, -1, 11, 100];
        for weight in invalid_weights {
            let result = conn.execute(r#"
                INSERT INTO project_weights (
                    project_id, project_name, workspace_id, weight_score, updated_at
                ) VALUES (?, 'テストプロジェクト', 'ws1', ?, '2025-01-01T00:00:00Z')
            "#, [&format!("proj_invalid_{}", weight), &weight.to_string()]);
            assert!(result.is_err(), "無効な重み値 {} での挿入が成功してしまいました", weight);
        }

        Ok(())
    }

    #[test]
    fn test_migration_v1_to_v2_execution() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定
        setup_v1_schema(&conn)?;
        
        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;
        
        // バージョンが2に更新されていることを確認
        let version: i32 = conn.query_row("SELECT version FROM db_version", [], |row| {
            row.get(0)
        })?;
        assert_eq!(version, 2);
        
        Ok(())
    }

    #[test]
    fn test_migration_data_preservation() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定
        setup_v1_schema(&conn)?;
        
        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;
        
        // データが保持されていることを確認
        let mut stmt = conn.prepare("SELECT id, project_id, workspace_id, title, priority, raw_data FROM tickets ORDER BY id")?;
        let rows: Result<Vec<_>> = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,  // id
                row.get::<_, String>(1)?,  // project_id
                row.get::<_, String>(2)?,  // workspace_id
                row.get::<_, String>(3)?,  // title
                row.get::<_, i32>(4)?,     // priority (now integer)
                row.get::<_, String>(5)?,  // raw_data
            ))
        })?.collect();
        
        let tickets = rows?;
        assert_eq!(tickets.len(), 2, "マイグレーション後にチケット数が一致しません");
        
        // ticket-1の検証
        let ticket1 = &tickets[0];
        assert_eq!(ticket1.0, "ticket-1");
        assert_eq!(ticket1.1, "project-1");
        assert_eq!(ticket1.2, "default_workspace"); // デフォルトワークスペースに設定
        assert_eq!(ticket1.3, "サマリー1"); // summary が title として使用
        assert_eq!(ticket1.4, 3); // "High" -> 3
        assert_eq!(ticket1.5, r#"{"original": "data"}"#);
        
        // ticket-2の検証
        let ticket2 = &tickets[1];
        assert_eq!(ticket2.0, "ticket-2");
        assert_eq!(ticket2.1, "project-2");
        assert_eq!(ticket2.2, "default_workspace");
        assert_eq!(ticket2.3, "タイトル2");
        assert_eq!(ticket2.4, 4); // "Critical" -> 4
        assert_eq!(ticket2.5, "{}"); // デフォルト値
        
        Ok(())
    }

    #[test]
    fn test_migration_new_tables_created() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定
        setup_v1_schema(&conn)?;
        
        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;
        
        // 新しいテーブルが作成されていることを確認
        let new_tables = vec!["workspaces", "project_weights", "ai_analyses"];
        
        for table in new_tables {
            let count: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?",
                [table],
                |row| row.get(0)
            )?;
            assert_eq!(count, 1, "マイグレーション後にテーブル '{}' が作成されていません", table);
        }
        
        Ok(())
    }

    #[test]
    fn test_migration_indexes_created() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定
        setup_v1_schema(&conn)?;
        
        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;
        
        // インデックスが作成されていることを確認
        let expected_indexes = vec![
            "idx_tickets_workspace_id",
            "idx_tickets_project_id",
            "idx_tickets_assignee_id",
            "idx_tickets_status",
            "idx_tickets_priority",
            "idx_tickets_updated_at",
            "idx_project_weights_workspace_id",
            "idx_ai_analyses_final_priority_score",
            "idx_ai_analyses_analyzed_at"
        ];
        
        for index in expected_indexes {
            let count: i32 = conn.query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='index' AND name=?",
                [index],
                |row| row.get(0)
            )?;
            assert_eq!(count, 1, "マイグレーション後にインデックス '{}' が作成されていません", index);
        }
        
        Ok(())
    }

    #[test]
    fn test_get_schema_for_version() {
        // バージョン2のスキーマ取得
        let schema = get_schema_for_version(2);
        assert_eq!(schema, INIT_SCHEMA);
    }

    #[test]
    #[should_panic(expected = "Version 1 is deprecated")]
    fn test_get_schema_for_version_v1_panics() {
        get_schema_for_version(1);
    }

    #[test]
    #[should_panic(expected = "Unsupported database version")]
    fn test_get_schema_for_version_invalid_panics() {
        get_schema_for_version(999);
    }

    #[test]
    fn test_get_migration_sql() {
        // v1からv2へのマイグレーション取得
        let migration = get_migration_sql(1, 2);
        assert!(migration.is_some());
        assert_eq!(migration.unwrap(), MIGRATION_V1_TO_V2);
        
        // サポートされていないマイグレーション
        let invalid_migration = get_migration_sql(2, 3);
        assert!(invalid_migration.is_none());
        
        let reverse_migration = get_migration_sql(2, 1);
        assert!(reverse_migration.is_none());
    }

    #[test]
    fn test_priority_mapping_completeness() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定（全優先度タイプをテスト）
        conn.execute_batch(r#"
            CREATE TABLE tickets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT,
                summary TEXT,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data TEXT
            );

            CREATE TABLE db_version (
                version INTEGER PRIMARY KEY
            );

            INSERT INTO db_version (version) VALUES (1);
        "#)?;

        // 各優先度レベルのテストデータ
        let priorities = vec![
            ("ticket-critical", "Critical", 4),
            ("ticket-high", "High", 3),
            ("ticket-normal", "Normal", 2),
            ("ticket-low", "Low", 1),
            ("ticket-unknown", "Unknown", 1), // デフォルト値
        ];

        for (id, priority_str, _) in &priorities {
            conn.execute(r#"
                INSERT INTO tickets (id, project_id, title, status, priority, created_at, updated_at)
                VALUES (?, 'proj', 'タイトル', 'open', ?, '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z')
            "#, [id, priority_str])?;
        }

        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;

        // 優先度マッピングの確認
        for (id, _, expected_priority) in priorities {
            let actual_priority: i32 = conn.query_row(
                "SELECT priority FROM tickets WHERE id = ?",
                [id],
                |row| row.get(0)
            )?;
            assert_eq!(actual_priority, expected_priority, 
                      "チケット {} の優先度マッピングが正しくありません", id);
        }

        Ok(())
    }

    #[test]
    fn test_database_integrity_after_migration() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定
        setup_v1_schema(&conn)?;
        
        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;
        
        // データベース整合性チェック
        let integrity_result: String = conn.query_row("PRAGMA integrity_check", [], |row| {
            row.get(0)
        })?;
        assert_eq!(integrity_result, "ok", "マイグレーション後のデータベース整合性チェックに失敗しました");
        
        // 外部キー整合性チェック
        let foreign_key_result: String = conn.query_row("PRAGMA foreign_key_check", [], |row| {
            row.get(0)
        }).unwrap_or_else(|_| "ok".to_string()); // エラーがなければ結果なし
        
        Ok(())
    }

    #[test]
    fn test_migration_with_null_data_handling() -> Result<()> {
        let conn = create_test_db()?;
        
        // v1スキーマ設定（NULLデータを含む）
        conn.execute_batch(r#"
            CREATE TABLE tickets (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                title TEXT,
                summary TEXT,
                description TEXT,
                status TEXT NOT NULL,
                priority TEXT NOT NULL,
                assignee TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                data TEXT
            );

            CREATE TABLE db_version (
                version INTEGER PRIMARY KEY
            );

            INSERT INTO db_version (version) VALUES (1);
        "#)?;

        // NULLデータを含むテストデータ
        conn.execute(r#"
            INSERT INTO tickets (
                id, project_id, status, priority, created_at, updated_at
            ) VALUES (
                'ticket-null', 'project-1', 'open', 'Normal', 
                '2025-01-01T00:00:00Z', '2025-01-01T00:00:00Z'
            )
        "#, [])?;

        // マイグレーション実行
        conn.execute_batch(MIGRATION_V1_TO_V2)?;

        // NULLデータが適切にデフォルト値に変換されることを確認
        let mut stmt = conn.prepare("SELECT title, description, assignee_id, reporter_id, raw_data FROM tickets WHERE id = 'ticket-null'")?;
        let row = stmt.query_row([], |row| {
            Ok((
                row.get::<_, String>(0)?,  // title
                row.get::<_, Option<String>>(1)?,  // description
                row.get::<_, Option<String>>(2)?,  // assignee_id
                row.get::<_, String>(3)?,  // reporter_id
                row.get::<_, String>(4)?,  // raw_data
            ))
        })?;

        assert_eq!(row.0, "無題"); // title の デフォルト値
        assert_eq!(row.1, None); // description は NULL のまま
        assert_eq!(row.2, None); // assignee_id は NULL のまま
        assert_eq!(row.3, "unknown"); // reporter_id の デフォルト値
        assert_eq!(row.4, "{}"); // raw_data の デフォルト値

        Ok(())
    }
}