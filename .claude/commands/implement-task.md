指定されたタスクをガイド付き支援とインタラクティブな意思決定で実装します。

引数: <task-number> (必須) - 例: 2.1, 3.2, など

**重要: すべてのガイダンス、提案、説明、ユーザーとのやり取りは日本語で行います。**

**設定・環境変数:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `KIRO_STEERING_DIR`: Custom steering directory (default: `.kiro/steering`)
- `TASKS_FILE`: Custom tasks file name (default: `tasks.md`)
- `REQUIREMENTS_FILE`: Custom requirements file name (default: `requirements.md`)
- `DESIGN_FILE`: Custom design file name (default: `design.md`)

フェーズ1: 実装計画の確認とタスク分析

**0. 実装計画の確認 (必須の最初のステップ)**
- **Check for existing implementation plan**: Look for `_docs/task-plans/$1-task-plan.md`
- **If plan does not exist**:
  ```
  ⚠️  実装計画が見つかりません (Task $1)
  
  このタスクを実装する前に、実装計画の作成をお勧めします。
  実装計画には以下が含まれます：
  - 詳細な要件分析
  - 技術設計とアプローチ
  - 実装ステップ
  - テスト計画
  - 品質基準
  
  /plan-implement-task $1 を実行して実装計画を作成しますか？ [Y/n/skip]:
  ```
- **If user chooses to create plan**: Execute `/plan-implement-task $1` command
- **If user skips**: Proceed with warning that implementation may be less structured
- **If plan exists**: Load and reference the plan throughout implementation

1. **計画統合によるタスク理解**
- **Verify specification files exist**: Check if specification directories and required files are present using environment variables
- **Error handling**: If specification files are missing, display clear error messages in Japanese and provide guidance
- **Load task details**: Use `${KIRO_SPECS_DIR}/${TASKS_FILE}` (default: `.kiro/specs/multi-project-dashboard/tasks.md`)
- Extract specific task ($1) content and requirements with validation
- **Load implementation plan**: If exists, load `_docs/task-plans/$1-task-plan.md` for guided implementation
- **Load requirements**: Use `${KIRO_SPECS_DIR}/${REQUIREMENTS_FILE}` (default: `.kiro/specs/multi-project-dashboard/requirements.md`)
- **Load design guidance**: Use `${KIRO_SPECS_DIR}/${DESIGN_FILE}` (default: `.kiro/specs/multi-project-dashboard/design.md`)
- **Load project context**: Use `${KIRO_STEERING_DIR}` (default: `.kiro/steering`) for:
  - `product.md`
  - `structure.md`
  - `tech.md`
- **Fallback behavior**: If any specification file is missing, continue with available information and warn user
- Parse and categorize requirements into actionable items (explain in Japanese)
- **Cross-reference with implementation plan** if available
- Assess complexity level and estimated implementation scope (present in Japanese)
- Check for existing implementation in the codebase (report findings in Japanese)

1.5. **タスク依存関係の検証**
- **Check prerequisite tasks**: Parse tasks.md to identify if prerequisite tasks are marked as completed
- **Dependency warnings**: If attempting to implement out of order, display warnings in Japanese
- **Override option**: Allow user to proceed with confirmation for development flexibility
- **Display task hierarchy**: Show related tasks and logical implementation sequence

2. **環境準備**
- Verify clean working directory: `git status --porcelain`
- Confirm current branch: `git branch --show-current`
- Switch to develop branch: `git checkout develop && git pull origin develop`
- Create feature branch from develop: `git checkout -b task-$1`
- Handle existing branch conflicts with user confirmation

3. **コードベース分析**
- Search relevant files using task keywords and context from loaded specifications
- Identify existing patterns and architectural conventions
- Map potential files requiring modification based on task requirements (present analysis in Japanese)
- Present initial implementation strategy for user approval (explain in Japanese)

フェーズ2: インタラクティブ実装

4. **実装アプローチの提示 (計画がある場合はそれに基づく)**
**If implementation plan exists**:
```
🔍 タスク分析完了 (Task $1)
📋 実装計画書から読み込み済み: _docs/task-plans/$1-task-plan.md

実装ステップ:
[実装計画から抽出されたステップ]
1. [ファイル/コンポーネント1] - [目的]
2. [ファイル/コンポーネント2] - [目的] 
3. [テスト] - [カバー範囲]

技術設計:
[実装計画の技術設計]

品質基準:
[実装計画の品質基準]

この実装計画に従って進めますか？ [Y/n/modify]:
```

**If no implementation plan**:
```
🔍 タスク分析完了 (Task $1)
⚠️  実装計画なしで進行中

📋 基本実装アプローチ:
1. [ファイル/コンポーネント1] - [目的]
2. [ファイル/コンポーネント2] - [目的] 
3. [テスト] - [カバー範囲]

注意: より詳細な計画が必要な場合は、/plan-implement-task $1 の実行を検討してください。

この基本計画で進めますか？ [Y/n/create-plan]:
```

5. **ファイル毎のガイド実装 (実装計画に従って)**
For each implementation step (communicate in Japanese):
- **Follow plan structure**: Use implementation plan's step-by-step guidance when available
- **Present Context**: Show current state and proposed changes in Japanese
- **Reference plan rationale**: Explain technical approach from implementation plan (in Japanese)
- **Offer Options**: Templates, patterns, or custom implementation (explain in Japanese)
- **Seek Confirmation**: Before making significant changes (ask in Japanese)
- **Validate against plan**: Ensure implementation aligns with planned approach (communicate in Japanese)

```
📁 次: src/components/LoginForm.tsx
現在: ファイルが存在しません
提案: フォームバリデーション付きReactコンポーネント

実装オプション:
1. 既存フォームテンプレートを使用して修正
2. プロジェクトパターンに従ってゼロから作成  
3. 手動で実装

選択してください [1/2/3]:
```

6. **継続的な検証とフィードバック**
- **Incremental Testing**: Run relevant tests after each file change (report results in Japanese)
- **Immediate Feedback**: Show test results and linting issues (in Japanese)
- **Progressive Fixes**: Address issues before moving to next step (guide in Japanese)
- **Human-Reviewable Commits**: Create logical, reviewable commits at natural breakpoints (explain in Japanese)
  - **Atomic Changes**: Each commit should represent a single, complete logical change
  - **Clear Purpose**: Every commit should have a clear, single responsibility
  - **Reviewable Size**: Aim for commits that can be reviewed in 5-15 minutes
  - **Logical Grouping**: Group related changes together (e.g., interface + implementation, test + code)
  - **Documentation**: Include context and reasoning in commit messages

```
🧪 LoginForm.tsxをテスト中...
✅ コンポーネントが正常にレンダリングされました
⚠️  PropTypesが不足しています（推奨ですが必須ではありません）
❌ ログインバリデーションテストが失敗しています

💾 コミット推奨ポイント: 基本コンポーネント構造が完成
   例: "Task 2.1: Add basic LoginForm component structure"

アクション:
[コミット/テスト修正/PropTypes追加/続行]:
```

**Commit Strategy Guidelines:**
- **Foundation commits**: Type definitions, interfaces, basic structure
  - Example: "Task 2.1: Add Docker service interface and types"
- **Feature commits**: Single functionality implementation (one method or function)
  - Example: "Task 2.1: Implement Docker version check functionality"  
- **Test commits**: Add tests for corresponding functionality
  - Example: "Task 2.1: Add unit tests for Docker service"
- **Integration commits**: Feature integration and wiring
  - Example: "Task 2.1: Integrate Docker service with main application"
- **Fix commits**: Bug fixes, refactoring
  - Example: "Task 2.1: Fix error handling in Docker service"

**Commit Quality Checklist (present in Japanese):**
- Changes have clear, single purpose
- Reviewable within 15 minutes
- Related changes are grouped together
- Commit message explains why, not just what

フェーズ3: 品質保証と最終処理

7. **包括的検証**
- Execute full test suite with detailed reporting (present results in Japanese)
- Perform complete linting and type checking (communicate status in Japanese)
- Validate implementation against original issue requirements (explain in Japanese)
- Check for potential breaking changes or regressions (report in Japanese)

8. **コミット準備と最終処理**
- Generate clear commit message following conventional format (present in Japanese)
- Include appropriate task references (e.g., "Task $1: [description]") (explain in Japanese)
- Reference related requirements from specification files (explain in Japanese)
- Offer commit message customization before finalizing (interact in Japanese)

9. **進捗追跡とタスク完了**
- **Automatically update task status**: Mark task as completed [x] in tasks.md file
- **Generate progress summary**: Show completed vs remaining tasks with percentages
- **Suggest next tasks**: Analyze dependencies and recommend next logical implementation steps
- **Update task completion timestamp**: Add completion date for tracking purposes
- Provide next steps guidance (PR creation, review assignment) (explain in Japanese)

```
🎯 実装サマリー:
- 3ファイル作成、2ファイル修正
- 8テスト追加、すべて通過
- リント問題 0件
- タスク要件: ✅ すべて対応済み

コミットメッセージ:
"feat(docker): Docker可用性チェック機能を実装

- Dockerコマンド実行可能性の検証機能を追加
- Dockerバージョン取得機能を実装
- MCP Serverコンテナ状態確認機能を作成

Task 2.1: Docker可用性チェックサービスの作成 実施"

コミットしますか？ [Y/edit/review]:
```

主な機能:

**Interactive Decision Making (in Japanese):**
- Always ask for confirmation before significant changes (in Japanese)
- Provide multiple implementation options with rationale (explain in Japanese)
- Allow switching approaches mid-process (guide in Japanese)
- Support pause/resume functionality (communicate in Japanese)

**Safety Measures (communicate in Japanese):**
- Validation gates preventing progression with failing tests (explain in Japanese)
- Rollback options at multiple checkpoints (present options in Japanese)
- Conflict resolution guidance (provide instructions in Japanese)
- Partial implementation support with TODO markers (explain in Japanese)

**Quality Assurance (report in Japanese):**
- Incremental testing after each change (show results in Japanese)
- Continuous linting and type checking (communicate status in Japanese)
- Requirement validation against original issue (explain findings in Japanese)
- Performance impact assessment (present analysis in Japanese)

**Learning Opportunities (provide in Japanese):**
- Explain code patterns and architectural decisions (in Japanese)
- Show why certain approaches are recommended (explain reasoning in Japanese)
- Provide detailed explanations when requested (in Japanese)
- Maintain consistency with existing codebase (explain patterns in Japanese)

Example Interaction Flow (in Japanese):
```
🔧 環境変数確認: KIRO_SPECS_DIR=/custom/specs, KIRO_STEERING_DIR=/custom/steering
🔍 Task 2.1を分析中: "Docker可用性チェックサービスの作成"
⚠️  依存関係チェック: Task 1.3 (必要な依存関係の追加) が完了済み ✅
📋 要件を発見: Docker コマンド実行検証、バージョン取得、コンテナ状態確認
📚 参照情報: requirements.md, design.md, tech.md から関連情報を取得
🎯 実装計画準備完了 - 進めますか？ [Y/n/modify]

📁 docker_service.rsを作成中...
💡 オプション: [既存パターン/カスタム/テンプレート]
🧪 テスト実行中... ✅ すべて通過
📝 Task 2.1完了 → tasks.mdを更新中...
📊 進捗: 8/90タスク完了 (8.9%)
🎯 次の推奨タスク: Task 2.2 (Docker エラーハンドリングUIの実装)
```

**Important Implementation Notes:**
- **Always check for implementation plan first**: `_docs/task-plans/$1-task-plan.md`
- **Reference plan throughout implementation**: Use plan's technical design and steps
- **Offer plan creation**: If no plan exists, suggest `/plan-implement-task $1`
- Always load specification files using configurable paths with environment variable fallbacks
- Validate task dependencies before starting implementation
- Automatically update task completion status in tasks.md file
- Provide comprehensive error handling with Japanese user feedback
- Support multiple project structures through configuration
- **Maintain consistency between plan and implementation**: Ensure actual implementation follows planned approach
