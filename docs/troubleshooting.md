# トラブルシューティングガイド

## よくある問題と解決方法

### 開発環境の問題

#### 1. macOS開発サーバー起動エラー

**症状**: `spawn EBADF` エラーが発生し、開発サーバーが起動しない

```bash
ERROR [unhandledRejection] spawn EBADF
    at ChildProcess.spawn (node:internal/child_process.js:413:11)
    at Object.spawn (node:child_process.js:698:9)
```

**原因**: macOS環境でのNuxt DevToolsとファイル監視の競合

**解決方法**:
1. `nuxt.config.ts` を以下のように設定
```typescript
export default defineNuxtConfig({
  devtools: { enabled: false },        // DevTools無効化
  vite: {
    server: {
      watch: {
        usePolling: true,              // ポーリングベース監視
        interval: 1000
      },
      hmr: {
        overlay: false                 // HMRオーバーレイ無効化
      }
    }
  }
})
```

2. 開発サーバーを再起動
```bash
yarn dev
```

**参考**: `_docs/implement-tasks/dev-server-startup-errors-fix.md`

#### 2. Docker関連エラー

**症状**: 「Dockerが見つかりません」エラーが表示される

**チェック項目**:
1. Docker Desktopが起動しているか確認
```bash
docker --version
docker ps
```

2. Docker Desktopの設定確認
   - Docker Desktop > Settings > General
   - "Use the WSL 2 based engine" (Windows)
   - "Use Docker Compose V2" を有効化

**解決手順**:
1. Docker Desktopを完全に再起動
2. アプリケーションを再起動
3. 「再試行」ボタンをクリック

#### 3. 依存関係インストールエラー

**症状**: `yarn install` 実行時にエラーが発生

**解決方法**:
```bash
# キャッシュクリア
yarn cache clean

# node_modules削除
rm -rf node_modules
rm yarn.lock

# 再インストール
yarn install
```

**Rust依存関係の問題**:
```bash
# Rust toolchain更新
rustup update

# Tauri CLI再インストール
cargo install tauri-cli --force
```

### 実行時の問題

#### 1. 通知重複エラー

**症状**: 同じ通知が複数回表示される

**原因**: Store間の循環参照または再試行時の重複制御不備

**確認ポイント**:
1. `dockerStore.isRetryMode` フラグが正しく管理されているか
2. カスタムイベントのリスナーが重複登録されていないか

**修正例**:
```typescript
// dockerStore.ts
async retryDockerEnvironment(): Promise<boolean> {
  this.isRetryMode = true  // 重複防止フラグ設定
  
  try {
    await this.initializeDockerEnvironment()
    return this.isDockerAvailable && this.isDockerRunning
  } finally {
    this.isRetryMode = false  // 必ずクリア
  }
}
```

#### 2. AI分析が実行されない

**症状**: チケット取得後にAI分析結果が表示されない

**チェック項目**:
1. APIキーが正しく設定されているか
2. AI Provider が選択されているか
3. ネットワーク接続に問題がないか

**デバッグ手順**:
```bash
# 開発者ツールでコンソールエラーを確認
# ネットワークタブでAPI呼び出しを確認
```

**解決方法**:
1. 設定画面でAPIキーを再入力
2. AI Providerを変更して試行
3. ネットワーク設定の確認

#### 3. データ同期エラー

**症状**: Backlogからのデータ取得が失敗する

**エラーメッセージ例**:
```
MCP Server接続エラー: connect ECONNREFUSED 127.0.0.1:3001
```

**解決手順**:
1. MCP Serverコンテナの状態確認
```bash
docker ps | grep backlog-mcp
docker logs backlog-mcp-server
```

2. コンテナ再起動
```bash
docker restart backlog-mcp-server
```

3. ポート競合の確認
```bash
lsof -i :3001
```

### ビルド・デプロイの問題

#### 1. Tauriビルドエラー

**症状**: `yarn tauri:build` でビルドが失敗する

**よくあるエラー**:
```
error: failed to run custom build command for `tauri v2.x.x`
```

**解決方法**:
1. Rust toolchainの確認
```bash
rustc --version
cargo --version
```

2. 必要なシステム依存関係のインストール

**Windows**:
```bash
# Microsoft C++ Build Tools
# Windows SDK
```

**macOS**:
```bash
xcode-select --install
```

**Linux**:
```bash
sudo apt-get update
sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.0-dev libayatana-appindicator3-dev librsvg2-dev
```

#### 2. 署名・公証エラー (macOS)

**症状**: macOSでアプリケーションが「開発元が未確認」として拒否される

**開発時の対処**:
```bash
# 開発用署名無効化
sudo spctl --master-disable
```

**本番用の対処**:
1. Apple Developer Program登録
2. 証明書の設定
3. `tauri.conf.json` での署名設定

### パフォーマンスの問題

#### 1. 起動時間が遅い

**症状**: アプリケーション起動に5秒以上かかる

**原因分析**:
1. Docker起動チェックの遅延
2. 大量キャッシュデータの読み込み
3. 複数ワークスペースの同期処理

**最適化方法**:
1. Docker起動チェックのタイムアウト設定
```typescript
const DOCKER_CHECK_TIMEOUT = 3000  // 3秒
```

2. キャッシュサイズの制限
```typescript
const MAX_CACHE_SIZE = 1000  // 最大1000件
```

3. 非同期読み込みの活用
```typescript
// 起動時は必要最小限のデータのみ取得
// 追加データは遅延読み込み
```

#### 2. メモリ使用量が多い

**症状**: アプリケーションのメモリ使用量が500MB以上

**対策**:
1. キャッシュの定期クリア
```typescript
setInterval(() => {
  cacheStore.cleanup()
}, 300000)  // 5分毎
```

2. 不要なデータの削除
```typescript
// 古いチケットデータの削除
const cutoffDate = new Date()
cutoffDate.setDate(cutoffDate.getDate() - 30)  // 30日前
```

### セキュリティの問題

#### 1. 認証情報が保存されない

**症状**: APIキーを入力しても次回起動時に消える

**チェック項目**:
1. 暗号化保存の権限
2. ファイルシステムの書き込み権限
3. ウイルス対策ソフトの干渉

**解決方法**:
1. アプリケーションを管理者権限で実行（一時的）
2. ウイルス対策ソフトの例外設定に追加
3. 手動での認証情報ファイル作成確認

#### 2. 認証情報の復号化失敗

**症状**: 保存された認証情報が読み込めない

**エラーメッセージ**:
```
Decryption failed: Invalid key or corrupted data
```

**対処方法**:
1. 認証情報ファイルの削除（再入力が必要）
```bash
# macOS
rm ~/Library/Application\ Support/ProjectLens/credentials.enc

# Windows  
del %APPDATA%\ProjectLens\credentials.enc

# Linux
rm ~/.config/ProjectLens/credentials.enc
```

2. アプリケーション再起動後に再設定

### ログとデバッグ

#### 1. ログファイルの場所

**macOS**: `~/Library/Logs/ProjectLens/`
**Windows**: `%LOCALAPPDATA%\ProjectLens\logs\`
**Linux**: `~/.local/share/ProjectLens/logs/`

#### 2. デバッグモードの有効化

**開発環境**:
```bash
RUST_LOG=debug yarn tauri:dev
```

**本番環境**:
```json
// tauri.conf.json
{
  "tauri": {
    "bundle": {
      "resources": ["logs/*"]
    }
  }
}
```

#### 3. よく使うデバッグ情報

```bash
# システム情報
uname -a
node --version
yarn --version
rustc --version
docker --version

# アプリケーション情報
cat ~/.config/ProjectLens/config.json
ls -la ~/.config/ProjectLens/
```

### サポートリクエストの準備

問題が解決しない場合、以下の情報を整理してサポートに連絡してください：

1. **環境情報**
   - OS版本
   - Node.js版本
   - Docker版本
   - ProjectLens版本

2. **エラー情報**
   - エラーメッセージ全文
   - スタックトレース
   - 再現手順

3. **ログファイル**
   - アプリケーションログ
   - システムログ（必要に応じて）

4. **設定情報**
   - 匿名化した設定ファイル
   - 環境変数（機密情報は除く）

### 既知の制限事項

1. **Docker Desktop必須**: Docker Engineのみでは一部機能が制限される場合があります
2. **macOS M1チップ**: Intel版Dockerコンテナを使用する場合は性能が劣化する可能性があります  
3. **Windows WSL2**: ファイル監視の遅延が発生する場合があります
4. **プロキシ環境**: 企業プロキシ環境では追加設定が必要な場合があります

### 最新情報の確認

- **GitHub Issues**: https://github.com/ProjectLens/issues
- **ドキュメント**: `docs/` ディレクトリ内の最新情報
- **実装ログ**: `_docs/implement-tasks/` の解決済み問題