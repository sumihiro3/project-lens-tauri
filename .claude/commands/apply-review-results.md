タスク文脈認識機能付きで `/reviews` ディレクトリからコードレビューフィードバックを適用します。

引数: [task-number] (オプション) - 特定タスクのレビューをフィルタ (例: 2.1)

**設定・環境変数:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `REVIEWS_DIR`: Custom reviews directory (default: `/reviews`)

ステップ:
1. `${REVIEWS_DIR}` でフィードバックファイル (.md, .txt, .json) をスキャン
2. 指定されている場合はタスク番号でフィルタ、または最近のレビューをすべて表示
3. 検証のため仕様書からタスク文脈を読み込み
4. フィードバックを解析・分類 (バグ、改善、スタイル、アーキテクチャなど)
5. 推定スコープとタスク要件整合性を含む実装計画を提示
6. 説明付きで段階的に変更を実装
7. 元のタスク要件に対して変更を検証
8. 重要な変更の度にテストを実行
9. 最終検証を実行 (テスト、リンティング、型チェック)
10. タスク参照付きの説明的コミットメッセージを生成
11. コミット前に変更を確認

追加要件:
- Handle missing `${REVIEWS_DIR}` directory gracefully
- Support multiple feedback file formats (.md, .txt, .json)
- Recognize task-specific review files (review-task-X.Y-*.md)
- Prioritize critical issues first based on task requirements
- Validate that changes don't break task dependencies
- Cross-reference with task specifications to ensure alignment
- Provide clear progress updates with task context
- Ask for confirmation on ambiguous feedback
- Update task completion status if all review items are addressed
- Generate summary of applied changes for task documentation

**タスク統合機能:**
- Automatically load task specifications to understand context
- Validate review suggestions against task requirements
- Check if applied changes fulfill task acceptance criteria
- Provide feedback on whether task can be marked as complete
- Suggest related tasks that might be affected by changes
- Generate task-aware commit messages (e.g., "Task 2.1: Apply review feedback - Fix error handling")

**レビューファイルパターン:**
- `review-task-{number}-{timestamp}.md` (task-specific reviews)
- `review-general-{timestamp}.md` (general code reviews)
- Support for both patterns with intelligent task correlation
