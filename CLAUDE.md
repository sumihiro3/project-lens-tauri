# ProjectLens 開発ガイド

## プロジェクト概要
BacklogのMCP Serverを活用した複数プロジェクトのチケット整理・可視化を行うTauri + Nuxt3デスクトップアプリケーション

**技術スタック**: Tauri 2.x + Nuxt 3.x + Vue 3 + Pinia + Vuetify + Rust
**対象プラットフォーム**: macOS, Windows, Linux

---

## 📁 ドキュメント構造と参照ガイド

### 🎯 要件・設計（.kiro/specs/ - 既存）
```
.kiro/specs/multi-project-dashboard/
├── requirements.md              # 機能要件・非機能要件・開発環境要件
├── design-document.md           # アーキテクチャ・UI/UX設計・エラーハンドリング設計
├── technical-specifications.md  # 技術仕様・プラットフォーム別設定・状態管理
└── tasks.md                    # 実装タスクリスト（進捗管理）
```

### 💻 開発・実装（docs/ - 新規追加）
```
docs/
├── README.md                   # ドキュメント構造ガイド
├── development/
│   ├── implementation-guide.md  # 実装パターン・コーディング規約
│   ├── best-practices.md       # ベストプラクティス集
│   └── coding-standards.md     # コーディング標準
├── architecture/
│   └── error-handling.md       # エラーハンドリング詳細設計
└── troubleshooting.md          # トラブルシューティング
```

### 📝 実装記録（_docs/ - 既存）
```
_docs/
├── implement-tasks/             # タスク実装ログ
│   ├── 2.2-task-logs.md
│   └── dev-server-startup-errors-fix.md
└── blog/                       # 技術ブログ記事
```

---

## 🚨 重要な制約・注意点

### Docker依存関係（必須・ブロッキング）
- **全機能がDocker前提**: アプリケーション起動時に必須チェック実装
- **ブロッキング設計**: Docker未起動時は他の操作を完全に制限
- **段階的エラー表示**: トースト通知 → エラーダイアログ → ブロッキングダイアログ
- **必須ダイアログ**: 背景クリック・ESCキー・クローズボタン全て無効化

### macOS開発環境の特別設定

#### 設定が必要な理由と背景

macOSでのTauri + Nuxt3開発環境では、以下の特有の問題が発生するため特別な設定が必要です：

1. **子プロセス管理の問題**
   - macOSのセキュリティ機能により、DevToolsが子プロセスを適切に管理できずにSIGTERMエラーが発生
   - Tauri + Nuxtの組み合わせで開発サーバー起動時に競合が発生

2. **ファイル監視システムの制限**
   - macOSのfseventsがNuxt3のHMR（Hot Module Replacement）と競合
   - inotifyベースの監視がmacOSで不安定になることがある

3. **ネットワーク設定の問題**
   - localhostとTauriアプリ間の通信でCORSエラーが発生しやすい
   - ホスト名解決の問題でアプリが起動しない場合がある

#### 実装される設定とその効果

```typescript
// nuxt.config.ts - macOS固有設定
const isDarwin = process.platform === 'darwin'

export default defineNuxtConfig({
  devtools: { enabled: !isDarwin },  // 子プロセス管理問題回避
  vite: {
    server: {
      watch: {
        usePolling: isDarwin,         // ファイル監視はポーリングモード
        interval: isDarwin ? 300 : undefined
      },
      hmr: { overlay: false }         // HMR競合回避
    }
  },
  devServer: {
    host: '127.0.0.1',               // ホスト明示指定必須
    port: 8765
  }
})
```

**各設定の詳細な効果：**

- `devtools: { enabled: !isDarwin }`: DevToolsを無効化してSIGTERMエラーを防止
- `usePolling: isDarwin`: fseventsの代わりにポーリングベースの監視を使用
- `interval: 300`: ポーリング間隔を300msに設定（パフォーマンスと安定性のバランス）
- `hmr: { overlay: false }`: HMRエラーオーバーレイを無効化して画面表示の競合を防止
- `host: '127.0.0.1'`: IPアドレス直接指定でTauriとの通信を安定化

#### 発生する問題の例

これらの設定を行わない場合に発生する典型的なエラー：

```bash
# DevTools関連
Error: spawn EBADF
Error: SIGTERM received

# ファイル監視関連
Error: ENOSPC: System limit for number of file watchers reached
[vite] file change detected but HMR failed

# ネットワーク関連
Access to fetch at 'http://localhost:8765' from origin 'tauri://localhost' has been blocked by CORS policy
```

### エラーハンドリング設計原則
- **通知重複防止**: `isRetryMode` フラグによる制御
- **Store間通信**: カスタムイベント使用（循環参照回避）
- **エラー階層**: Info/Warning → Error → Critical(Blocking)

---

## 💡 実装時のクイックリファレンス

### Docker依存チェックパターン
```typescript
// 参照: src/stores/dockerStore.ts
const dockerStore = useDockerStore()

// 初期化（アプリ起動時）
await dockerStore.initializeDockerEnvironment()

// エラー処理（初回のみ通知作成）
if (!dockerStore.isDockerAvailable && !dockerStore.showErrorDialog) {
  dockerStore.handleDockerError('not-installed')
}

// 再試行（重複通知防止）
const retryDockerEnvironment = async () => {
  isRetryMode.value = true  // 個別チェック関数での通知を抑制
  try {
    await initializeDockerEnvironment()
    // 成功/失敗に応じた通知を表示
  } finally {
    isRetryMode.value = false
  }
}
```

### Store間通信パターン（循環参照回避）
```typescript
// 推奨: カスタムイベント
window.dispatchEvent(new CustomEvent('show-docker-error-dialog', {
  detail: { errorType, message }
}))

// イベントリスナー設定
window.addEventListener('show-docker-error-dialog', (event) => {
  const { errorType, message } = (event as CustomEvent).detail
  showInstallationGuide(errorType, message)
})

// 非推奨: 直接参照（循環参照リスク）
// const otherStore = useOtherStore()
```

### ブロッキングダイアログパターン
```vue
<!-- 参照: src/components/settings/DockerErrorDialog.vue -->
<template>
  <div class="blocking-dialog" v-if="visible">
    <!-- 背景クリック無効化 -->
    <div class="dialog-overlay"></div>
    
    <div class="dialog-content">
      <!-- クローズボタンなし -->
      <header class="dialog-header">
        <h2>{{ errorTitle }}</h2>
      </header>
      
      <!-- 解決アクションのみ提供 -->
      <footer class="dialog-footer">
        <button @click="retry" :disabled="isRetrying">
          {{ isRetrying ? '確認中...' : '再試行' }}
        </button>
      </footer>
    </div>
  </div>
</template>

<script setup>
// ESCキー無効化
const handleKeydown = (event) => {
  if (event.key === 'Escape' && props.visible) {
    event.preventDefault()
    event.stopPropagation()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown, true)
})
</script>
```

### 通知システムパターン
```typescript
// 参照: src/stores/notificationStore.ts
const notificationStore = useNotificationStore()

// 基本通知
notificationStore.error('エラータイトル', 'エラーメッセージ')

// アクション付き通知
notificationStore.error('Docker未起動', 'Docker Desktopを起動してください', {
  duration: 0,  // 永続表示
  actions: [
    {
      label: 'インストールガイド',
      type: 'primary',
      dismissOnClick: true,
      handler: () => {
        // カスタムイベント発行
        window.dispatchEvent(new CustomEvent('show-docker-error-dialog', {
          detail: { errorType: 'not-running' }
        }))
      }
    }
  ]
})
```

---

## 🔧 開発環境セットアップ

### 必須要件
- Node.js 20.x以上
- Docker Desktop（macOS/Windows）または Docker Engine（Linux）
- Yarn 1.22以上（推奨）

### トラブルシューティング
```bash
# spawn EBADF エラー（macOS）
# → DevTools無効化、ポーリング設定で解決

# 依存関係問題
rm -rf node_modules package-lock.json yarn.lock .nuxt .output
yarn install --network-timeout 100000

# 開発サーバー起動
NODE_OPTIONS="--max-old-space-size=4096" yarn dev
```

**詳細**: 
- 開発サーバー問題 → `_docs/implement-tasks/dev-server-startup-errors-fix.md`
- Docker関連問題 → `docs/troubleshooting.md#docker-issues`

---

## 📚 実装時の参照フロー

### 新機能実装時
1. **要件確認**: `.kiro/specs/multi-project-dashboard/requirements.md`
2. **設計方針**: `.kiro/specs/multi-project-dashboard/design-document.md`
3. **技術仕様**: `.kiro/specs/multi-project-dashboard/technical-specifications.md`
4. **実装パターン**: `docs/development/implementation-guide.md`
5. **過去の実装例**: `src/` ディレクトリの類似機能
6. **実装記録**: `_docs/implement-tasks/` の関連ログ

### バグ修正・問題解決時
1. **問題特定**: `docs/troubleshooting.md`
2. **既知の問題**: `_docs/implement-tasks/` の修正記録
3. **設計制約**: `.kiro/specs/` の制約事項確認
4. **修正パターン**: `docs/development/best-practices.md`

### エラーハンドリング実装時
1. **設計方針**: `docs/architecture/error-handling.md`
2. **実装パターン**: 上記クイックリファレンス
3. **参考実装**: `src/stores/dockerStore.ts`, `src/components/settings/DockerErrorDialog.vue`

---

## 🎯 重要なファイル参照

### 最新の実装例（参考コード）
- **Docker検証**: `src/stores/dockerStore.ts`
- **エラーダイアログ**: `src/components/settings/DockerErrorDialog.vue`
- **通知システム**: `src/stores/notificationStore.ts`
- **アプリ統合**: `src/app.vue`

### 設定ファイル
- **Nuxt設定**: `nuxt.config.ts`（プラットフォーム別設定重要）
- **Tauri設定**: `src-tauri/tauri.conf.json`
- **TypeScript**: `tsconfig.json`

### 現在実装済みの機能
- [x] Docker環境チェック機能
- [x] エラーハンドリングUI（ブロッキングダイアログ）
- [x] 通知システム（重複防止機能付き）
- [x] macOS開発環境対応

---

## ⚠️ 実装時の絶対に守るべきルール

1. **Docker依存**: 全機能実装前にDockerチェックを必須で実装
2. **エラー通知重複防止**: 必ず`isRetryMode`等のフラグで制御
3. **ブロッキングダイアログ**: 必須サービス未起動時は他操作を完全に制限
4. **Store間通信**: 循環参照回避のためカスタムイベント使用
5. **macOS対応**: 開発環境設定は必ずプラットフォーム判定で分岐

---

**このガイドは実装経験に基づいて継続的に更新されます。新しい知見や制約は随時追加してください。**

**最終更新**: 2025年7月22日 - Docker依存管理、エラーハンドリング、macOS開発環境対応を追加