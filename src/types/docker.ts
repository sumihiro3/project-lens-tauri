/**
 * Docker関連の型定義
 */

/**
 * コンテナの状態を表す型
 */
export interface ContainerStatus {
  /** コンテナ名 */
  name: string;
  /** コンテナの状態 (running, stopped など) */
  state: string;
  /** コンテナが実行中かどうか */
  isRunning: boolean;
}

/**
 * Dockerサービスのレスポンス型
 */
export type DockerResponse<T> = {
  /** 処理が成功したかどうか */
  success: boolean;
  /** 処理結果のデータ */
  data?: T;
  /** エラーメッセージ (エラー時のみ) */
  error?: string;
};