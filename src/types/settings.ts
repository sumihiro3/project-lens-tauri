// 設定関連の型定義

export interface AppSettings {
  theme: Theme;
  language: Language;
  autoUpdate: boolean;
  aiConfig: AISettings;
  cacheConfig: CacheSettings;
  loggingConfig: LoggingSettings;
}

export enum Theme {
  Light = 'light',
  Dark = 'dark',
  System = 'system'
}

export enum Language {
  Japanese = 'ja',
  English = 'en'
}

export interface AISettings {
  provider: string;
  analysisInterval: number;
}

export interface CacheSettings {
  maxSize: string;
  retentionDays: number;
}

export interface LoggingSettings {
  level: LogLevel;
  maxFiles: number;
}

export enum LogLevel {
  Error = 'error',
  Warn = 'warn',
  Info = 'info',
  Debug = 'debug'
}