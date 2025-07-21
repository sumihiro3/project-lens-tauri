// チケット関連の型定義

export interface Ticket {
  id: string;
  projectId: string;
  title: string;
  description: string;
  status: TicketStatus;
  priority: Priority;
  assignee?: User;
  reporter: User;
  comments: Comment[];
  mentions: User[];
  watchers: User[];
  createdAt: string;
  updatedAt: string;
  dueDate?: string;
}

export enum TicketStatus {
  Open = 'open',
  InProgress = 'in_progress',
  Resolved = 'resolved',
  Closed = 'closed',
  Pending = 'pending'
}

export enum Priority {
  Low = 'low',
  Normal = 'normal',
  High = 'high',
  Critical = 'critical'
}

export interface User {
  id: string;
  name: string;
  email: string;
  icon?: string;
}

export interface Comment {
  id: string;
  content: string;
  author: User;
  createdAt: string;
  updatedAt: string;
}