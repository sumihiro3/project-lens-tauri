Apply code review feedback from `/reviews` directory with task context awareness.

Arguments: [task-number] (optional) - Filter reviews for specific task (e.g., 2.1)

**Configuration and Environment Variables:**
- `KIRO_SPECS_DIR`: Custom specification directory (default: `.kiro/specs/multi-project-dashboard`)
- `REVIEWS_DIR`: Custom reviews directory (default: `/reviews`)

Steps:
1. Scan `${REVIEWS_DIR}` for feedback files (.md, .txt, .json)
2. Filter by task number if specified, or show all recent reviews
3. Load task context from specifications for validation
4. Parse and categorize feedback (bugs, improvements, style, architecture, etc.)
5. Present implementation plan with estimated scope and task requirement alignment
6. Implement changes incrementally with explanations
7. Validate changes against original task requirements
8. Run tests after each significant change
9. Perform final validation (tests, linting, type checking)
10. Generate descriptive commit message with task references
11. Confirm changes before committing

Additional requirements:
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

**Task Integration Features:**
- Automatically load task specifications to understand context
- Validate review suggestions against task requirements
- Check if applied changes fulfill task acceptance criteria
- Provide feedback on whether task can be marked as complete
- Suggest related tasks that might be affected by changes
- Generate task-aware commit messages (e.g., "Task 2.1: Apply review feedback - Fix error handling")

**Review File Patterns:**
- `review-task-{number}-{timestamp}.md` (task-specific reviews)
- `review-general-{timestamp}.md` (general code reviews)
- Support for both patterns with intelligent task correlation
