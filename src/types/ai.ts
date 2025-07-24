/**
 * AI関連の型定義
 */

/**
 * サポートされているAIプロバイダーの列挙型
 */
export enum AIProvider {
  /** OpenAI GPT */
  OpenAI = 'openai',
  /** Anthropic Claude */
  Claude = 'claude',
  /** Google Gemini */
  Gemini = 'gemini'
}

/**
 * AI設定情報
 */
export interface AIConfig {
  /** 使用するAIプロバイダー */
  provider: AIProvider;
  /** APIキー */
  apiKey: string;
  /** 使用するモデル名 */
  model: string;
  /** 分析実行間隔（分単位） */
  analysisInterval: number;
}

/**
 * AIによるチケット分析結果
 */
export interface AIAnalysis {
  /** 分析対象チケットID */
  ticketId: string;
  /** 緊急度スコア（0-100） */
  urgencyScore: number;
  /** 複雑度スコア（0-100） */
  complexityScore: number;
  /** ユーザー関連度スコア（0-100） */
  userRelevanceScore: number;
  /** プロジェクト重み係数（1-10） */
  projectWeightFactor: number;
  /** 最終優先度スコア（計算結果） */
  finalPriorityScore: number;
  /** 推奨理由の説明文 */
  recommendationReason: string;
  /** カテゴリ分類結果 */
  category: TaskCategory;
}

/**
 * タスクカテゴリの列挙型
 */
export enum TaskCategory {
  /** 緊急タスク */
  Urgent = 'urgent',
  /** 推奨タスク */
  Recommended = 'recommended',
  /** 関連タスク */
  Related = 'related',
  /** 通常タスク */
  Normal = 'normal'
}