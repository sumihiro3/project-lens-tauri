// ユーティリティ関数

/**
 * 日付をフォーマットする
 * @param dateString ISO形式の日付文字列
 * @param format フォーマット
 * @returns フォーマットされた日付文字列
 */
export function formatDate(dateString: string, format: string = 'YYYY/MM/DD'): string {
  // 実装は後で追加
  return dateString;
}

/**
 * チケットの優先度に基づいて色を返す
 * @param priority チケットの優先度
 * @returns 対応する色コード
 */
export function getPriorityColor(priority: string): string {
  switch (priority) {
    case 'critical':
      return '#ff4d4f';
    case 'high':
      return '#faad14';
    case 'normal':
      return '#1890ff';
    case 'low':
      return '#52c41a';
    default:
      return '#d9d9d9';
  }
}

/**
 * エラーメッセージを整形する
 * @param error エラーオブジェクト
 * @returns 整形されたエラーメッセージ
 */
export function formatError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  return String(error);
}