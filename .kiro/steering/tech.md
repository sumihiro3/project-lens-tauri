# ProjectLens - 技術スタック

## アーキテクチャ
クロスプラットフォーム対応（Windows/macOS/Linux）のTauriフレームワークを使用したデスクトップアプリケーション。

## 技術スタック
- **フロントエンド**: Tauri + Vue.js (Nuxt3) + TypeScript
- **バックエンド**: Rust (Tauriバックエンド)
- **AI統合**: Mastra（LangChain代替）
- **外部連携**: Backlog MCP Server（Docker経由）
- **データ保存**: ローカルのみ（SQLite等）

## 対応AIプロバイダー
- OpenAI API
- Anthropic Claude API
- Google Gemini API

## 主要技術要件
- **プライバシー重視**: チケット内容はローカルでのみ処理
- **軽量**: 高速起動・低メモリ使用量
- **セキュア**: APIキーの暗号化保存
- **クロスプラットフォーム**: Windows/macOS/Linux対応
- **リアルタイム更新**: オフライン対応

## システム構成
```
┌─ Desktop App (Tauri) ──────────────────────┐
│ ┌─ Frontend (Nuxt3/Vue) ─────────────────┐ │
│ │ ・ダッシュボード画面                    │ │
│ │ ・設定画面                             │ │
│ │ ・プロジェクト重み管理                  │ │
│ └───────────────────────────────────────┘ │
│                                           │
│ ┌─ Backend (Rust) ───────────────────────┐ │
│ │ ・Mastra AI統合                        │ │
│ │ ・ローカルデータ管理                    │ │
│ │ ・APIキー暗号化                        │ │
│ └───────────────────────────────────────┘ │
└───────────────────────────────────────────┘
                     ↕
┌─ Docker Container ─────────────────────────┐
│ Backlog MCP Server                        │
│ ・Backlog API連携                         │
│ ・チケット情報取得                         │
└───────────────────────────────────────────┘
                     ↕
┌─ External Services ───────────────────────┐
│ ・Backlog API                             │
│ ・AI API (OpenAI/Claude/Gemini)           │
└───────────────────────────────────────────┘
```

## 開発フェーズ

### Phase 1: ベータ版MVP（3ヶ月）
- 基本機能（ユーザーAPI使用）
- フリープランのみ
- Backlog MCP Server連携
- フィードバック収集機能

### Phase 2: 正式版リリース（+3ヶ月）
- Pro プラン・決済機能
- 7日間トライアル
- ProjectLens提供AI
- 高度な分析機能

### Phase 3: スケール・拡張（+6ヶ月）
- 企業向け機能
- チームプラン
- 他ツール連携
- モバイル対応検討

## 共通コマンド

### 開発環境セットアップ
```bash
# 依存関係インストール
npm install
cargo install tauri-cli

# 開発サーバー起動
npm run tauri dev
```

### ビルド・テスト
```bash
# 本番ビルド
npm run tauri build

# テスト実行
npm run test
cargo test
```

### 外部依存
```bash
# Backlog MCP Server（別途起動が必要）
docker run -d --name backlog-mcp-server [image]
```

## 開発優先順位
1. ローカルファーストのデータ処理
2. 暗号化された認証情報管理
3. 外部MCP Server接続管理
4. マルチAIプロバイダー抽象化
5. クロスプラットフォーム互換性

## 注意事項
- チケットデータはローカルでのみ処理（外部送信禁止）
- APIキーは暗号化保存必須
- MCP ServerはDocker外部依存（アプリに内包しない）
- 接続エラー時の適切なハンドリング実装