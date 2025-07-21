// 共通の型定義

export interface Result<T> {
  data?: T;
  error?: Error;
  loading: boolean;
}

export interface Error {
  code: string;
  message: string;
  details?: unknown;
}

export interface Pagination {
  page: number;
  perPage: number;
  total: number;
}