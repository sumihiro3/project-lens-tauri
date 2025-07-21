// プロジェクト管理ストア
import { defineStore } from 'pinia'
import { Project, ProjectWeight } from '../types'

export const useProjectStore = defineStore('projects', {
  state: () => ({
    projects: [] as Project[],
    projectWeights: [] as ProjectWeight[],
    loading: false,
    error: null as Error | null
  }),

  actions: {
    async fetchProjects() {
      // プロジェクト取得ロジック
    },
    async saveProjectWeight(projectId: string, weight: number) {
      // プロジェクト重み保存ロジック
    }
  }
})