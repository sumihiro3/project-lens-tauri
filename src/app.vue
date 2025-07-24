<template lang="pug">
div
  NuxtPage
  
  //- システム通知トースト
  SystemNotificationToast
  
  //- Dockerエラーダイアログ
  DockerErrorDialog(
    :visible="dockerStore.showErrorDialog"
    :errorType="dockerStore.errorDialogType"
    :errorMessage="dockerStore.error"
  )
</template>

<script setup lang="ts">
import { onMounted, onErrorCaptured } from 'vue'
import { useDockerStore } from '~/stores/dockerStore'
import { useNotificationStore } from '~/stores/notificationStore'
import SystemNotificationToast from '~/components/common/SystemNotificationToast.vue'
import DockerErrorDialog from '~/components/settings/DockerErrorDialog.vue'

// ストアの初期化
const dockerStore = useDockerStore()
const notificationStore = useNotificationStore()

// エラー境界の実装
onErrorCaptured((error, instance, info) => {
  console.error('アプリケーションエラー:', error)
  console.error('エラー情報:', info)
  console.error('コンポーネントインスタンス:', instance)
  
  // 通知でエラーを表示
  notificationStore.error(
    'アプリケーションエラー',
    '予期しないエラーが発生しました。アプリケーションを再起動してください。',
    { 
      duration: 0, // 永続表示
      actions: [
        {
          label: 'リロード',
          type: 'primary',
          handler: () => window.location.reload()
        },
        {
          label: '閉じる',
          type: 'default',
          dismissOnClick: true
        }
      ]
    }
  )
  
  // エラーを伝播させない（falseを返すことでエラーをキャッチ）
  return false
})

// アプリケーション初期化
onMounted(async () => {
  // アプリケーション開始通知
  notificationStore.info(
    'ProjectLens 起動中',
    'Docker環境を確認しています...',
    { duration: 3000 }
  )
  
  // Docker環境の初期化
  try {
    await dockerStore.initializeDockerEnvironment()
    
    if (dockerStore.isDockerAvailable && dockerStore.isDockerRunning) {
      notificationStore.success(
        'ProjectLens 準備完了',
        'すべてのサービスが正常に動作しています。',
        { duration: 4000 }
      )
    }
  } catch (error) {
    console.error('アプリケーション初期化エラー:', error)
    notificationStore.error(
      'アプリケーション初期化エラー',
      'ProjectLensの初期化中にエラーが発生しました。',
      { duration: 8000 }
    )
  }
})

// SEO設定
useHead({
  title: 'ProjectLens - AI-Powered Project Management Dashboard',
  meta: [
    { name: 'description', content: 'ProjectLens: BacklogのMCP Serverを活用して複数プロジェクトのチケットをAIで整理・可視化するデスクトップアプリケーション' },
    { name: 'viewport', content: 'width=device-width, initial-scale=1' },
    { charset: 'utf-8' }
  ],
  link: [
    { rel: 'icon', type: 'image/x-icon', href: '/favicon.ico' }
  ]
})
</script>

<style>
/* グローバルスタイル */
html {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  line-height: 1.6;
  color: #2c3e50;
}

body {
  margin: 0;
  padding: 0;
  background-color: #f8f9fa;
  overflow-x: hidden;
}

* {
  box-sizing: border-box;
}

/* スクロールバースタイリング */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: #a8a8a8;
}

/* フォーカス時のアウトライン */
button:focus,
input:focus,
select:focus,
textarea:focus {
  outline: 2px solid #4f46e5;
  outline-offset: 2px;
}

/* ボタンの基本スタイル */
button {
  cursor: pointer;
  transition: all 0.2s ease-in-out;
}

button:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

/* リンクスタイル */
a {
  color: #4f46e5;
  text-decoration: none;
  transition: color 0.2s ease-in-out;
}

a:hover {
  color: #3730a3;
  text-decoration: underline;
}

/* ダークモード対応の基本設定 */
@media (prefers-color-scheme: dark) {
  html {
    color: #e2e8f0;
  }
  
  body {
    background-color: #1a202c;
  }
  
  ::-webkit-scrollbar-track {
    background: #2d3748;
  }
  
  ::-webkit-scrollbar-thumb {
    background: #4a5568;
  }
  
  ::-webkit-scrollbar-thumb:hover {
    background: #718096;
  }
  
  a {
    color: #60a5fa;
  }
  
  a:hover {
    color: #93c5fd;
  }
}

/* アクセシビリティ: 動きを減らす設定 */
@media (prefers-reduced-motion: reduce) {
  *,
  *::before,
  *::after {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
    scroll-behavior: auto !important;
  }
}

/* プリント時のスタイル */
@media print {
  body {
    background: white !important;
    color: black !important;
  }
  
  /* 不要な要素を隠す */
  .notification-container,
  .docker-error-dialog {
    display: none !important;
  }
}
</style>