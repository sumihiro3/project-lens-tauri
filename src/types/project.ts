/**
 * プロジェクト関連の型定義
 */

/**
 * Backlogプロジェクト情報
 */
export interface Project {
  /** プロジェクトID */
  id: string;
  /** プロジェクト名 */
  name: string;
  /** プロジェクトキー */
  key: string;
  /** ワークスペース名 */
  workspaceName: string;
  /** プロジェクト説明 */
  description?: string;
  /** プロジェクトアイコン */
  icon?: string;
}

/**
 * プロジェクト重み付け設定
 * ダッシュボードでの表示優先度を決定するための重み値を保持
 */
export interface ProjectWeight {
  /** プロジェクトID */
  projectId: string;
  /** プロジェクト名 */
  projectName: string;
  /** ワークスペース名 */
  workspaceName: string;
  /** 重み値（1-10の範囲で設定） */
  weightScore: number;
  /** 設定更新日時 */
  updatedAt: string;
}