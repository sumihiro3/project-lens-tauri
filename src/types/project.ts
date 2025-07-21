// プロジェクト関連の型定義

export interface Project {
  id: string;
  name: string;
  key: string;
  workspaceName: string;
  description?: string;
  icon?: string;
}

export interface ProjectWeight {
  projectId: string;
  projectName: string;
  workspaceName: string;
  weightScore: number; // 1-10
  updatedAt: string;
}