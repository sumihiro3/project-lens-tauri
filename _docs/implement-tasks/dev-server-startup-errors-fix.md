# Nuxt開発サーバー起動エラーの解決記録

## 対応日時
2025年7月22日

## 発生した問題

### 1. spawn EBADF エラー（複数回発生）
```
ERROR  [unhandledRejection] spawn EBADF                                                                                  6:20:31 PM

    at ChildProcess.spawn (node:internal/child_process:420:11)
    at spawn (node:child_process:762:9)
    at fork (node:child_process:172:10)
    at restart (node_modules/@nuxt/cli/dist/chunks/dev.mjs:279:17)
    at startSubprocess (node_modules/@nuxt/cli/dist/chunks/dev.mjs:327:9)
    at Object.run (node_modules/@nuxt/cli/dist/chunks/dev.mjs:151:19)
    at async runCommand (node_modules/citty/dist/index.mjs:316:16)
    at async runCommand (node_modules/citty/dist/index.mjs:307:11)
    at async runMain (node_modules/citty/dist/index.mjs:445:7)

✨  Done in 12.52s.
 ERROR  [unhandledRejection] spawn EBADF                                                                                  6:20:38 PM

    at ChildProcess.spawn (node:internal/child_process:420:11)
    at spawn (node:child_process:762:9)
    at fork (node:child_process:172:10)
    at restart (node_modules/@nuxt/cli/dist/chunks/dev.mjs:279:17)
    at startSubprocess (node_modules/@nuxt/cli/dist/chunks/dev.mjs:327:9)
    at Object.run (node_modules/@nuxt/cli/dist/chunks/dev.mjs:151:19)
    at async runCommand (node_modules/citty/dist/index.mjs:316:16)
    at async runCommand (node_modules/citty/dist/index.mjs:307:11)
    at async runMain (node_modules/citty/dist/index.mjs:445:7)
```

### 2. Promise Rejection Warning
```
ERROR  (node:84932) PromiseRejectionHandledWarning: Promise rejection was handled asynchronously (rejection id: 627)     6:20:38 PM
(Use node --trace-warnings ... to show where the warning was created)
```

### 3. Vite Pre-transform エラー（複数回発生）
```
ERROR  Pre-transform error: The service was stopped                                                                      6:20:38 PM
  Plugin: vite:client-inject
  File: /Users/sumihiro/projects/ProjectLens/node_modules/@vue/reactivity/dist/reactivity.esm-bundler.js?v=0bc5e60c


 ERROR  Pre-transform error: The service was stopped                                                                      6:20:38 PM
  Plugin: vite:client-inject
  File: /Users/sumihiro/projects/ProjectLens/node_modules/@vue/reactivity/dist/reactivity.esm-bundler.js?v=0bc5e60c (x2)
```

## 環境情報
- **OS**: macOS Darwin 24.5.0
- **Node.js**: v20.19.4
- **npm**: 10.8.2
- **yarn**: 1.22.22
- **Nuxt**: 3.17.7
- **プロジェクト**: Tauri + Nuxt3 + Vue3 アプリケーション

## エラーの原因分析

### spawn EBADF エラーの原因
- Nuxtの子プロセス管理における問題
- macOSでのファイルディスクリプター処理の問題
- Tauriアプリケーションとの組み合わせで特に発生しやすい
- Nuxt DevToolsの子プロセス管理による競合

### Vite Pre-transform エラーの原因
- Viteのサービスが異常終了することによる
- HMR（Hot Module Replacement）の競合
- ファイルウォッチャーの不安定性

## 実施した解決策

### 1. 依存関係のクリーンアップと再インストール
```bash
# 既存のnode_modulesとロックファイルを削除
rm -rf node_modules package-lock.json yarn.lock

# yarnで依存関係を再インストール
yarn install --network-timeout 100000
```

### 2. Nuxtキャッシュのクリア
```bash
# Nuxtの生成されたキャッシュを削除
rm -rf .nuxt .output
```

### 3. nuxt.config.ts の修正

#### devtoolsの無効化
```typescript
devtools: {
  enabled: false,  // trueから変更
},
```

**理由**: DevToolsの子プロセス管理がspawnエラーの原因となっていた

#### devServerの設定追加
```typescript
devServer: {
  port: 8765,
  host: '127.0.0.1'  // 追加
},
```

**理由**: ホストを明示的に指定することで接続を安定化

#### Vite設定の強化
```typescript
vite: {
  // 既存の設定...
  
  // 開発サーバー設定（プロセス管理の改善）
  server: {
    watch: {
      usePolling: true,    // ポーリングベースのウォッチャー使用
      interval: 300        // ポーリング間隔
    },
    hmr: {
      overlay: false       // HMRエラーオーバーレイ無効化
    }
  }
},
```

**理由**: 
- `usePolling: true`: ファイルシステムイベントの代わりにポーリングを使用し、macOSでの安定性を向上
- `overlay: false`: HMRエラー表示の競合を回避
- `interval: 300`: 適度なポーリング間隔でパフォーマンスとレスポンシブ性のバランスを取る

### 4. 起動コマンドの調整
```bash
# メモリ制限を上げて実行
NODE_OPTIONS="--max-old-space-size=4096" yarn dev
```

## 解決結果

### 修正前の状態
- 開発サーバーが起動時にspawn EBADFエラーで異常終了
- Viteのpre-transformエラーが頻発
- 安定した開発環境の構築ができない

### 修正後の状態
```
yarn run v1.22.22
$ nuxt dev
[nuxi] Nuxt 3.17.7 with Nitro 2.12.4

  ➜ Local:    http://127.0.0.1:8765/
  ➜ Network:  use --host to expose

ℹ Nuxt Icon server bundle mode is set to local
✔ Vite client built in 28ms
✔ Vite server built in 155ms
[nitro] ✔ Nuxt Nitro server built in 1101ms
ℹ Vite client warmed up in 1ms
```

- エラーなしで正常に起動
- 安定した開発環境の提供
- HMRも正常に動作

## 学んだこと・ベストプラクティス

### 1. Tauriアプリでの開発サーバー設定
- DevToolsは本番環境に近い状態でのテストでは無効化を検討
- ホスト指定を明示的に行う（`127.0.0.1`推奨）
- ポーリングベースのファイルウォッチャーがmacOSで安定

### 2. エラー対応の手順
1. 依存関係の完全なクリーンアップ
2. キャッシュのクリア
3. 設定ファイルの段階的な修正
4. 環境固有の最適化

### 3. 予防策
- 定期的な`node_modules`のクリーンアップ
- プラットフォーム固有の設定を予め用意
- 開発環境と本番環境の設定分離

## 今後の改善点

### 1. プラットフォーム別設定
```typescript
// 将来的な改善案
const isDarwin = process.platform === 'darwin'
const isWindows = process.platform === 'win32'

export default defineNuxtConfig({
  vite: {
    server: {
      watch: {
        usePolling: isDarwin, // macOSでのみポーリング使用
        interval: isDarwin ? 300 : undefined
      }
    }
  }
})
```

### 2. 環境変数での制御
```bash
# .env.development
NUXT_DEVTOOLS_ENABLED=false
NUXT_DEV_SERVER_HOST=127.0.0.1
NUXT_VITE_POLLING=true
```

### 3. 自動復旧機能
プロセス異常終了時の自動再起動機能の実装を検討

## 関連リンク
- [Nuxt DevTools 設定ガイド](https://devtools.nuxt.com/)
- [Vite Server Options](https://vitejs.dev/config/server-options.html)
- [Tauri Development Guide](https://tauri.app/v1/guides/development/development-cycle)

---

**注意**: この記録は macOS Darwin 24.5.0 環境での解決例です。他のOSや環境では異なる対応が必要な場合があります。