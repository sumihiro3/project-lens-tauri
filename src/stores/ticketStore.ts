/**
 * チケット管理ストア
 * Backlogチケットの取得とカテゴリ分けを管理する
 */
import { defineStore } from 'pinia'
import { Ticket, TaskCategory } from '../types'

/**
 * チケットストアの定義
 */
export const useTicketStore = defineStore('tickets', {
  /**
   * ストアの状態を定義
   */
  state: () => ({
    /** チケット一覧 */
    tickets: [] as Ticket[],
    /** データ読み込み中フラグ */
    loading: false,
    /** エラー情報 */
    error: null as Error | null,
    /** 最終更新日時 */
    lastUpdated: null as string | null
  }),

  getters: {
    /**
     * 緊急タスクを取得
     * 期限が近い、または優先度が高いタスクを返す
     */
    urgentTasks: (state) => {
      // 緊急タスクの取得ロジック
      return []
    },
    
    /**
     * 推奨タスクを取得
     * AIによる分析結果に基づいて推奨されるタスクを返す
     */
    recommendedTasks: (state) => {
      // 推奨タスクの取得ロジック
      return []
    },
    
    /**
     * 関連タスクを取得
     * 現在選択中のタスクに関連するタスクを返す
     */
    relatedTasks: (state) => {
      // 関連タスクの取得ロジック
      return []
    }
  },

  actions: {
    /**
     * Backlogからチケット一覧を取得
     * MCP Serverを通じてBacklog APIを呼び出し、チケット情報を取得する
     * @returns チケット取得結果のPromise
     */
    async fetchTickets() {
      // チケット取得ロジック
    }
  }
})