// AI分析ストア
import { defineStore } from 'pinia'
import { AIAnalysis, AIProvider } from '../types'

export const useAIStore = defineStore('ai', {
  state: () => ({
    analyses: [] as AIAnalysis[],
    loading: false,
    error: null as Error | null,
    currentProvider: AIProvider.OpenAI
  }),

  actions: {
    async analyzeTickets() {
      // チケット分析ロジック
    },
    async changeProvider(provider: AIProvider) {
      // AIプロバイダー変更ロジック
    }
  }
})