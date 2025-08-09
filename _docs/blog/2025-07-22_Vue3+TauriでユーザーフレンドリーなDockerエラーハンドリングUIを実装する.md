# Vue 3 + TauriでユーザーフレンドリーなDockerエラーハンドリングUIを実装する

## 概要

デスクトップアプリケーションでDockerを活用する際、最も大きな課題の一つがエラーハンドリングです。Dockerがインストールされていない、実行されていない、または接続できない場合、技術的なエラーメッセージをそのまま表示してしまうと、エンドユーザーにとって理解しにくく、解決方法も分からない状況が生まれます。

この記事では、Vue 3とTauriを使用して、Docker関連のエラーを直感的で解決指向のUIで表現する方法を詳しく解説します。実装を通じて学んだベストプラクティスや設計判断も含めて共有します。

## 解決したい問題

- **技術的すぎるエラーメッセージ**: 「Docker daemon not found」のような開発者向けエラーメッセージ
- **解決手段の不明確さ**: エラーが表示されても、次に何をすべきか分からない
- **プラットフォーム差異の無視**: OS別の具体的なインストール手順の不足
- **状態の不透明性**: システムの現在状態がユーザーに伝わらない

## 技術スタック

- **Frontend**: Vue 3 (Composition API), TypeScript, Pug, SCSS
- **Desktop Framework**: Tauri
- **状態管理**: Pinia
- **UIコンポーネント**: 自作モーダル・トーストシステム
- **アイコン**: Phosphor Icons

## 実装アーキテクチャ

### 1. レイヤード設計

```
┌─────────────────────────────────────────┐
│ UI Layer (Components)                   │
│ ├─ DockerErrorDialog.vue                │
│ └─ SystemNotificationToast.vue          │
├─────────────────────────────────────────┤
│ State Management Layer (Pinia)          │
│ ├─ dockerStore.ts                       │
│ └─ notificationStore.ts                 │
├─────────────────────────────────────────┤
│ Integration Layer (Tauri)               │
│ └─ Rust Commands                        │
└─────────────────────────────────────────┘
```

### 2. 責任の分離

- **dockerStore**: Docker状態の管理とバックエンド連携
- **notificationStore**: 通知システムの管理
- **DockerErrorDialog**: 詳細なエラー情報とガイダンスの表示
- **SystemNotificationToast**: 軽量な状態通知

## 核心の実装内容

### 1. Docker状態管理ストア (dockerStore.ts)

```typescript
export const useDockerStore = defineStore('docker', () => {
  // 状態
  const isDockerAvailable = ref<boolean | null>(null);
  const isDockerRunning = ref<boolean | null>(null);
  const dockerVersion = ref<string | null>(null);
  const showErrorDialog = ref<boolean>(false);
  const errorDialogType = ref<'not-installed' | 'not-running' | 'connection-failed'>('not-installed');

  // 算出プロパティ
  const dockerStatusText = computed(() => {
    if (isDockerAvailable.value === null) return '確認中...';
    if (!isDockerAvailable.value) return 'Dockerが見つかりません';
    if (isDockerRunning.value === null) return '状態確認中...';
    if (!isDockerRunning.value) return 'Dockerが実行されていません';
    return 'Docker実行中';
  });

  // Docker可用性チェック
  async function checkDockerAvailability(): Promise<DockerResponse<boolean>> {
    isLoading.value = true;
    try {
      const result = await invoke<DockerResponse<boolean>>('check_docker_availability');
      isDockerAvailable.value = result.success;
      
      if (!result.success) {
        // エラーダイアログを表示
        errorDialogType.value = 'not-installed';
        showErrorDialog.value = true;
      }
      
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Unknown error';
      notificationStore.addError('Docker状態確認失敗', errorMessage);
      throw err;
    } finally {
      isLoading.value = false;
    }
  }
});
```

### 2. エラーダイアログコンポーネント (DockerErrorDialog.vue)

#### Template構造 (Pug)

```pug
.docker-error-dialog(v-if="visible")
  .dialog-overlay(@click="onOverlayClick")
  .dialog-content
    header.dialog-header
      .error-icon
        Icon(name="ph:warning-circle-fill")
      h2.dialog-title {{ errorTitle }}
      button.close-button(@click="close")
        Icon(name="ph:x")
    
    .dialog-body
      .error-message
        p {{ errorMessage }}
      
      .installation-guide(v-if="showInstallationGuide")
        h3 Dockerのインストール方法
        .platform-guides
          .guide-section(v-for="platform in platformGuides" :key="platform.name")
            h4 {{ platform.name }}
            ol
              li(v-for="step in platform.steps" :key="step") {{ step }}
            .download-links(v-if="platform.links")
              a.download-link(
                v-for="link in platform.links"
                :key="link.label"
                :href="link.url"
                target="_blank"
              )
                Icon(name="ph:download")
                span {{ link.label }}
```

#### Script Setup (TypeScript)

```typescript
// Props & Emits定義
interface Props {
  visible: boolean;
  errorType: 'not-installed' | 'not-running' | 'connection-failed';
  diagnosticInfo?: Array<{ label: string; value: string; status: string }>;
}

interface Emits {
  close: [];
  retry: [];
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();

// リアクティブな状態
const isRetrying = ref(false);

// 算出プロパティ
const errorTitle = computed(() => {
  switch (props.errorType) {
    case 'not-installed':
      return 'Dockerがインストールされていません';
    case 'not-running':
      return 'Dockerが実行されていません';
    case 'connection-failed':
      return 'Dockerに接続できません';
    default:
      return 'Docker関連のエラーが発生しました';
  }
});

const platformGuides = computed(() => [
  {
    name: 'Windows',
    steps: [
      'Docker Desktop for Windowsをダウンロード',
      'インストーラーを実行して画面に従ってインストール',
      'インストール後、システムを再起動',
      'Docker Desktopを起動して初期設定を完了'
    ],
    links: [
      {
        label: 'Docker Desktop for Windows',
        url: 'https://docs.docker.com/desktop/install/windows-install/'
      }
    ]
  },
  {
    name: 'macOS',
    steps: [
      'Docker Desktop for Macをダウンロード',
      '.dmgファイルをマウントしてDocker.appをApplicationsフォルダにドラッグ',
      'Applicationsフォルダから Docker Desktop を起動',
      '初期設定ウィザードに従って設定を完了'
    ],
    links: [
      {
        label: 'Docker Desktop for Mac',
        url: 'https://docs.docker.com/desktop/install/mac-install/'
      }
    ]
  },
  // Linux用の設定も含む...
]);

// リトライ機能
async function retry() {
  isRetrying.value = true;
  try {
    emit('retry');
    // リトライ成功時は自動的にダイアログが閉じられる
  } finally {
    setTimeout(() => {
      isRetrying.value = false;
    }, 1000);
  }
}
```

### 3. 通知システムの統合

```typescript
export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info' | 'loading';
  title: string;
  message?: string;
  duration?: number;
  dismissible?: boolean;
  actions?: NotificationAction[];
  createdAt: Date;
}

export const useNotificationStore = defineStore('notification', () => {
  const notifications = ref<Notification[]>([]);

  function addError(title: string, message?: string, options?: Partial<CreateNotificationOptions>) {
    return add({
      type: 'error',
      title,
      message,
      duration: 0, // エラーは手動で閉じる
      dismissible: true,
      ...options
    });
  }

  function add(options: CreateNotificationOptions): string {
    const notification: Notification = {
      id: generateId(),
      createdAt: new Date(),
      duration: 4000,
      dismissible: true,
      ...options
    };

    notifications.value.push(notification);
    
    // 自動削除タイマー設定
    if (notification.duration && notification.duration > 0) {
      notification.timerId = setTimeout(() => {
        remove(notification.id);
      }, notification.duration);
    }

    return notification.id;
  }
});
```

## 設計における重要な判断

### 1. 段階的エラー情報提示

エラー情報を一度に全て表示するのではなく、ユーザーのニーズに応じて段階的に提示する設計を採用しました。

```typescript
// 基本エラー表示 → 詳細情報 → 解決ガイド
const showInstallationGuide = computed(() => 
  props.errorType === 'not-installed' || props.errorType === 'not-running'
);
```

### 2. アクション指向のUI

エラーメッセージと同時に、具体的な解決手段を提供する設計にしました。

```pug
footer.dialog-footer
  .action-buttons
    button.btn.btn-secondary(@click="close") キャンセル
    button.btn.btn-primary(@click="retry")
      Icon(name="ph:arrow-clockwise" v-if="isRetrying")
      Icon(name="ph:play" v-else)
      span {{ isRetrying ? '確認中...' : '再試行' }}
```

### 3. プラットフォーム適応設計

ユーザーの環境に応じて適切な情報を表示するため、データ駆動のアプローチを採用しました。

```typescript
const platformGuides = computed(() => {
  // 現在のプラットフォームを検出して適切なガイドを返す
  const platform = getCurrentPlatform();
  return platforms.filter(p => p.supported.includes(platform));
});
```

## 学んだベストプラクティス

### 1. Vue 3 Composition APIの活用

**Props/Emitsの型安全な定義**:

```typescript
// 従来の書き方
export default defineComponent({
  props: {
    visible: Boolean,
    errorType: String
  },
  emits: ['close', 'retry']
})

// Composition APIでの改善版
interface Props {
  visible: boolean;
  errorType: 'not-installed' | 'not-running' | 'connection-failed';
}

interface Emits {
  close: [];
  retry: [];
}

const props = defineProps<Props>();
const emit = defineEmits<Emits>();
```

### 2. 状態管理の分離パターン

複数のストア間で適切に責任を分離し、疎結合な関係を維持しました。

```typescript
// dockerStore内での通知ストアの利用
export const useDockerStore = defineStore('docker', () => {
  const notificationStore = useNotificationStore();
  
  async function checkDockerAvailability() {
    try {
      // Docker処理...
    } catch (err) {
      // 通知ストアに委譲
      notificationStore.addError('Docker確認失敗', err.message);
    }
  }
});
```

### 3. アクセシビリティ対応

キーボードナビゲーションとフォーカス管理を実装しました。

```typescript
// モーダルオープン時のフォーカス管理
onMounted(() => {
  if (props.visible) {
    document.addEventListener('keydown', handleKeyDown);
    // 最初のフォーカス可能要素にフォーカス
    nextTick(() => {
      const firstFocusable = dialogContent.value?.querySelector('button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])');
      if (firstFocusable instanceof HTMLElement) {
        firstFocusable.focus();
      }
    });
  }
});

function handleKeyDown(event: KeyboardEvent) {
  if (event.key === 'Escape') {
    close();
  }
}
```

## 遭遇した課題と解決策

### 1. z-indexの競合問題

**問題**: 他のUIコンポーネントとのレイヤー競合
**解決策**: スタッキングコンテキストの理解とCSS変数による統一管理

```scss
:root {
  --z-index-modal: 1000;
  --z-index-toast: 1100;
  --z-index-tooltip: 1200;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
  z-index: var(--z-index-modal);
}
```

### 2. 非同期状態の可視化

**問題**: リトライ処理中の状態をユーザーに適切に伝達する必要
**解決策**: ローディングオーバーレイとアニメーション

```pug
footer.dialog-footer
  .loading-overlay(v-if="isRetrying")
    .loading-spinner
      Icon(name="ph:spinner")
    .loading-text Docker環境を確認しています...
  .action-buttons(:class="{ 'action-buttons--hidden': isRetrying }")
```

```scss
.loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.9);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-direction: column;
  
  .loading-spinner {
    animation: spin 1s linear infinite;
  }
}
```

## パフォーマンス最適化

### 1. コンポーネントの遅延ローディング

```typescript
// エラーダイアログは必要時のみロード
const DockerErrorDialog = defineAsyncComponent(() => 
  import('@/components/settings/DockerErrorDialog.vue')
);
```

### 2. メモ化の活用

```typescript
const platformGuides = computed(() => {
  // 重い計算をキャッシュ
  return memoizedPlatformGuides.value;
});
```

## 今後の拡張可能性

### 1. 多言語化対応

```typescript
// i18nとの統合例
const errorTitle = computed(() => {
  const key = `docker.errors.${props.errorType}.title`;
  return t(key);
});
```

### 2. 詳細診断機能

```typescript
interface DiagnosticInfo {
  dockerVersion?: string;
  containerCount?: number;
  runningContainers?: string[];
  memoryUsage?: string;
  diskSpace?: string;
}
```

### 3. ユーザー設定による表示カスタマイズ

```typescript
interface UserPreferences {
  errorDialogLevel: 'basic' | 'detailed' | 'advanced';
  autoRetryEnabled: boolean;
  showTechnicalDetails: boolean;
}
```

## まとめ

Vue 3 + Tauriを使ったDockerエラーハンドリングUIの実装を通じて、以下の重要な学びを得ました：

### 技術的学び
- **Composition API**による型安全で保守性の高いコンポーネント設計
- **Pinia**を使った適切な状態管理とストア間の責任分離
- **Tauri**でのフロントエンド・バックエンド統合における型安全性の確保

### UX設計の学び
- エラーハンドリングをユーザー体験の一部として設計することの重要性
- 技術的情報を段階的に提示するProgressive Enhancement
- 具体的な解決手段を提供するアクション指向の設計

### アーキテクチャの学び
- レイヤード設計による関心の分離
- 再利用可能なコンポーネント設計パターン
- エラー境界の適切な設定と管理

この実装アプローチは、Dockerに限らず、他の外部依存関係（データベース、API、開発ツールなど）のエラーハンドリングにも応用可能です。特に、デスクトップアプリケーションにおけるシステム依存の問題を解決する際の参考になるでしょう。

エラーハンドリングは単なる技術的な問題解決ではなく、ユーザーとシステムをつなぐ重要な接点です。適切に設計されたエラーハンドリングUIは、ユーザーの信頼感を高め、アプリケーション全体の品質を向上させる重要な要素となります。

---

## 関連リソース

- [Vue 3 Composition API ドキュメント](https://vuejs.org/guide/extras/composition-api-faq.html)
- [Tauri コマンド統合ガイド](https://tauri.app/v1/guides/features/command)  
- [Pinia 状態管理パターン](https://pinia.vuejs.org/core-concepts/state.html)
- [Docker インストールガイド](https://docs.docker.com/get-docker/)

**実装ファイル**:
- `/src/components/settings/DockerErrorDialog.vue` (525行)
- `/src/components/common/SystemNotificationToast.vue` (406行)
- `/src/stores/dockerStore.ts` (Docker状態管理)
- `/src/stores/notificationStore.ts` (通知管理)