# 技術仕様書

## 概要

ProjectLensは、Tauri 2.x + Nuxt 3.x + Vue 3技術スタックを使用し、BacklogのMCP Serverを活用したクロスプラットフォーム対応デスクトップアプリケーションです。本文書は実装に必要な技術的詳細仕様を定義します。

## 技術スタック詳細

### フロントエンド

#### フレームワーク構成
- **Tauri 2.x**: デスクトップアプリケーションフレームワーク
- **Nuxt 3.x**: Vue.jsベースのフルスタックフレームワーク
- **Vue 3**: プログレッシブJavaScriptフレームワーク
- **Pinia**: Vue 3用状態管理ライブラリ
- **Vuetify 3**: マテリアルデザインコンポーネントライブラリ

#### テンプレートエンジン
- **Pug (Jade)**: Vue.jsコンポーネントテンプレート用
  - インデントベースの簡潔な記法
  - HTMLよりも少ないコード量
  - 階層構造の明確な可視化

#### 状態管理アーキテクチャ
```typescript
// Piniaストア構成
stores/
├── dockerStore.ts        // Docker環境管理
├── notificationStore.ts  // 通知システム
├── settingsStore.ts      // アプリケーション設定
├── projectStore.ts       // プロジェクトデータ管理
└── aiStore.ts           // AI分析結果管理
```

### バックエンド

#### Rust構成
```rust
// src-tauri/src/ 構成
src/
├── main.rs              // アプリケーションエントリーポイント
├── lib.rs               // ライブラリルート
├── commands/            // Tauriコマンド群
│   ├── docker.rs        // Docker操作コマンド
│   ├── mcp.rs          // MCP Server通信
│   ├── ai.rs           // AI統合
│   └── storage.rs      // データ永続化
├── services/           // ビジネスロジック
│   ├── docker_service.rs
│   ├── mcp_service.rs
│   ├── ai_service.rs
│   └── storage_service.rs
└── models/             // データモデル
    ├── ticket.rs
    ├── project.rs
    └── analysis.rs
```

## 開発環境仕様

### 必須環境
- **Node.js**: v20.19以上
- **Yarn**: v1.22以上（推奨）
- **Rust**: 最新安定版（rustup経由）
- **Docker**: Docker Desktop（macOS/Windows）またはDocker Engine（Linux）

### macOS開発環境特別設定
Nuxt開発サーバーでの`spawn EBADF`エラー対策として以下の設定が必要：

```typescript
// nuxt.config.ts
export default defineNuxtConfig({
  devtools: { enabled: false },        // DevTools無効化
  vite: {
    server: {
      watch: {
        usePolling: true,              // ポーリングベースファイル監視
        interval: 1000
      },
      hmr: {
        overlay: false                 // HMRオーバーレイ無効化
      }
    }
  }
})
```

### Docker統合要件
```yaml
# Docker Compose設定例（MCP Server用）
version: '3.8'
services:
  backlog-mcp-server:
    image: backlog-mcp-server:latest
    ports:
      - "3001:3001"
    environment:
      - NODE_ENV=production
    volumes:
      - ./config:/app/config
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:3001/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

## エラーハンドリング技術仕様

### 必須サービス依存管理

#### Docker依存チェック実装
```typescript
// stores/dockerStore.ts
interface DockerServiceState {
  isDockerAvailable: boolean | null
  isDockerRunning: boolean | null
  dockerVersion: string | null
  showErrorDialog: boolean
  errorDialogType: 'not-installed' | 'not-running' | 'connection-failed'
  isRetryMode: boolean  // 重複通知防止フラグ
}

// Docker環境初期化フロー
async initializeDockerEnvironment() {
  this.isDockerAvailable = await checkDockerInstallation()
  if (!this.isDockerAvailable) {
    this.handleDockerError('not-installed')
    return
  }
  
  this.isDockerRunning = await checkDockerRunning()
  if (!this.isDockerRunning) {
    this.handleDockerError('not-running')
    return
  }
  
  this.dockerVersion = await getDockerVersion()
}
```

#### ブロッキングダイアログ実装
```vue
<!-- DockerErrorDialog.vue -->
<template lang="pug">
.docker-error-dialog(v-if="visible")
  .dialog-overlay
    // 背景クリック無効化
  .dialog-content
    header.dialog-header
      // クローズボタンなし
    .dialog-body
      // エラー内容とガイド
    footer.dialog-footer
      // 再試行ボタンのみ
</template>

<script setup lang="ts">
// ESCキー無効化
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.visible) {
    event.preventDefault()
    event.stopPropagation()
  }
}
</script>
```

### Store間通信パターン

#### 循環参照回避通信
```typescript
// カスタムイベントによる疎結合Store間通信
class StoreEventBus {
  // Docker エラーダイアログ表示要求
  static notifyDockerDialog(errorType: string, message?: string) {
    window.dispatchEvent(new CustomEvent('show-docker-error-dialog', {
      detail: { errorType, message }
    }))
  }
  
  // イベントリスナー設定
  static setupDockerDialogListener(handler: (detail: any) => void) {
    const listener = (event: CustomEvent) => handler(event.detail)
    window.addEventListener('show-docker-error-dialog', listener)
    return () => window.removeEventListener('show-docker-error-dialog', listener)
  }
}

// 使用例：notificationStore.ts
dockerError(message: string, dismissOnClick = true) {
  // 通知表示
  this.show(notification)
  
  // Docker ダイアログ表示要求（循環参照なし）
  if (!dismissOnClick) {
    StoreEventBus.notifyDockerDialog('not-installed', message)
  }
}
```

### 通知システム技術仕様

#### 階層化通知システム
```typescript
// 通知レベル定義
type NotificationLevel = 'info' | 'success' | 'warning' | 'error' | 'critical'

interface NotificationLevelConfig {
  level: NotificationLevel
  displayType: 'toast' | 'banner' | 'dialog'
  blocking: boolean
  autoClose: boolean
  duration: number
}

const NOTIFICATION_LEVELS: Record<NotificationLevel, NotificationLevelConfig> = {
  info: { level: 'info', displayType: 'toast', blocking: false, autoClose: true, duration: 4000 },
  success: { level: 'success', displayType: 'toast', blocking: false, autoClose: true, duration: 4000 },
  warning: { level: 'warning', displayType: 'banner', blocking: false, autoClose: false, duration: 0 },
  error: { level: 'error', displayType: 'toast', blocking: false, autoClose: false, duration: 8000 },
  critical: { level: 'critical', displayType: 'dialog', blocking: true, autoClose: false, duration: 0 }
}
```

#### 重複通知防止機構
```typescript
interface NotificationDeduplication {
  activeNotifications: Map<string, NotificationInstance>
  isRetryMode: boolean
  
  shouldShowNotification(type: string, context: string): boolean {
    const key = `${type}:${context}`
    
    // 再試行モード中は重複通知を抑制
    if (this.isRetryMode && type.includes('retry')) {
      return false
    }
    
    // 既存通知が存在する場合は抑制
    if (this.activeNotifications.has(key)) {
      return false
    }
    
    return true
  }
}
```

## データ永続化仕様

### ローカルストレージ構成
```rust
// SQLite データベーススキーマ
CREATE TABLE IF NOT EXISTS tickets (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL,
    priority INTEGER NOT NULL,
    assignee_id TEXT,
    reporter_id TEXT NOT NULL,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    due_date TEXT,
    raw_data TEXT NOT NULL -- JSON形式でオリジナルデータを保存
);

CREATE TABLE IF NOT EXISTS workspaces (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    domain TEXT NOT NULL,
    api_key_encrypted TEXT NOT NULL,
    encryption_version TEXT NOT NULL DEFAULT 'v1',  -- 🔥 Task 3.2で追加
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS project_weights (
    project_id TEXT PRIMARY KEY,
    project_name TEXT NOT NULL,
    workspace_id TEXT NOT NULL,
    weight_score INTEGER NOT NULL CHECK (weight_score BETWEEN 1 AND 10),
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_analyses (
    ticket_id TEXT PRIMARY KEY,
    urgency_score REAL NOT NULL,
    complexity_score REAL NOT NULL,
    user_relevance_score REAL NOT NULL,
    project_weight_factor REAL NOT NULL,
    final_priority_score REAL NOT NULL,
    recommendation_reason TEXT NOT NULL,
    category TEXT NOT NULL,
    analyzed_at TEXT NOT NULL,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id)
);
```

### 暗号化仕様
```rust
// AES-256-GCM による認証情報暗号化
use aes_gcm::{Aes256Gcm, Key, Nonce};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

pub struct CryptoService {
    key: Key<Aes256Gcm>,
}

impl CryptoService {
    pub fn new(password: &str, salt: &[u8]) -> Self {
        let mut key_bytes = [0u8; 32];
        pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 100_000, &mut key_bytes);
        let key = Key::<Aes256Gcm>::from_slice(&key_bytes);
        
        Self { key: *key }
    }
    
    pub fn encrypt(&self, plaintext: &str) -> Result<Vec<u8>, CryptoError> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(&generate_random_nonce());
        
        cipher.encrypt(nonce, plaintext.as_bytes())
            .map_err(|_| CryptoError::EncryptionFailed)
    }
    
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<String, CryptoError> {
        let cipher = Aes256Gcm::new(&self.key);
        let nonce = Nonce::from_slice(&ciphertext[..12]);
        
        let plaintext = cipher.decrypt(nonce, &ciphertext[12..])
            .map_err(|_| CryptoError::DecryptionFailed)?;
            
        String::from_utf8(plaintext)
            .map_err(|_| CryptoError::InvalidUtf8)
    }
}
```

## AI統合技術仕様

### マルチプロバイダー対応アーキテクチャ
```rust
// AI プロバイダー抽象化
pub trait AIProvider: Send + Sync {
    async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<Vec<AIAnalysis>, AIError>;
    async fn recommend_priorities(&self, analyses: Vec<AIAnalysis>) -> Result<Vec<Recommendation>, AIError>;
    fn provider_name(&self) -> &'static str;
    fn supported_models(&self) -> Vec<String>;
}

// OpenAI プロバイダー実装
pub struct OpenAIProvider {
    client: OpenAIClient,
    model: String,
}

impl AIProvider for OpenAIProvider {
    async fn analyze_tickets(&self, tickets: Vec<Ticket>) -> Result<Vec<AIAnalysis>, AIError> {
        let prompt = self.build_analysis_prompt(&tickets);
        let response = self.client.completions()
            .create(CreateCompletionRequest {
                model: self.model.clone(),
                prompt,
                max_tokens: Some(2000),
                temperature: Some(0.3),
                ..Default::default()
            })
            .await?;
            
        self.parse_analysis_response(&response)
    }
}

// Claude プロバイダー実装
pub struct ClaudeProvider {
    client: AnthropicClient,
    model: String,
}

// Gemini プロバイダー実装
pub struct GeminiProvider {
    client: GoogleAIClient,
    model: String,
}
```

### AI分析アルゴリズム
```rust
// 優先度スコア計算
pub fn calculate_priority_score(
    ticket: &Ticket,
    user_relevance: f32,
    project_weight: f32,
    urgency_factors: &UrgencyFactors,
) -> f32 {
    let base_score = match ticket.priority {
        Priority::Critical => 100.0,
        Priority::High => 80.0,
        Priority::Medium => 60.0,
        Priority::Low => 40.0,
    };
    
    let urgency_multiplier = calculate_urgency_multiplier(urgency_factors);
    let user_relevance_boost = user_relevance * 20.0; // 最大20点ブースト
    let project_weight_multiplier = project_weight / 5.0; // 1-10スケールを0.2-2.0に正規化
    
    let final_score = (base_score * urgency_multiplier + user_relevance_boost) * project_weight_multiplier;
    
    // 0-100の範囲にクランプ
    final_score.max(0.0).min(100.0)
}

// 緊急度計算
fn calculate_urgency_multiplier(factors: &UrgencyFactors) -> f32 {
    let mut multiplier = 1.0;
    
    // 期限による緊急度
    if let Some(due_date) = factors.due_date {
        let days_until_due = (due_date - Utc::now()).num_days();
        multiplier *= match days_until_due {
            ..=0 => 2.0,      // 期限切れ
            1..=1 => 1.8,     // 1日以内
            2..=3 => 1.5,     // 2-3日以内
            4..=7 => 1.2,     // 1週間以内
            _ => 1.0,         // それ以上
        };
    }
    
    // コメント活動による緊急度
    if factors.recent_comments > 3 {
        multiplier *= 1.3;
    }
    
    // メンション数による緊急度
    if factors.mentions_count > 1 {
        multiplier *= 1.2;
    }
    
    multiplier
}
```

## MCP Server統合仕様

### MCP通信プロトコル
```rust
// MCP Server通信クライアント
pub struct MCPClient {
    base_url: String,
    timeout: Duration,
    client: reqwest::Client,
}

impl MCPClient {
    pub async fn fetch_tickets(&self, workspace: &BacklogWorkspace) -> Result<Vec<Ticket>, MCPError> {
        let request = MCPRequest {
            method: "backlog.getIssues",
            params: MCPParams {
                workspace_domain: workspace.domain.clone(),
                api_key: workspace.decrypt_api_key()?,
                filters: IssueFilters {
                    status_id: vec![1, 2, 3], // 未対応、処理中、処理済み
                    assignee_id: Some(workspace.user_id.clone()),
                    count: 100,
                },
            },
        };
        
        let response = self.client
            .post(&format!("{}/mcp/call", self.base_url))
            .json(&request)
            .timeout(self.timeout)
            .send()
            .await?;
            
        let mcp_response: MCPResponse<Vec<BacklogIssue>> = response.json().await?;
        
        mcp_response.result
            .into_iter()
            .map(|issue| Ticket::from_backlog_issue(issue, workspace))
            .collect()
    }
    
    pub async fn health_check(&self) -> Result<HealthStatus, MCPError> {
        let response = self.client
            .get(&format!("{}/health", self.base_url))
            .timeout(Duration::from_secs(5))
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(HealthStatus::Healthy)
        } else {
            Ok(HealthStatus::Unhealthy)
        }
    }
}
```

### Docker統合コマンド
```rust
// Tauraコマンド：Docker操作
#[tauri::command]
pub async fn check_docker_availability() -> Result<DockerStatus, String> {
    let output = Command::new("docker")
        .args(&["--version"])
        .output()
        .await
        .map_err(|e| format!("Docker実行エラー: {}", e))?;
        
    if output.status.success() {
        let version = String::from_utf8_lossy(&output.stdout);
        Ok(DockerStatus {
            available: true,
            version: Some(version.trim().to_string()),
            running: check_docker_daemon().await?,
        })
    } else {
        Ok(DockerStatus {
            available: false,
            version: None,
            running: false,
        })
    }
}

#[tauri::command]
pub async fn start_mcp_server() -> Result<(), String> {
    let status = Command::new("docker")
        .args(&[
            "run", "-d",
            "--name", "backlog-mcp-server",
            "--restart", "unless-stopped",
            "-p", "3001:3001",
            "backlog-mcp-server:latest"
        ])
        .status()
        .await
        .map_err(|e| format!("MCPサーバー起動エラー: {}", e))?;
        
    if status.success() {
        Ok(())
    } else {
        Err("MCPサーバーの起動に失敗しました".to_string())
    }
}
```

## パフォーマンス仕様

### レスポンス性要件
- **アプリケーション起動**: 3秒以内
- **ダッシュボード初期表示**: 2秒以内
- **AI分析処理**: 5秒以内（100チケット）
- **チケット検索**: 500ms以内
- **設定画面表示**: 1秒以内

### メモリ使用量制限
- **最大メモリ使用量**: 512MB（通常時）
- **チケットキャッシュ**: 最大10,000件
- **AI分析結果キャッシュ**: 最大1,000件
- **画像・添付ファイルキャッシュ**: 最大100MB

### 同時処理仕様
```rust
// 並列API呼び出し実装
pub async fn fetch_all_workspace_tickets(
    workspaces: Vec<BacklogWorkspace>
) -> Result<Vec<Ticket>, MCPError> {
    let futures: Vec<_> = workspaces
        .into_iter()
        .map(|workspace| async move {
            let client = MCPClient::new();
            client.fetch_tickets(&workspace).await
        })
        .collect();
        
    let results = futures::future::join_all(futures).await;
    
    let mut all_tickets = Vec::new();
    for result in results {
        match result {
            Ok(tickets) => all_tickets.extend(tickets),
            Err(e) => log::warn!("ワークスペースからのチケット取得に失敗: {}", e),
        }
    }
    
    Ok(all_tickets)
}
```

## テスト仕様

### テストレベル構成
```
tests/
├── unit/                    # ユニットテスト
│   ├── stores/             # Piniaストアテスト
│   ├── components/         # Vueコンポーネントテスト
│   └── services/           # Rustサービステスト
├── integration/            # 統合テスト
│   ├── docker/            # Docker統合テスト
│   ├── mcp/              # MCP Server統合テスト
│   └── ai/               # AI統合テスト
└── e2e/                   # E2Eテスト
    ├── dashboard/         # ダッシュボード機能テスト
    ├── settings/          # 設定画面テスト
    └── error-handling/    # エラーハンドリングテスト
```

### モックとスタブ仕様
```typescript
// Docker Service Mock
export class MockDockerService {
  private mockAvailable = true
  private mockRunning = true
  
  async isDockerAvailable(): Promise<boolean> {
    await new Promise(resolve => setTimeout(resolve, 100)) // 実際の遅延をシミュレート
    return this.mockAvailable
  }
  
  setMockStatus(available: boolean, running: boolean) {
    this.mockAvailable = available
    this.mockRunning = running
  }
}

// MCP Client Mock
export class MockMCPClient {
  private mockTickets: Ticket[] = []
  
  async fetchTickets(workspace: BacklogWorkspace): Promise<Ticket[]> {
    return [...this.mockTickets]
  }
  
  setMockTickets(tickets: Ticket[]) {
    this.mockTickets = tickets
  }
}
```

## セキュリティ仕様詳細

### 認証情報保護
```rust
// プラットフォーム別安全な保存場所
pub fn get_credentials_path() -> PathBuf {
    match std::env::consts::OS {
        "windows" => {
            let appdata = std::env::var("APPDATA").expect("APPDATA環境変数が見つかりません");
            PathBuf::from(appdata).join("ProjectLens").join("credentials.enc")
        },
        "macos" => {
            let home = std::env::var("HOME").expect("HOME環境変数が見つかりません");
            PathBuf::from(home)
                .join("Library")
                .join("Application Support")
                .join("ProjectLens")
                .join("credentials.enc")
        },
        "linux" => {
            let home = std::env::var("HOME").expect("HOME環境変数が見つかりません");
            PathBuf::from(home).join(".config").join("ProjectLens").join("credentials.enc")
        },
        _ => panic!("サポートされていないOS"),
    }
}
```

### メモリ安全性
```rust
// 機密データの安全な消去
pub struct SecureString {
    inner: Vec<u8>,
}

impl SecureString {
    pub fn new(s: String) -> Self {
        Self {
            inner: s.into_bytes(),
        }
    }
    
    pub fn expose(&self) -> &str {
        std::str::from_utf8(&self.inner).expect("無効なUTF-8")
    }
}

impl Drop for SecureString {
    fn drop(&mut self) {
        // メモリを明示的にゼロクリア
        for byte in &mut self.inner {
            *byte = 0;
        }
    }
}
```

この技術仕様書は、ProjectLensの実装における具体的な技術的詳細を定義し、開発者が一貫した実装を行うためのガイドラインを提供します。