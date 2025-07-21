// AI関連の型定義

export enum AIProvider {
  OpenAI = 'openai',
  Claude = 'claude',
  Gemini = 'gemini'
}

export interface AIConfig {
  provider: AIProvider;
  apiKey: string;
  model: string;
  analysisInterval: number; // 分単位
}

export interface AIAnalysis {
  ticketId: string;
  urgencyScore: number;
  complexityScore: number;
  userRelevanceScore: number;
  projectWeightFactor: number;
  finalPriorityScore: number;
  recommendationReason: string;
  category: TaskCategory;
}

export enum TaskCategory {
  Urgent = 'urgent',
  Recommended = 'recommended',
  Related = 'related',
  Normal = 'normal'
}