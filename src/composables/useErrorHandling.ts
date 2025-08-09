/**
 * エラーハンドリング標準化コンポーザブル
 * 技術仕様書準拠の統一エラーハンドリングパターンを提供
 */
import { ref, type Ref } from 'vue'
import { useNotificationStore } from '~/stores/notificationStore'

/** エラーレベル定義（技術仕様書準拠） */
export type ErrorLevel = 'info' | 'warning' | 'error' | 'critical'

/** エラーカテゴリ定義 */
export type ErrorCategory = 'system' | 'user' | 'network' | 'data'

/** 構造化されたエラータイプ定義 */
export interface ErrorDefinition {
  type: string
  category: ErrorCategory
  defaultLevel: ErrorLevel
  description: string
}

/** エラータイプのマッピング定義 */
const ERROR_DEFINITIONS: Record<string, ErrorDefinition> = {
  'not-installed': {
    type: 'not-installed',
    category: 'system',
    defaultLevel: 'critical',
    description: 'Required service is not installed'
  },
  'not-running': {
    type: 'not-running',
    category: 'system',
    defaultLevel: 'error',
    description: 'Required service is not running'
  },
  'connection-failed': {
    type: 'connection-failed',
    category: 'network',
    defaultLevel: 'error',
    description: 'Network connection failed'
  },
  'authentication-failed': {
    type: 'authentication-failed',
    category: 'user',
    defaultLevel: 'error',
    description: 'Authentication credentials are invalid'
  },
  'api-error': {
    type: 'api-error',
    category: 'network',
    defaultLevel: 'error',
    description: 'API request failed'
  },
  'validation-error': {
    type: 'validation-error',
    category: 'data',
    defaultLevel: 'warning',
    description: 'Input data validation failed'
  },
  'timeout-error': {
    type: 'timeout-error',
    category: 'network',
    defaultLevel: 'warning',
    description: 'Request timed out'
  },
  'unknown-error': {
    type: 'unknown-error',
    category: 'system',
    defaultLevel: 'error',
    description: 'An unexpected error occurred'
  }
} as const

/** エラータイプの型安全な定義 */
export type ErrorType = keyof typeof ERROR_DEFINITIONS

/** エラーレベル設定（技術仕様書準拠） */
interface ErrorLevelConfig {
  level: ErrorLevel
  displayType: 'toast' | 'banner' | 'dialog'
  blocking: boolean
  autoClose: boolean
  duration: number
}

const ERROR_LEVEL_CONFIGS: Record<ErrorLevel, ErrorLevelConfig> = {
  info: { 
    level: 'info', 
    displayType: 'toast', 
    blocking: false, 
    autoClose: true, 
    duration: 4000 
  },
  warning: { 
    level: 'warning', 
    displayType: 'banner', 
    blocking: false, 
    autoClose: false, 
    duration: 0 
  },
  error: { 
    level: 'error', 
    displayType: 'toast', 
    blocking: false, 
    autoClose: false, 
    duration: 8000 
  },
  critical: { 
    level: 'critical', 
    displayType: 'dialog', 
    blocking: true, 
    autoClose: false, 
    duration: 0 
  }
}

/** 重複通知防止機構 */
interface NotificationDeduplication {
  activeNotifications: Map<string, string>
  isRetryMode: boolean
}

/** エラーハンドリング設定オプション */
interface ErrorHandlingOptions {
  /** サービス名（例: "Docker", "MCP", "AI"） */
  serviceName: string
  /** 重複通知防止機構を使用するか */
  enableDeduplication?: boolean
  /** カスタムエラーダイアログイベント名 */
  customDialogEvent?: string
}

/** エラー情報インターフェース */
interface ErrorInfo {
  type: ErrorType
  category: ErrorCategory
  level: ErrorLevel
  title: string
  message: string
  description: string
  context?: Record<string, any>
}

/**
 * 標準化エラーハンドリングコンポーザブル
 */
export function useErrorHandling(options: ErrorHandlingOptions) {
  const notificationStore = useNotificationStore()
  
  // 重複通知防止状態
  const deduplication: NotificationDeduplication = {
    activeNotifications: new Map<string, string>(),
    isRetryMode: false
  }
  
  // エラーダイアログ状態
  const showErrorDialog = ref(false)
  const errorDialogType = ref<ErrorType>('unknown-error')
  const errorDialogMessage = ref<string>('')
  
  /**
   * エラー情報の正規化（型安全性向上版）
   */
  const normalizeError = (
    type: ErrorType, 
    message?: string, 
    level?: ErrorLevel
  ): ErrorInfo => {
    const errorDefinition = ERROR_DEFINITIONS[type]
    
    if (!errorDefinition) {
      throw new Error(`未定義のエラータイプです: ${type}`)
    }
    
    // デフォルトタイトル生成（エラーカテゴリ別）
    const generateTitle = (definition: ErrorDefinition): string => {
      switch (definition.category) {
        case 'system':
          return definition.type === 'not-installed' 
            ? `${options.serviceName}が見つかりません`
            : definition.type === 'not-running'
            ? `${options.serviceName}が実行されていません`
            : `${options.serviceName}システムエラー`
        case 'network':
          return definition.type === 'connection-failed'
            ? `${options.serviceName}接続エラー`
            : definition.type === 'api-error'
            ? `${options.serviceName} APIエラー`
            : `${options.serviceName}ネットワークエラー`
        case 'user':
          return definition.type === 'authentication-failed'
            ? `${options.serviceName}認証エラー`
            : `${options.serviceName}ユーザーエラー`
        case 'data':
          return definition.type === 'validation-error'
            ? '入力データエラー'
            : `${options.serviceName}データエラー`
        default:
          return `${options.serviceName}エラー`
      }
    }
    
    return {
      type,
      category: errorDefinition.category,
      level: level || errorDefinition.defaultLevel,
      title: generateTitle(errorDefinition),
      message: message || `${options.serviceName}でエラーが発生しました`,
      description: errorDefinition.description,
    }
  }
  
  /**
   * 重複通知チェック
   */
  const shouldShowNotification = (type: ErrorType, context: string): boolean => {
    if (!options.enableDeduplication) return true
    
    const key = `${type}:${context}`
    
    // 再試行モード中は重複通知を抑制
    if (deduplication.isRetryMode && type.includes('retry' as any)) {
      return false
    }
    
    // 既存通知が存在する場合は抑制
    if (deduplication.activeNotifications.has(key)) {
      return false
    }
    
    return true
  }
  
  /**
   * エラー処理の実行（技術仕様書準拠）
   */
  const handleError = (
    type: ErrorType,
    message?: string,
    level?: ErrorLevel,
    context?: Record<string, any>
  ): string | null => {
    const errorInfo = normalizeError(type, message, level)
    const config = ERROR_LEVEL_CONFIGS[errorInfo.level]
    const contextKey = JSON.stringify(context || {})
    
    // 重複チェック
    if (!shouldShowNotification(type, contextKey)) {
      return null
    }
    
    let notificationId: string | null = null
    
    // 表示タイプに応じて処理を分岐
    switch (config.displayType) {
      case 'toast':
        notificationId = showToastNotification(errorInfo, config)
        break
        
      case 'banner':
        notificationId = showBannerNotification(errorInfo, config)
        break
        
      case 'dialog':
        showDialogNotification(errorInfo, config)
        break
    }
    
    // 重複防止に登録
    if (options.enableDeduplication && notificationId) {
      deduplication.activeNotifications.set(`${type}:${contextKey}`, notificationId)
    }
    
    return notificationId
  }
  
  /**
   * トースト通知の表示
   */
  const showToastNotification = (errorInfo: ErrorInfo, config: ErrorLevelConfig): string => {
    const notificationOptions: any = {
      duration: config.duration,
      dismissible: true
    }
    
    // レベルに応じた通知メソッドを呼び出し
    switch (errorInfo.level) {
      case 'info':
        return notificationStore.info(errorInfo.title, errorInfo.message, notificationOptions)
      case 'warning':
        return notificationStore.warning(errorInfo.title, errorInfo.message, notificationOptions)
      case 'error':
        return notificationStore.error(errorInfo.title, errorInfo.message, notificationOptions)
      default:
        return notificationStore.error(errorInfo.title, errorInfo.message, notificationOptions)
    }
  }
  
  /**
   * バナー通知の表示
   */
  const showBannerNotification = (errorInfo: ErrorInfo, config: ErrorLevelConfig): string => {
    return notificationStore.warning(errorInfo.title, errorInfo.message, {
      duration: config.duration,
      dismissible: true,
      actions: [
        {
          label: '詳細を確認',
          type: 'primary',
          dismissOnClick: true,
          handler: () => showErrorDialog.value = true
        },
        {
          label: '閉じる',
          type: 'default',
          dismissOnClick: true
        }
      ]
    })
  }
  
  /**
   * ダイアログ通知の表示
   */
  const showDialogNotification = (errorInfo: ErrorInfo, config: ErrorLevelConfig): void => {
    errorDialogType.value = errorInfo.type
    errorDialogMessage.value = errorInfo.message
    showErrorDialog.value = true
    
    // カスタムイベント発行（Store間通信パターン）
    if (typeof window !== 'undefined') {
      const eventName = options.customDialogEvent || `show-${options.serviceName.toLowerCase()}-error-dialog`
      window.dispatchEvent(new CustomEvent(eventName, {
        detail: { 
          errorType: errorInfo.type, 
          message: errorInfo.message,
          level: errorInfo.level
        }
      }))
    }
  }
  
  /**
   * 再試行モードの制御
   */
  const setRetryMode = (enabled: boolean): void => {
    if (options.enableDeduplication) {
      deduplication.isRetryMode = enabled
    }
  }
  
  /**
   * エラーダイアログを閉じる
   */
  const closeErrorDialog = (): void => {
    showErrorDialog.value = false
  }
  
  /**
   * 通知の削除とクリーンアップ
   */
  const clearNotification = (notificationId: string): void => {
    notificationStore.dismiss(notificationId)
    
    // 重複防止マップからも削除
    if (options.enableDeduplication) {
      for (const [key, id] of deduplication.activeNotifications.entries()) {
        if (id === notificationId) {
          deduplication.activeNotifications.delete(key)
          break
        }
      }
    }
  }
  
  /**
   * すべての通知をクリア
   */
  const clearAllNotifications = (): void => {
    if (options.enableDeduplication) {
      deduplication.activeNotifications.clear()
    }
  }
  
  return {
    // 状態
    showErrorDialog: showErrorDialog as Readonly<Ref<boolean>>,
    errorDialogType: errorDialogType as Readonly<Ref<ErrorType>>,
    errorDialogMessage: errorDialogMessage as Readonly<Ref<string>>,
    
    // メソッド  
    handleError,
    setRetryMode,
    closeErrorDialog,
    clearNotification,
    clearAllNotifications,
    
    // 便利メソッド
    handleCriticalError: (type: ErrorType, message?: string) => 
      handleError(type, message, 'critical'),
    handleWarning: (type: ErrorType, message?: string) => 
      handleError(type, message, 'warning'),
    handleInfo: (type: ErrorType, message?: string) => 
      handleError(type, message, 'info'),
  }
}

/** エラータイプのエクスポート */
export { 
  type ErrorType, 
  type ErrorLevel, 
  type ErrorLevelConfig, 
  type ErrorCategory, 
  type ErrorDefinition,
  ERROR_DEFINITIONS 
}