/**
 * 通知管理ストア
 */
import { defineStore } from 'pinia'
import { ref, nextTick } from 'vue'

// 通知アクションの型定義
export interface NotificationAction {
  /** アクションラベル */
  label: string
  /** アクション処理関数 */
  handler?: () => void
  /** アクションタイプ */
  type?: 'default' | 'primary' | 'danger'
  /** クリック時にトーストを閉じるかどうか */
  dismissOnClick?: boolean
}

// 通知の型定義
export interface Notification {
  /** 一意のID */
  id: string
  /** 通知タイプ */
  type: 'success' | 'error' | 'warning' | 'info' | 'loading'
  /** タイトル */
  title: string
  /** メッセージ（オプション） */
  message?: string
  /** 自動削除時間（ミリ秒、0で永続表示） */
  duration?: number
  /** 手動で閉じることができるかどうか */
  dismissible?: boolean
  /** アクションボタン */
  actions?: NotificationAction[]
  /** 作成日時 */
  createdAt: Date
  /** 一時停止状態 */
  paused?: boolean
  /** タイマーID */
  timerId?: NodeJS.Timeout
}

// 通知作成時のオプション
export interface CreateNotificationOptions {
  /** 通知タイプ */
  type: Notification['type']
  /** タイトル */
  title: string
  /** メッセージ（オプション） */
  message?: string
  /** 自動削除時間（ミリ秒、デフォルト: 5000、0で永続表示） */
  duration?: number
  /** 手動で閉じることができるかどうか（デフォルト: true） */
  dismissible?: boolean
  /** アクションボタン */
  actions?: NotificationAction[]
  /** 重複チェック用のキー */
  key?: string
}

/**
 * 通知管理ストア
 */
export const useNotificationStore = defineStore('notification', () => {
  // 状態
  const notifications = ref<Notification[]>([])
  const maxNotifications = ref(5)
  
  // 通知IDのカウンター
  let notificationIdCounter = 0
  
  /**
   * 新しい通知IDを生成
   */
  const generateNotificationId = (): string => {
    return `notification-${++notificationIdCounter}-${Date.now()}`
  }
  
  /**
   * 通知を作成
   */
  const create = (options: CreateNotificationOptions): string => {
    // 重複チェック
    if (options.key) {
      const existingNotification = notifications.value.find(n => 
        n.title === options.title && n.message === options.message
      )
      if (existingNotification) {
        return existingNotification.id
      }
    }
    
    const id = generateNotificationId()
    const notification: Notification = {
      id,
      type: options.type,
      title: options.title,
      message: options.message,
      duration: options.duration ?? (options.type === 'loading' ? 0 : 5000),
      dismissible: options.dismissible ?? true,
      actions: options.actions,
      createdAt: new Date(),
      paused: false
    }
    
    // 最大通知数を超える場合は古いものを削除
    if (notifications.value.length >= maxNotifications.value) {
      const oldestNotification = notifications.value[notifications.value.length - 1]
      dismiss(oldestNotification.id)
    }
    
    // 通知を追加（新しい通知を先頭に）
    notifications.value.unshift(notification)
    
    // 自動削除タイマー設定
    if (notification.duration && notification.duration > 0) {
      notification.timerId = setTimeout(() => {
        dismiss(id)
      }, notification.duration)
    }
    
    return id
  }
  
  /**
   * 通知を削除
   */
  const dismiss = (id: string): boolean => {
    const index = notifications.value.findIndex(n => n.id === id)
    if (index === -1) return false
    
    const notification = notifications.value[index]
    
    // タイマーをクリア
    if (notification.timerId) {
      clearTimeout(notification.timerId)
    }
    
    // 通知を削除
    notifications.value.splice(index, 1)
    
    return true
  }
  
  /**
   * すべての通知を削除
   */
  const dismissAll = (): void => {
    // すべてのタイマーをクリア
    notifications.value.forEach(notification => {
      if (notification.timerId) {
        clearTimeout(notification.timerId)
      }
    })
    
    // 通知をクリア
    notifications.value = []
  }
  
  /**
   * 通知を一時停止/再開
   */
  const pause = (id: string): boolean => {
    const notification = notifications.value.find(n => n.id === id)
    if (!notification) return false
    
    notification.paused = true
    if (notification.timerId) {
      clearTimeout(notification.timerId)
      notification.timerId = undefined
    }
    
    return true
  }
  
  const resume = (id: string): boolean => {
    const notification = notifications.value.find(n => n.id === id)
    if (!notification || !notification.paused) return false
    
    notification.paused = false
    
    // 残り時間を計算してタイマーを再設定
    if (notification.duration && notification.duration > 0) {
      const elapsed = Date.now() - notification.createdAt.getTime()
      const remaining = notification.duration - elapsed
      
      if (remaining > 0) {
        notification.timerId = setTimeout(() => {
          dismiss(id)
        }, remaining)
      } else {
        // 既に期限切れの場合はすぐに削除
        nextTick(() => dismiss(id))
      }
    }
    
    return true
  }
  
  // 便利メソッド
  
  /**
   * 成功通知を表示
   */
  const success = (title: string, message?: string, options?: Partial<CreateNotificationOptions>): string => {
    return create({
      type: 'success',
      title,
      message,
      ...options
    })
  }
  
  /**
   * エラー通知を表示
   */
  const error = (title: string, message?: string, options?: Partial<CreateNotificationOptions>): string => {
    return create({
      type: 'error',
      title,
      message,
      duration: options?.duration ?? 8000, // エラーは少し長めに表示
      ...options
    })
  }
  
  /**
   * 警告通知を表示
   */
  const warning = (title: string, message?: string, options?: Partial<CreateNotificationOptions>): string => {
    return create({
      type: 'warning',
      title,
      message,
      duration: options?.duration ?? 6000,
      ...options
    })
  }
  
  /**
   * 情報通知を表示
   */
  const info = (title: string, message?: string, options?: Partial<CreateNotificationOptions>): string => {
    return create({
      type: 'info',
      title,
      message,
      ...options
    })
  }
  
  /**
   * ローディング通知を表示
   */
  const loading = (title: string, message?: string, options?: Partial<CreateNotificationOptions>): string => {
    return create({
      type: 'loading',
      title,
      message,
      duration: 0, // ローディングは手動で閉じる
      dismissible: false,
      ...options
    })
  }
  
  /**
   * Docker関連のエラー通知を表示
   */
  const dockerError = (errorType: 'not-installed' | 'not-running' | 'connection-failed', message?: string): string => {
    const titles = {
      'not-installed': 'Dockerが見つかりません',
      'not-running': 'Dockerが実行されていません',
      'connection-failed': 'Docker接続エラー'
    }
    
    const messages = {
      'not-installed': 'ProjectLensを使用するにはDockerのインストールが必要です。',
      'not-running': 'DockerデスクトップまたはDocker Engineを起動してください。',
      'connection-failed': 'Dockerとの接続に失敗しました。設定を確認してください。'
    }
    
    return error(
      titles[errorType],
      message || messages[errorType],
      {
        duration: 0,
        actions: [
          {
            label: 'インストールガイド',
            type: 'primary',
            dismissOnClick: true,
            handler: () => {
              // globalThisを使ってDockerエラーダイアログ表示イベントを発行
              if (typeof window !== 'undefined') {
                window.dispatchEvent(new CustomEvent('show-docker-error-dialog', {
                  detail: { errorType, message }
                }))
              }
            }
          },
          {
            label: '閉じる',
            type: 'default',
            dismissOnClick: true
          }
        ]
      }
    )
  }
  
  /**
   * 通知の存在チェック
   */
  const exists = (id: string): boolean => {
    return notifications.value.some(n => n.id === id)
  }
  
  /**
   * 通知の取得
   */
  const getById = (id: string): Notification | undefined => {
    return notifications.value.find(n => n.id === id)
  }
  
  /**
   * 特定タイプの通知数を取得
   */
  const countByType = (type: Notification['type']): number => {
    return notifications.value.filter(n => n.type === type).length
  }
  
  return {
    // 状態
    notifications,
    maxNotifications,
    
    // アクション
    create,
    dismiss,
    dismissAll,
    pause,
    resume,
    
    // 便利メソッド
    success,
    error,
    warning,
    info,
    loading,
    dockerError,
    
    // ユーティリティ
    exists,
    getById,
    countByType
  }
})

// 型エクスポート
export type { Notification, NotificationAction, CreateNotificationOptions }