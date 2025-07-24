/**
 * チケット関連の型定義
 */

/**
 * Backlogチケット情報
 */
export interface Ticket {
  /** チケットID */
  id: string;
  /** 所属プロジェクトID */
  projectId: string;
  /** チケットタイトル */
  title: string;
  /** チケット詳細説明 */
  description: string;
  /** チケット状態 */
  status: TicketStatus;
  /** 優先度 */
  priority: Priority;
  /** 担当者（未設定の場合はundefined） */
  assignee?: User;
  /** 報告者 */
  reporter: User;
  /** コメント一覧 */
  comments: Comment[];
  /** メンション設定されたユーザー一覧 */
  mentions: User[];
  /** ウォッチャー一覧 */
  watchers: User[];
  /** 作成日時 */
  createdAt: string;
  /** 最終更新日時 */
  updatedAt: string;
  /** 期限日（設定されている場合） */
  dueDate?: string;
}

/**
 * チケット状態の列挙型
 */
export enum TicketStatus {
  /** 未対応 */
  Open = 'open',
  /** 処理中 */
  InProgress = 'in_progress',
  /** 処理済み */
  Resolved = 'resolved',
  /** 完了 */
  Closed = 'closed',
  /** 保留 */
  Pending = 'pending'
}

/**
 * 優先度の列挙型
 */
export enum Priority {
  /** 低 */
  Low = 'low',
  /** 中 */
  Normal = 'normal',
  /** 高 */
  High = 'high',
  /** 最高 */
  Critical = 'critical'
}

/**
 * ユーザー情報
 */
export interface User {
  /** ユーザーID */
  id: string;
  /** ユーザー名 */
  name: string;
  /** メールアドレス */
  email: string;
  /** アイコン画像URL */
  icon?: string;
}

/**
 * チケットコメント情報
 */
export interface Comment {
  /** コメントID */
  id: string;
  /** コメント内容 */
  content: string;
  /** コメント投稿者 */
  author: User;
  /** 投稿日時 */
  createdAt: string;
  /** 最終更新日時 */
  updatedAt: string;
}