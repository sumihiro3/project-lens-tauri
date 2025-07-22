自動タスク完了追跡機能付きでプルリクエストを作成します。

使用方法: カンマ区切りのタスク番号を指定 (例: "2.1,2.2" または単一タスク "3.1")

**設定・環境変数:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `KIRO_STEERING_DIR`: Custom steering directory (default: `.kiro/steering`)
- `TASKS_FILE`: Custom tasks file name (default: `tasks.md`)

事前チェック:
1. 現在のブランチがdevelop/main/masterでないことを確認
2. ローカル変更の存在確認: `git rev-list develop..HEAD --count`
3. GitHub CLI認証の検証: `gh auth status`
4. タスク番号形式の解析・検証 (例: 2.1, 3.2)
5. `${KIRO_SPECS_DIR}/${TASKS_FILE}` からタスク仕様を読み込み
6. 指定されたタスクがタスクファイルに存在することを確認

ブランチ準備:
1. 変更サマリーを表示: `git log develop..HEAD --oneline` と `git diff develop --stat`
2. クリーンなワーキングディレクトリを確保 (すべての変更がコミット済み)
3. フィーチャーブランチをプッシュ: `git push -u origin HEAD`

PR作成プロセス:
1. 読み込まれたタスク仕様からタスク詳細を抽出
2. 関連文脈の読み込み:
   - `${KIRO_SPECS_DIR}/requirements.md`
   - `${KIRO_SPECS_DIR}/design.md` 
   - `${KIRO_STEERING_DIR}/tech.md` (for technical context)
3. インテリジェントなPRタイトルの生成:
   - 単一タスク: タスク番号プレフィックス付きでタスク説明を使用
   - 複数タスク: 共通機能に基づくサマリーを作成
   - 形式: "Task X.Y: [説明]" または "Tasks X.Y,X.Z: [サマリー]"
4. 構造化された日本語説明の作成:
   - 実装タスク (list completed task numbers and descriptions)
   - 変更の概要 (based on commits and task requirements)
   - 主な変更点 (from git diff summary)
   - テスト方法 (prompt if not obvious)
   - 完了タスク: Task X.Y, Task X.Z (instead of closing issues)

PR作成実行:
- Run: `gh pr create --base develop --title "<generated-title>" --body "<japanese-description>"`
- Handle draft PR option for large changes
- Auto-assign reviewers based on CODEOWNERS if available

作成後処理:
- PR URLと番号を表示
- CI/チェック状況を表示
- `${KIRO_SPECS_DIR}/${TASKS_FILE}` でタスク完了状況を更新 (タスクを [x] でマーク)
- 完了済み対残りタスクの進捗サマリーを生成
- 依存関係に基づく次の論理的タスクを提案
- 次のアクションを提案 (レビュー依頼など)

エラーシナリオ:
- No commits ahead of main: Explain and suggest workflow
- Task numbers not found: List available pending tasks from `${KIRO_SPECS_DIR}/${TASKS_FILE}`
- Invalid task number format: Show expected format (X.Y)
- Task specification files missing: Guide user to create proper project structure
- Permission denied: Authentication troubleshooting
- Merge conflicts: Resolution guidance
- Rate limiting: Retry suggestions

品質保証:
- Verify all task numbers are correctly formatted in description
- Check PR title length (GitHub limits)
- Ensure description is meaningful and in Japanese
- Confirm task completion references are accurate
- Validate against task requirements from specification files
- Confirm branch protection rules are satisfied
- Ensure task dependencies are properly handled
