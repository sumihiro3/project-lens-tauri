Create pull request with automatic task completion tracking.

Usage: Provide comma-separated task numbers (e.g., "2.1,2.2" or single task "3.1")

**Configuration and Environment Variables:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `KIRO_STEERING_DIR`: Custom steering directory (default: `.kiro/steering`)
- `TASKS_FILE`: Custom tasks file name (default: `tasks.md`)

Pre-flight checks:
1. Verify current branch is not main/master
2. Confirm local changes exist: `git rev-list main..HEAD --count`
3. Validate GitHub CLI authentication: `gh auth status`
4. Parse and verify task numbers format (e.g., 2.1, 3.2)
5. Load task specifications from `${KIRO_SPECS_DIR}/${TASKS_FILE}`
6. Verify specified tasks exist in the task file

Branch preparation:
1. Show change summary: `git log main..HEAD --oneline` and `git diff main --stat`
2. Ensure clean working directory (all changes committed)
3. Push feature branch: `git push -u origin HEAD`

PR creation process:
1. Extract task details from loaded task specifications
2. Load related context from:
   - `${KIRO_SPECS_DIR}/requirements.md`
   - `${KIRO_SPECS_DIR}/design.md` 
   - `${KIRO_STEERING_DIR}/tech.md` (for technical context)
3. Generate intelligent PR title:
   - Single task: Use task description with Task number prefix
   - Multiple tasks: Create summary based on common functionality
   - Format: "Task X.Y: [Description]" or "Tasks X.Y,X.Z: [Summary]"
4. Create structured Japanese description:
   - 実装タスク (list completed task numbers and descriptions)
   - 変更の概要 (based on commits and task requirements)
   - 主な変更点 (from git diff summary)
   - テスト方法 (prompt if not obvious)
   - 完了タスク: Task X.Y, Task X.Z (instead of closing issues)

Execute PR creation:
- Run: `gh pr create --title "<generated-title>" --body "<japanese-description>"`
- Handle draft PR option for large changes
- Auto-assign reviewers based on CODEOWNERS if available

Post-creation:
- Display PR URL and number
- Show CI/check status
- Update task completion status in `${KIRO_SPECS_DIR}/${TASKS_FILE}` (mark tasks as [x])
- Generate progress summary with completed vs remaining tasks
- Suggest next logical tasks based on dependencies
- Suggest next actions (request reviews, etc.)

Error scenarios:
- No commits ahead of main: Explain and suggest workflow
- Task numbers not found: List available pending tasks from `${KIRO_SPECS_DIR}/${TASKS_FILE}`
- Invalid task number format: Show expected format (X.Y)
- Task specification files missing: Guide user to create proper project structure
- Permission denied: Authentication troubleshooting
- Merge conflicts: Resolution guidance
- Rate limiting: Retry suggestions

Quality assurance:
- Verify all task numbers are correctly formatted in description
- Check PR title length (GitHub limits)
- Ensure description is meaningful and in Japanese
- Confirm task completion references are accurate
- Validate against task requirements from specification files
- Confirm branch protection rules are satisfied
- Ensure task dependencies are properly handled
