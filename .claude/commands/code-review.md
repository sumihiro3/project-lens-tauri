Review local changes based on task specifications and requirements.

Arguments: <task-number> (required) - e.g., 2.1, 3.2, etc.

**Configuration and Environment Variables:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `KIRO_STEERING_DIR`: Custom steering directory (default: `.kiro/steering`)
- `TASKS_FILE`: Custom tasks file name (default: `tasks.md`)

Workflow:
1. Verify prerequisites (git repo, task specifications access)
2. Load task specifications from `${KIRO_SPECS_DIR}/${TASKS_FILE}`
3. Extract specific task ($1) details and requirements
4. Load related context from:
   - `${KIRO_SPECS_DIR}/requirements.md`
   - `${KIRO_SPECS_DIR}/design.md`
   - `${KIRO_STEERING_DIR}/tech.md`
5. Analyze local changes:
   - `git diff main...HEAD --name-status` (changed files)
   - `git diff main...HEAD` (detailed changes)
   - `git log main..HEAD --oneline` (commit history)
6. Contextual review based on task requirements and specifications
7. Generate structured Japanese feedback

Review dimensions:
- Task requirement compliance: Alignment with specified task requirements and acceptance criteria
- Architecture compliance: Adherence to project structure and design patterns from tech.md
- Code quality: Readability, maintainability, consistency with existing codebase
- Bug risks: Error handling, edge cases, potential failure points
- Testing: Coverage and test case validity for task functionality
- Performance: Efficiency and scalability considerations
- Security: Potential vulnerabilities and secure coding practices
- Dependencies: Proper integration with prerequisite tasks

Output specification:
- Path: `/reviews/review-task-{task-number}-{YYYYMMDD-HHMMSS}.md`
- Format: Task summary, change summary, detailed review, improvement suggestions
- Content sections:
  - タスク概要 (Task overview and requirements)
  - 変更サマリー (Change summary)
  - 詳細レビュー (Detailed review by dimension)
  - 改善提案 (Improvement suggestions with priorities)
  - 依存関係チェック (Dependency validation)
- Style: Clear action items with priorities
- Language: Japanese for all feedback content

Error recovery:
- Missing task: Suggest valid task numbers from tasks.md
- Invalid task format: Show expected format (X.Y)
- Task specification files missing: Guide user to create proper project structure
- No changes: Inform user and suggest workflow
- Prerequisite task incomplete: Show dependency requirements
