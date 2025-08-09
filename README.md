# ProjectLens

BacklogのMCP Serverを活用した複数プロジェクトのチケット整理・可視化を行うAI搭載デスクトップアプリケーション

**技術スタック**: Tauri 2.x + Nuxt 3.x + Vue 3 + Pinia + Vuetify + Rust  
**対象プラットフォーム**: macOS, Windows, Linux

> **Note**: Tauri 2.0.6 does not work out-of-the-box with Nuxt, and running Tauri directly through npm scripts may cause issues. This template addresses these compatibility challenges and will be continuously updated for improvements.

## Features

- **Tauri**: Secure, lightweight, and optimized for native desktop applications.
- **Nuxt 3**: Flexible and powerful full-stack framework, enhancing frontend capabilities with Vue 3.
- **Cross-platform Support**: Build for Windows, macOS, and Linux from a single codebase.
- **Easy Setup**: Streamlined template for rapid initialization of Tauri and Nuxt projects.

## 🤖 Claude Code 開発者向けクイックリファレンス

### 📚 重要ドキュメント
- **開発開始前に必読**: `CLAUDE.md` （プロジェクト全体の開発ガイド）
- **要件確認**: `.kiro/specs/multi-project-dashboard/requirements.md`
- **設計指針**: `.kiro/specs/multi-project-dashboard/design-document.md`
- **実装パターン**: `docs/development/implementation-guide.md`

### 🚨 重要な制約・注意点
- **Docker必須**: 全機能がDocker前提、起動チェック必須実装
- **macOS開発環境**: 特別設定必要（DevTools無効、ポーリング使用）
- **エラー処理**: 段階的表示（通知→ダイアログ→ブロッキング）
- **Store間通信**: 循環参照回避のためカスタムイベント使用

### 💡 実装時のクイックTips
```typescript
// Docker依存チェックパターン
if (!dockerStore.isDockerAvailable) {
  dockerStore.handleDockerError('not-installed') // ブロッキングダイアログ表示
}

// Store間通信パターン（循環参照回避）
window.dispatchEvent(new CustomEvent('custom-event', { detail: data }))

// 通知重複防止パターン
isRetryMode.value = true // 再試行中は個別通知を抑制
```

### 🔧 トラブルシューティング
- 開発サーバー起動問題 → `docs/troubleshooting.md#dev-server`
- Docker関連エラー → `_docs/implement-tasks/dev-server-startup-errors-fix.md`

---

## Prerequisites

- **Node.js**: Recommended version [v20.19 or higher](https://nodejs.org/en/).
- **Docker**: Docker Desktop（macOS/Windows）または Docker Engine（Linux）**必須**
- **Rust**: Latest stable version. Install from [rustup.rs](https://rustup.rs/).
- **Tauri CLI**: Install globally via `cargo install tauri-cli`.
- **Yarn**: v1.22以上（推奨）

## Getting Started


Replace all instances of tauri-nuxt-app with your-target-app-name

### 1. Install Dependencies

```bash
yarn install
```

### 2. Docker Setup
**重要**: ProjectLensは全機能がDocker前提で設計されています。

```bash
# Docker Desktopを起動（macOS/Windows）
# または Docker Engineを起動（Linux）

# Docker動作確認
docker --version
docker ps
```

### 3. Run the Application

**Development Mode**

```bash
# 開発サーバー起動
yarn dev

# または Tauri開発モード
yarn tauri:dev
```

**macOS環境でのトラブルシューティング**: 
- `spawn EBADF` エラーが発生した場合: `_docs/implement-tasks/dev-server-startup-errors-fix.md` を参照

**Build for Production**

```bash
yarn tauri:build
```

## Project Structure

```
ProjectLens/
├── CLAUDE.md                  # Claude Code自動参照用プロジェクトガイド
├── .claude/
│   └── commands/              # カスタムコマンド
├── .kiro/                     # 要件・設計書
│   ├── specs/multi-project-dashboard/
│   └── steering/
├── docs/                      # 技術文書
│   ├── development/           # 実装ガイド
│   ├── architecture/          # アーキテクチャ詳細
│   └── troubleshooting.md     # トラブルシューティング
├── _docs/                     # 実装記録
│   ├── implement-tasks/       # タスク実装ログ
│   └── blog/                  # 技術ブログ
├── src/                       # Nuxt app source files
├── src-tauri/                 # Tauri configuration and Rust backend
└── README.md                  # プロジェクト基本情報
```

## Configuration

All Tauri configurations are managed within the src-tauri/tauri.conf.json file. Adjust these settings based on your specific needs, including window size, title, and permission configurations.

## Additional Resources

- **Tauri Documentation**: [tauri.app](https://tauri.app/v2/)

- **Nuxt Documentation**: [nuxt.com](https://nuxt.com/docs/getting-started/introduction)

## License

This template is licensed under the MIT License.

With this template, you’re ready to start building powerful desktop applications leveraging the best of Tauri and Nuxt. Happy coding! 🎉
