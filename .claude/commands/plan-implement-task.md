Create detailed implementation plan for specific task from tasks.md.

Usage: Provide task number (e.g., 3.1, 5.2, 12.1)

Process:
1. **Task analysis**
   - Read tasks.md file from `.kiro/specs/multi-project-dashboard/tasks.md`
   - Extract specific task content by task number
   - Parse task description, requirements references, and subtasks
   - Extract requirement references (e.g., "_要件: 7.1, 7.2, 7.3_")
   - Cross-reference with requirements.md to get detailed acceptance criteria
   - Analyze task hierarchy and identify parent/child relationships
   - Identify prerequisite tasks (must be completed before this task)
   - Identify blocking tasks (tasks that depend on this task)
   - Detect parallel execution opportunities

2. **Codebase reconnaissance**
   - Intelligent file search based on task keywords and requirements
   - Pattern: `git grep -rn "relevant-terms" --include="*.{js,ts,py,java,rs,vue}"`
   - Analyze existing similar implementations
   - Identify architectural patterns and conventions
   - Map potential integration points

3. **Project context analysis**
   - Read project specification files for context:
     - `.kiro/specs/multi-project-dashboard/design.md` - Architecture and system design
     - `.kiro/steering/product.md` - Product concept and value proposition
     - `.kiro/steering/structure.md` - Directory structure and architectural patterns
     - `.kiro/steering/tech.md` - Technical stack and system configuration
     - `.kiro/specs/multi-project-dashboard/requirements.md` - Detailed requirements and acceptance criteria
   - Extract relevant architectural patterns and conventions
   - Identify technology stack constraints and best practices

4. **Comprehensive planning**
   - Decompose task into granular, actionable steps
   - Define technical approach with rationale
   - Identify required changes by file/module
   - Plan testing strategy (unit, integration, e2e)
   - Consider performance and security implications
   - Estimate effort and identify blockers

5. **Generate structured plan file**
   - Save to `_docs/task-plans/{task-number}-task-plan.md`
   - Comprehensive Japanese documentation
   - Reference original task number and description
   - Include all technical details and implementation steps

6. **Plan validation and updates**
   - Validate plan against task requirements
   - Check for potential conflicts with existing tasks
   - Suggest improvements or alternative approaches if needed

7. **Task completion confirmation and logging**
   - Check with developer if the task is truly completed
   - Verify completion criteria have been met (Definition of Done)
   - Ask for explicit confirmation before creating implementation log
   - Upon confirmation, create implementation log file
   - Save to `_docs/implement-tasks/{task-number}-task-logs.md`
   - **Write all log content in Japanese**: Ensure comprehensive Japanese documentation
   - Document actual implementation steps, challenges faced, and solutions (in Japanese)
   - Include technical insights suitable for blog content generation (in Japanese)
   - Record lessons learned and best practices discovered (in Japanese)
   - **Language consistency**: All sections of the implementation log must be written in Japanese

Implementation plan template:
## 📋 実装計画概要
**対象タスク**: {task-number} - {task-title}
**計画作成日**: {date}
**推定工数**: {estimate}
**要件参照**: {requirement-refs}

## 🎯 タスク概要
{task-description}

## 📖 要件詳細
{detailed-requirements-from-requirements.md}

### 受け入れ基準
{acceptance-criteria-from-requirements}

## 🏗️ 技術設計
### アプローチ
{technical approach}

### 影響範囲
{affected files and components}

### 依存関係
**前提タスク（完了が必要）:**
{prerequisite-tasks}

**並行実行可能なタスク:**
{parallel-tasks}

**このタスクをブロックしているタスク:**
{blocking-tasks}

## ✅ 実装ステップ
- [ ] {specific step 1}
- [ ] {specific step 2}
- [ ] テスト実装
- [ ] ドキュメント更新

## 🧪 テスト計画
### テスト戦略
{testing strategy}

### テスト種別
- **ユニットテスト**: {unit-test-targets}
- **統合テスト**: {integration-test-targets}
- **E2Eテスト**: {e2e-test-scenarios}

### テスト網羅度
- **機能カバレッジ**: 最低80%を目標
- **エラーパスのテスト**: 例外処理とエラーハンドリングの検証
- **パフォーマンステスト**: レスポンス時間とメモリ使用量の検証

## ⚠️ リスクと考慮事項
{risks and considerations}

## 📚 参考資料
{relevant documentation or similar implementations}

## 🏗️ プロジェクト技術情報
**技術スタック**: Tauri + Vue.js (Nuxt3) + TypeScript + Pug, Rust バックエンド
**AI統合**: Mastra (OpenAI/Claude/Gemini 対応)
**状態管理**: Pinia
**データ保存**: SQLite (ローカルのみ)
**外部連携**: Backlog MCP Server (Docker経由)

## 📋 アーキテクチャ参照
- **レイヤー構成**: プレゼンテーション層(Vue/Nuxt3) → アプリケーション層(Rust) → データ層(SQLite)
- **セキュリティ**: APIキー暗号化保存、ローカルのみでのデータ処理
- **パフォーマンス**: 軽量起動、低メモリ使用量重視

## 🔗 関連タスク
### 依存関係マップ
```
{dependency-map-visualization}
```

### 関連タスク詳細
{related-tasks-from-tasks.md}

## ✅ 完成条件 (Definition of Done)
- [ ] 全機能が要件を満たして動作する
- [ ] 全テストケースが通過する（最低80%カバレッジ）
- [ ] エラーハンドリングが適切に実装されている
- [ ] パフォーマンス基準を満たしている
- [ ] セキュリティ要件を満たしている
- [ ] ドキュメント（コメント・README）が更新されている
- [ ] コードレビューが完了している
- [ ] 統合テストで既存機能に問題が発生しない

## 📝 レビューチェックリスト
### コード品質
- [ ] コーディング規約に準拠している
- [ ] 適切なエラーハンドリングが実装されている
- [ ] パフォーマンスへの配慮がされている
- [ ] セキュリティベストプラクティスが適用されている

### アーキテクチャ準拠
- [ ] 既存のアーキテクチャパターンに従っている
- [ ] レイヤー分離が適切に実装されている
- [ ] 依存関係の注入が適切に行われている
- [ ] 単一責任の原則が守られている

### テスト
- [ ] テストケースが網羅的である
- [ ] エッジケースがテストされている
- [ ] モックが適切に使用されている
- [ ] テストの可読性が高い

## 📝 タスク完了ログテンプレート（日本語で作成）
Implementation log template for `_docs/implement-tasks/{task-number}-task-logs.md` (write entirely in Japanese):

```markdown
# 【実装ログ】{task-number}: {task-title}

## 📋 基本情報
**実装日**: {completion-date}
**実装者**: {implementer}
**推定工数**: {estimated-effort}
**実作業時間**: {actual-time-spent}
**関連要件**: {requirement-refs}

## 🎯 実装概要
{brief-summary-of-what-was-implemented}

## 🔧 実装詳細
### 主な変更点
{key-changes-made}

### 追加されたファイル
{new-files-created}

### 変更されたファイル  
{modified-files}

### 技術的なアプローチ
{technical-approach-used}

## 💡 技術的発見・学習内容
### 新しく学んだこと
{new-technical-knowledge}

### 既存知識の応用
{applied-existing-knowledge}

### アーキテクチャへの洞察
{architectural-insights}

## 🚧 遭遇した課題と解決策
### 課題 1: {challenge-title}
**問題**: {problem-description}
**解決策**: {solution-implemented}
**学習**: {lessons-learned}

### 課題 2: {challenge-title-2}
**問題**: {problem-description-2}
**解決策**: {solution-implemented-2}  
**学習**: {lessons-learned-2}

## 🧪 テスト・検証
### 実施したテスト
{tests-performed}

### 発見された不具合
{bugs-found-and-fixed}

### パフォーマンス検証
{performance-validation}

## 📈 品質・パフォーマンス向上点
{quality-and-performance-improvements}

## 🔄 リファクタリング・最適化
{refactoring-and-optimizations}

## 🌟 ベストプラクティス・パターン発見
{best-practices-and-patterns}

## 💭 振り返り・今後への示唆
### うまくいったこと
{what-went-well}

### 改善できること  
{areas-for-improvement}

### 次回への学び
{lessons-for-future}

## 🎨 ブログネタ候補
### 技術記事のアイデア
- {blog-idea-1}
- {blog-idea-2}
- {blog-idea-3}

### 共有価値のある発見
{valuable-insights-to-share}

### 他の開発者に役立つTips
{helpful-tips-for-other-developers}

## 🔗 関連リソース
{related-documentation-links}
{useful-external-resources}
```

## 🎯 タスク完了確認プロセス

### 完了確認チェックリスト
開発者に以下を確認してからログ作成を行う：

1. **機能実装の確認**
   - [ ] 計画された機能がすべて実装されているか？
   - [ ] 要件で定義された受け入れ基準を満たしているか？

2. **品質基準の確認**
   - [ ] 全テストケースが通過しているか？
   - [ ] コードレビューが完了しているか？
   - [ ] エラーハンドリングが適切に実装されているか？

3. **統合テストの確認**
   - [ ] 既存機能に問題が発生していないか？
   - [ ] パフォーマンス基準を満たしているか？

4. **ドキュメント更新の確認**
   - [ ] 必要なドキュメントが更新されているか？
   - [ ] コメント・READMEが適切に記述されているか？

### 確認プロンプト例
```
このタスク ({task-number}) は完了しましたか？

以下のDefinition of Doneをすべて満たしていることを確認してください：
- ✅ 全機能が要件を満たして動作する
- ✅ 全テストケースが通過する（最低80%カバレッジ）
- ✅ エラーハンドリングが適切に実装されている
- ✅ パフォーマンス基準を満たしている
- ✅ セキュリティ要件を満たしている
- ✅ ドキュメント（コメント・README）が更新されている
- ✅ コードレビューが完了している
- ✅ 統合テストで既存機能に問題が発生しない

タスクが完了している場合、実装ログを作成しますか？ (yes/no)
```

Safety measures:
- Validate task number exists in tasks.md
- Check task dependencies are satisfied
- Ensure plan aligns with overall project architecture
- Support incremental implementation approach
- **Require explicit developer confirmation before creating implementation log**
- Validate completion criteria before marking task as done
- Require code review approval for quality assurance
- **Create comprehensive implementation log entirely in Japanese for knowledge sharing**
- **Document insights suitable for technical blog content in Japanese**
- **Do not create logs for incomplete or partially implemented tasks**
- **Ensure language consistency throughout the implementation log**
