<template lang="pug">
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
                rel="noopener noreferrer"
              )
                Icon(name="ph:download")
                span {{ link.label }}
      
      .diagnostic-info(v-if="diagnosticInfo")
        h4 診断情報
        .diagnostic-details
          .diagnostic-item(v-for="item in diagnosticInfo" :key="item.label")
            span.label {{ item.label }}:
            span.value(:class="item.status") {{ item.value }}
    
    footer.dialog-footer
      .action-buttons
        button.btn.btn-secondary(@click="close" :disabled="isRetrying") キャンセル
        button.btn.btn-primary(@click="retry" :disabled="isRetrying")
          Icon(name="ph:arrow-clockwise" v-if="isRetrying")
          Icon(name="ph:play" v-else)
          span {{ isRetrying ? '確認中...' : '再試行' }}
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useDockerStore } from '~/stores/dockerStore'

interface Props {
  visible: boolean
  errorType: 'not-installed' | 'not-running' | 'connection-failed'
  errorMessage?: string
}

interface PlatformGuide {
  name: string
  steps: string[]
  links?: { label: string; url: string }[]
}

interface DiagnosticItem {
  label: string
  value: string
  status: 'success' | 'error' | 'warning'
}

const props = withDefaults(defineProps<Props>(), {
  errorMessage: 'Dockerに関するエラーが発生しました'
})

const emit = defineEmits<{
  close: []
  retry: []
}>()

const dockerStore = useDockerStore()
const isRetrying = ref(false)
const showInstallationGuide = ref(false)

const errorTitle = computed(() => {
  switch (props.errorType) {
    case 'not-installed':
      return 'Dockerが見つかりません'
    case 'not-running':
      return 'Dockerが実行されていません'
    case 'connection-failed':
      return 'Docker接続エラー'
    default:
      return 'Dockerエラー'
  }
})

const platformGuides = computed<PlatformGuide[]>(() => [
  {
    name: 'Windows',
    steps: [
      'Docker Desktop for Windowsをダウンロードしてください',
      'インストーラーを実行し、指示に従ってインストールを完了してください',
      'インストール後、Docker Desktopを起動してください',
      'システムトレイのDockerアイコンが緑色になるまで待機してください'
    ],
    links: [
      {
        label: 'Docker Desktop for Windows',
        url: 'https://docs.docker.com/desktop/windows/install/'
      }
    ]
  },
  {
    name: 'macOS',
    steps: [
      'Docker Desktop for Macをダウンロードしてください',
      'ダウンロードしたDockerアプリケーションをApplicationsフォルダにドラッグしてください',
      'ApplicationsフォルダからDockerを起動してください',
      'メニューバーのDockerアイコンが表示されるまで待機してください'
    ],
    links: [
      {
        label: 'Docker Desktop for Mac',
        url: 'https://docs.docker.com/desktop/mac/install/'
      }
    ]
  },
  {
    name: 'Linux',
    steps: [
      'パッケージマネージャーを更新してください: sudo apt update',
      'Docker Engineをインストールしてください: sudo apt install docker.io',
      'Dockerサービスを開始してください: sudo systemctl start docker',
      'ユーザーをdockerグループに追加してください: sudo usermod -aG docker $USER'
    ],
    links: [
      {
        label: 'Docker Engine for Linux',
        url: 'https://docs.docker.com/engine/install/'
      }
    ]
  }
])

const diagnosticInfo = computed<DiagnosticItem[]>(() => {
  const info: DiagnosticItem[] = []
  
  if (dockerStore.isDockerAvailable !== null) {
    info.push({
      label: 'Docker インストール状況',
      value: dockerStore.isDockerAvailable ? 'インストール済み' : '未インストール',
      status: dockerStore.isDockerAvailable ? 'success' : 'error'
    })
  }
  
  if (dockerStore.isDockerRunning !== null) {
    info.push({
      label: 'Docker 実行状況',
      value: dockerStore.isDockerRunning ? '実行中' : '停止中',
      status: dockerStore.isDockerRunning ? 'success' : 'error'
    })
  }
  
  if (dockerStore.dockerVersion) {
    info.push({
      label: 'Docker バージョン',
      value: dockerStore.dockerVersion,
      status: 'success'
    })
  }
  
  return info
})

const onOverlayClick = (event: MouseEvent) => {
  if (event.target === event.currentTarget) {
    close()
  }
}

const close = () => {
  emit('close')
}

const retry = async () => {
  if (isRetrying.value) return
  
  isRetrying.value = true
  try {
    // Docker環境の初期化を実行
    await dockerStore.initializeDockerEnvironment()
    
    // エラーが解決された場合はダイアログを閉じる
    if (dockerStore.isDockerAvailable && dockerStore.isDockerRunning) {
      close()
    }
  } catch (error) {
    console.error('Docker再試行中にエラーが発生しました:', error)
  } finally {
    isRetrying.value = false
  }
  
  emit('retry')
}

onMounted(() => {
  // インストールガイドの表示判定
  showInstallationGuide.value = props.errorType === 'not-installed'
})
</script>

<style scoped>
.docker-error-dialog {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  z-index: 1000;
}

.dialog-overlay {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  background-color: rgba(0, 0, 0, 0.5);
  backdrop-filter: blur(4px);
}

.dialog-content {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  overflow-y: auto;
  background: white;
  border-radius: 12px;
  box-shadow: 0 25px 50px rgba(0, 0, 0, 0.25);
}

.dialog-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 24px;
  border-bottom: 1px solid #f0f0f0;
}

.error-icon {
  font-size: 32px;
  color: #ff4d4f;
}

.dialog-title {
  flex: 1;
  margin: 0;
  font-size: 20px;
  font-weight: 600;
  color: #262626;
}

.close-button {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: #8c8c8c;
  cursor: pointer;
  transition: all 0.2s;
}

.close-button:hover {
  background: #f5f5f5;
  color: #262626;
}

.dialog-body {
  padding: 24px;
}

.error-message {
  margin-bottom: 24px;
  padding: 16px;
  background: #fff1f0;
  border: 1px solid #ffccc7;
  border-radius: 8px;
  color: #cf1322;
}

.error-message p {
  margin: 0;
  line-height: 1.5;
}

.installation-guide {
  margin-bottom: 24px;
}

.installation-guide h3 {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #262626;
}

.platform-guides {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.guide-section h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #595959;
}

.guide-section ol {
  margin: 0 0 12px 0;
  padding-left: 20px;
}

.guide-section li {
  margin-bottom: 8px;
  line-height: 1.5;
  color: #595959;
}

.download-links {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}

.download-link {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 12px;
  border: 1px solid #1890ff;
  border-radius: 6px;
  background: transparent;
  color: #1890ff;
  text-decoration: none;
  font-size: 14px;
  transition: all 0.2s;
}

.download-link:hover {
  background: #1890ff;
  color: white;
}

.diagnostic-info h4 {
  margin: 0 0 12px 0;
  font-size: 16px;
  font-weight: 600;
  color: #262626;
}

.diagnostic-details {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.diagnostic-item {
  display: flex;
  justify-content: space-between;
  padding: 8px 12px;
  background: #fafafa;
  border-radius: 6px;
}

.diagnostic-item .label {
  font-weight: 500;
  color: #595959;
}

.diagnostic-item .value {
  font-weight: 500;
}

.diagnostic-item .value.success {
  color: #52c41a;
}

.diagnostic-item .value.error {
  color: #ff4d4f;
}

.diagnostic-item .value.warning {
  color: #faad14;
}

.dialog-footer {
  padding: 16px 24px;
  border-top: 1px solid #f0f0f0;
  background: #fafafa;
  border-bottom-left-radius: 12px;
  border-bottom-right-radius: 12px;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.btn {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 8px 16px;
  border: 1px solid;
  border-radius: 6px;
  background: transparent;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
}

.btn:disabled {
  cursor: not-allowed;
  opacity: 0.6;
}

.btn-secondary {
  border-color: #d9d9d9;
  color: #595959;
}

.btn-secondary:hover:not(:disabled) {
  border-color: #40a9ff;
  color: #1890ff;
}

.btn-primary {
  border-color: #1890ff;
  background: #1890ff;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  border-color: #40a9ff;
  background: #40a9ff;
}

@media (max-width: 768px) {
  .dialog-content {
    width: 95%;
    margin: 20px;
  }
  
  .platform-guides {
    gap: 16px;
  }
  
  .action-buttons {
    flex-direction: column;
  }
  
  .btn {
    width: 100%;
    justify-content: center;
  }
}
</style>