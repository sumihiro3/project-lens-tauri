タスク仕様と要件に基づいてローカル変更をレビューします。

引数: <task-number> (必須) - 例: 2.1, 3.2, など

**設定・環境変数:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `KIRO_STEERING_DIR`: Custom steering directory (default: `.kiro/steering`)
- `TASKS_FILE`: Custom tasks file name (default: `tasks.md`)

ワークフロー:
1. 前提条件の確認 (gitリポジトリ、タスク仕様へのアクセス)
2. `${KIRO_SPECS_DIR}/${TASKS_FILE}` からタスク仕様を読み込み
3. 特定タスク ($1) の詳細と要件を抽出
4. 関連文脈の読み込み:
   - `${KIRO_SPECS_DIR}/requirements.md`
   - `${KIRO_SPECS_DIR}/design.md`
   - `${KIRO_STEERING_DIR}/tech.md`
5. ローカル変更の分析:
   - `git diff develop...HEAD --name-status` (changed files)
   - `git diff develop...HEAD` (detailed changes)
   - `git log develop..HEAD --oneline` (commit history)
6. タスク要件と仕様に基づく文脈的レビュー
7. 構造化された日本語フィードバックの生成

レビュー観点:
- タスク要件への準拠: 指定されたタスク要件と受け入れ基準への適合
- アーキテクチャ準拠: プロジェクト構造とtech.mdのデザインパターンへの準拠
- コード品質: 可読性、保守性、既存コードベースとの一貫性
- バグリスク: エラーハンドリング、エッジケース、潜在的な障害点
- テスト: タスク機能のカバレッジとテストケースの妥当性
- パフォーマンス: 効率性とスケーラビリティの考慮
- セキュリティ: 潜在的な脆弱性と安全なコーディング実践
- 依存関係: 前提タスクとの適切な統合

出力仕様:
- パス: `/reviews/review-task-{task-number}-{YYYYMMDD-HHMMSS}.md`
- 形式: タスクサマリー、変更サマリー、詳細レビュー、改善提案
- 内容セクション:
  - タスク概要 (Task overview and requirements)
  - 変更サマリー (Change summary)
  - 詳細レビュー (Detailed review by dimension)
  - 改善提案 (Improvement suggestions with priorities)
  - 依存関係チェック (Dependency validation)
- スタイル: 優先度付きの明確なアクション項目
- 言語: すべてのフィードバック内容は日本語

エラー復旧:
- タスクが見つからない: tasks.mdから有効なタスク番号を提案
- 無効なタスク形式: 期待される形式 (X.Y) を表示
- タスク仕様ファイル不在: 適切なプロジェクト構造作成をガイド
- 変更なし: ユーザーに通知してワークフローを提案
- 前提タスク未完了: 依存関係要件を表示
