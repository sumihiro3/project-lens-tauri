/**
 * プロジェクト管理ストア
 * Backlogプロジェクトの取得と重み付け設定を管理する
 */
import { defineStore } from 'pinia'
import { Project, ProjectWeight } from '../types'

/**
 * プロジェクトストアの定義
 */
export const useProjectStore = defineStore('projects', {
  /**
   * ストアの状態を定義
   */
  state: () => ({
    /** プロジェクト一覧 */
    projects: [] as Project[],
    /** プロジェクト重み付け設定一覧 */
    projectWeights: [] as ProjectWeight[],
    /** データ読み込み中フラグ */
    loading: false,
    /** エラー情報 */
    error: null as Error | null
  }),

  actions: {
    /**
     * Backlogからプロジェクト一覧を取得
     * @returns プロジェクト取得結果のPromise
     */
    async fetchProjects() {
      // プロジェクト取得ロジック
    },
    
    /**
     * プロジェクトの重み付けを保存
     * @param projectId プロジェクトID
     * @param weight 重み値（1-10の範囲）
     * @returns 保存結果のPromise
     */
    async saveProjectWeight(projectId: string, weight: number) {
      // プロジェクト重み保存ロジック
    }
  }
})