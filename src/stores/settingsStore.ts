// 設定管理ストア
import { defineStore } from 'pinia'
import { AppSettings, Theme, Language } from '../types'

export const useSettingsStore = defineStore('settings', {
  state: () => ({
    settings: {
      theme: Theme.Light,
      language: Language.Japanese,
      autoUpdate: true,
      aiConfig: {
        provider: 'openai',
        analysisInterval: 300
      },
      cacheConfig: {
        maxSize: '100MB',
        retentionDays: 7
      },
      loggingConfig: {
        level: 'info',
        maxFiles: 5
      }
    } as AppSettings
  }),

  actions: {
    async saveSettings() {
      // 設定保存ロジック
    },
    async loadSettings() {
      // 設定読み込みロジック
    }
  }
})