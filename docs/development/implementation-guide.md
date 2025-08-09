# 実装ガイド

## 概要

本ガイドは、ProjectLens開発における具体的な実装パターンと注意点をまとめたものです。実装時の参考資料として活用してください。

## 重要な実装パターン

### Docker依存管理パターン

ProjectLensは全機能がDocker前提で設計されています。以下のパターンを必ず実装してください：

#### 1. 起動時Docker環境チェック
```typescript
// app.vue での実装例
onMounted(async () => {
  try {
    await dockerStore.initializeDockerEnvironment()
    
    if (dockerStore.isDockerAvailable && dockerStore.isDockerRunning) {
      notificationStore.success('ProjectLens 準備完了', 'すべてのサービスが正常に動作しています。')
    }
  } catch (error) {
    console.error('アプリケーション初期化エラー:', error)
    notificationStore.error('アプリケーション初期化エラー', 'ProjectLensの初期化中にエラーが発生しました。')
  }
})
```

#### 2. ブロッキングエラーダイアログ
```vue
<!-- DockerErrorDialog.vue パターン -->
<template lang="pug">
.docker-error-dialog(v-if="visible")
  .dialog-overlay
    // 背景クリック無効化（クリックイベントなし）
  .dialog-content
    header.dialog-header
      // クローズボタンは配置しない
    .dialog-body
      // エラー詳細とOS別インストールガイド
    footer.dialog-footer
      // 再試行ボタンのみ
</template>

<script setup lang="ts">
// ESCキー無効化
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape' && props.visible) {
    event.preventDefault()
    event.stopPropagation()
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown, true)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown, true)
})
</script>
```

### Store間通信パターン

#### 循環参照回避のカスタムイベント通信
```typescript
// StoreEventBus パターン
class StoreEventBus {
  static notifyDockerDialog(errorType: string, message?: string) {
    window.dispatchEvent(new CustomEvent('show-docker-error-dialog', {
      detail: { errorType, message }
    }))
  }
  
  static setupDockerDialogListener(handler: (detail: any) => void) {
    const listener = (event: CustomEvent) => handler(event.detail)
    window.addEventListener('show-docker-error-dialog', listener)
    return () => window.removeEventListener('show-docker-error-dialog', listener)
  }
}

// notificationStore.ts での使用例
dockerError(message: string, dismissOnClick = true) {
  const notification = this.createNotification({
    type: 'error',
    title: 'Docker環境エラー',
    message,
    dismissOnClick,
    actions: dismissOnClick ? [] : [
      {
        label: 'インストールガイド',
        type: 'primary',
        handler: () => {
          // Docker ダイアログ表示要求（循環参照なし）
          StoreEventBus.notifyDockerDialog('not-installed', message)
        }
      }
    ]
  })
  
  this.notifications.push(notification)
}
```

### 通知重複防止パターン

#### 再試行モードでの重複制御
```typescript
// dockerStore.ts での実装例
async retryDockerEnvironment(): Promise<boolean> {
  // 再試行中フラグを設定（重複通知防止）
  this.isRetryMode = true
  
  try {
    await this.initializeDockerEnvironment()
    
    if (this.isDockerAvailable && this.isDockerRunning) {
      // 成功時のみダイアログを閉じる
      this.showErrorDialog = false
      return true
    } else {
      // 失敗時の通知（再試行中は抑制される）
      if (!this.isRetryMode) {
        notificationStore.error('Docker再試行失敗', 'Docker環境が正常に動作していません。')
      }
      return false
    }
  } finally {
    // 必ず再試行フラグをクリア
    this.isRetryMode = false
  }
}
```

## macOS開発環境対応

### 必須設定: nuxt.config.ts
```typescript
export default defineNuxtConfig({
  // macOS環境でのspawn EBADFエラー対策
  devtools: { enabled: false },
  
  vite: {
    server: {
      watch: {
        usePolling: true,    // ポーリングベースファイル監視
        interval: 1000
      },
      hmr: {
        overlay: false       // HMRオーバーレイ無効化
      }
    }
  }
})
```

### よくある問題と解決方法
1. **spawn EBADF エラー**: DevTools無効化 + ポーリング使用
2. **HMR動作不安定**: HMRオーバーレイ無効化
3. **ファイル監視失効**: usePolling: true に設定

## Pinia Store実装パターン

### 基本構造
```typescript
// stores/exampleStore.ts
interface ExampleState {
  data: any[]
  loading: boolean
  error: string | null
}

export const useExampleStore = defineStore('example', () => {
  // State
  const state = reactive<ExampleState>({
    data: [],
    loading: false,
    error: null
  })
  
  // Getters
  const hasError = computed(() => state.error !== null)
  const isEmpty = computed(() => state.data.length === 0)
  
  // Actions
  const fetchData = async () => {
    state.loading = true
    state.error = null
    
    try {
      const result = await apiCall()
      state.data = result
    } catch (error) {
      state.error = error.message
      console.error('データ取得エラー:', error)
    } finally {
      state.loading = false
    }
  }
  
  const resetError = () => {
    state.error = null
  }
  
  return {
    // State
    ...toRefs(state),
    
    // Getters  
    hasError,
    isEmpty,
    
    // Actions
    fetchData,
    resetError
  }
})
```

### 非同期処理での注意点
```typescript
// ❌ 間違った例：エラーハンドリング不備
const badAction = async () => {
  const result = await apiCall() // エラー時の処理なし
  state.data = result
}

// ✅ 正しい例：適切なエラーハンドリング
const goodAction = async () => {
  state.loading = true
  state.error = null
  
  try {
    const result = await apiCall()
    state.data = result
  } catch (error) {
    state.error = error.message
    
    // 通知Store経由でユーザーに通知
    const notificationStore = useNotificationStore()
    notificationStore.error('データ取得エラー', error.message)
  } finally {
    state.loading = false
  }
}
```

## Vue 3 + Composition API パターン

### 基本構造
```vue
<template lang="pug">
.component-name
  .loading(v-if="loading") 読み込み中...
  .error(v-if="error") {{ error }}
  .content(v-else)
    // メインコンテンツ
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useExampleStore } from '~/stores/exampleStore'

// Props定義
interface Props {
  required: string
  optional?: number
}

const props = withDefaults(defineProps<Props>(), {
  optional: 0
})

// Emits定義
interface Emits {
  update: [value: string]
  error: [error: Error]
}

const emit = defineEmits<Emits>()

// Store使用
const store = useExampleStore()

// Local state
const localData = ref<string>('')
const isActive = ref(false)

// Computed
const displayValue = computed(() => {
  return `${props.required}: ${localData.value}`
})

// Methods
const handleUpdate = (value: string) => {
  localData.value = value
  emit('update', value)
}

// Lifecycle
onMounted(async () => {
  try {
    await store.fetchData()
  } catch (error) {
    emit('error', error as Error)
  }
})

onUnmounted(() => {
  // クリーンアップ処理
})

// Expose（テスト用）
defineExpose({
  handleUpdate,
  localData
})
</script>
```

### リアクティブ性の注意点
```typescript
// ❌ 間違った例：リアクティブ性を失う
const badState = {
  data: ref([])
}

// ✅ 正しい例：reactive使用
const goodState = reactive({
  data: []
})

// ❌ 間違った例：分割代入でリアクティブ性を失う
const { data } = store // リアクティブ性を失う

// ✅ 正しい例：toRefsまたは直接参照
const { data } = toRefs(store) // リアクティブ性を保持
const data = computed(() => store.data) // または直接computed
```

## Tauriコマンド実装パターン

### Rust側コマンド定義
```rust
// src-tauri/src/main.rs
use tauri::Manager;

#[tauri::command]
async fn example_command(
    param: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // パラメータ検証
    if param.is_empty() {
        return Err("パラメータが空です".to_string());
    }
    
    // ビジネスロジック
    match perform_operation(&param).await {
        Ok(result) => Ok(result),
        Err(e) => {
            // ログ出力
            log::error!("操作失敗: {}", e);
            Err(format!("操作に失敗しました: {}", e))
        }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            example_command
        ])
        .run(tauri::generate_context!())
        .expect("Tauriアプリケーションの実行に失敗しました");
}
```

### フロントエンド側呼び出し
```typescript
// plugins/tauri.ts
import { invoke } from '@tauri-apps/api/tauri'

export class TauriAPI {
  static async exampleCommand(param: string): Promise<string> {
    try {
      const result = await invoke<string>('example_command', { param })
      return result
    } catch (error) {
      console.error('Tauriコマンドエラー:', error)
      throw new Error(`コマンド実行エラー: ${error}`)
    }
  }
}

// Store内での使用例
const executeCommand = async (param: string) => {
  state.loading = true
  try {
    const result = await TauriAPI.exampleCommand(param)
    state.result = result
  } catch (error) {
    state.error = error.message
    notificationStore.error('コマンド実行エラー', error.message)
  } finally {
    state.loading = false
  }
}
```

## エラーハンドリングベストプラクティス

### 階層化エラー処理
```typescript
// 1. Component レベル
const handleComponentError = (error: Error) => {
  console.error('コンポーネントエラー:', error)
  emit('error', error)
}

// 2. Store レベル
const handleStoreError = (error: Error, context: string) => {
  console.error(`ストアエラー(${context}):`, error)
  
  const notificationStore = useNotificationStore()
  notificationStore.error(`${context}エラー`, error.message)
}

// 3. Global レベル (app.vue)
onErrorCaptured((error, instance, info) => {
  console.error('グローバルエラー:', error)
  
  notificationStore.error(
    'アプリケーションエラー',
    '予期しないエラーが発生しました。',
    {
      duration: 0,
      actions: [
        {
          label: 'リロード',
          type: 'primary',
          handler: () => window.location.reload()
        }
      ]
    }
  )
  
  return false // エラー伝播を停止
})
```

## テスト実装パターン

### Vue コンポーネントテスト
```typescript
// tests/components/ExampleComponent.spec.ts
import { mount } from '@vue/test-utils'
import { createPinia, setActivePinia } from 'pinia'
import ExampleComponent from '~/components/ExampleComponent.vue'

describe('ExampleComponent', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('プロパティが正しく表示される', () => {
    const wrapper = mount(ExampleComponent, {
      props: {
        required: 'test-value'
      }
    })

    expect(wrapper.text()).toContain('test-value')
  })

  it('イベントが正しく発行される', async () => {
    const wrapper = mount(ExampleComponent, {
      props: {
        required: 'test-value'
      }
    })

    await wrapper.find('button').trigger('click')
    
    expect(wrapper.emitted('update')).toBeTruthy()
    expect(wrapper.emitted('update')[0]).toEqual(['new-value'])
  })
})
```

### Store テスト
```typescript
// tests/stores/exampleStore.spec.ts
import { setActivePinia, createPinia } from 'pinia'
import { useExampleStore } from '~/stores/exampleStore'

describe('ExampleStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('初期状態が正しい', () => {
    const store = useExampleStore()
    
    expect(store.data).toEqual([])
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
  })

  it('データ取得が成功する', async () => {
    const store = useExampleStore()
    
    // モック設定
    vi.mocked(apiCall).mockResolvedValue(['item1', 'item2'])
    
    await store.fetchData()
    
    expect(store.data).toEqual(['item1', 'item2'])
    expect(store.loading).toBe(false)
    expect(store.error).toBeNull()
  })
})
```

## パフォーマンス最適化

### 仮想スクロール実装
```vue
<template lang="pug">
.virtual-scroll-container(
  ref="container"
  @scroll="handleScroll"
  :style="{ height: containerHeight + 'px' }"
)
  .virtual-scroll-spacer(:style="{ height: topSpacerHeight + 'px' }")
  .virtual-scroll-item(
    v-for="(item, index) in visibleItems"
    :key="item.id"
    :style="{ height: itemHeight + 'px' }"
  )
    slot(:item="item" :index="startIndex + index")
  .virtual-scroll-spacer(:style="{ height: bottomSpacerHeight + 'px' }")
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Props {
  items: any[]
  itemHeight: number
  containerHeight: number
}

const props = defineProps<Props>()

const container = ref<HTMLElement>()
const scrollTop = ref(0)

const visibleCount = computed(() => 
  Math.ceil(props.containerHeight / props.itemHeight) + 2
)

const startIndex = computed(() => 
  Math.floor(scrollTop.value / props.itemHeight)
)

const endIndex = computed(() => 
  Math.min(startIndex.value + visibleCount.value, props.items.length)
)

const visibleItems = computed(() => 
  props.items.slice(startIndex.value, endIndex.value)
)

const topSpacerHeight = computed(() => 
  startIndex.value * props.itemHeight
)

const bottomSpacerHeight = computed(() => 
  (props.items.length - endIndex.value) * props.itemHeight
)

const handleScroll = (event: Event) => {
  const target = event.target as HTMLElement
  scrollTop.value = target.scrollTop
}
</script>
```

## セキュリティ実装

### XSS対策
```vue
<template lang="pug">
// ❌ 危険：v-html使用
.content(v-html="userInput")

// ✅ 安全：テキストとして表示
.content {{ userInput }}

// ✅ 安全：サニタイズ後のHTML
.content(v-html="sanitizedContent")
</template>

<script setup lang="ts">
import DOMPurify from 'dompurify'

const sanitizedContent = computed(() => {
  return DOMPurify.sanitize(props.htmlContent)
})
</script>
```

### 認証情報の安全な扱い
```typescript
// ❌ 危険：平文でログ出力
console.log('API Key:', apiKey)

// ✅ 安全：マスク処理
console.log('API Key:', apiKey.replace(/.(?=.{4})/g, '*'))

// ❌ 危険：ローカルストレージに平文保存
localStorage.setItem('apiKey', apiKey)

// ✅ 安全：Tauriコマンドで暗号化保存
await invoke('save_encrypted_credential', { key: 'apiKey', value: apiKey })
```

このガイドを参考に、一貫性のある安全で保守可能なコードを実装してください。