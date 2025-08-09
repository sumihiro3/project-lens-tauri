/**
 * Docker関連の状態管理ストア
 */
import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotificationStore } from './notificationStore';
import type { ContainerStatus, DockerResponse } from '~/types/docker';

/**
 * Dockerストア
 */
export const useDockerStore = defineStore('docker', () => {
  // 依存関係
  const notificationStore = useNotificationStore();
  
  // 状態
  const isDockerAvailable = ref<boolean | null>(null);
  const isDockerRunning = ref<boolean | null>(null);
  const dockerVersion = ref<string | null>(null);
  const mcpServerStatus = ref<ContainerStatus | null>(null);
  const isLoading = ref<boolean>(false);
  const error = ref<string | null>(null);
  const showErrorDialog = ref<boolean>(false);
  const errorDialogType = ref<'not-installed' | 'not-running' | 'connection-failed'>('not-installed');
  const isRetryMode = ref<boolean>(false);

  // 算出プロパティ
  const isMcpServerRunning = computed(() => mcpServerStatus.value?.isRunning || false);
  const dockerStatusText = computed(() => {
    if (isDockerAvailable.value === null) return '確認中...';
    if (!isDockerAvailable.value) return 'Dockerが見つかりません';
    if (isDockerRunning.value === null) return '状態確認中...';
    if (!isDockerRunning.value) return 'Dockerが実行されていません';
    return 'Docker実行中';
  });

  const mcpServerStatusText = computed(() => {
    if (!isDockerAvailable.value) return 'Docker未検出';
    if (!isDockerRunning.value) return 'Docker未起動';
    if (mcpServerStatus.value === null) return '確認中...';
    return mcpServerStatus.value.isRunning ? 'MCP Server実行中' : 'MCP Server停止中';
  });

  /**
   * Dockerの可用性を確認
   */
  async function checkDockerAvailability(): Promise<DockerResponse<boolean>> {
    isLoading.value = true;
    error.value = null;

    try {
      const available = await invoke<boolean>('check_docker_available');
      isDockerAvailable.value = available;
      
      if (!available && !isRetryMode.value) {
        handleDockerError('not-installed');
      }
      
      return { success: true, data: available };
    } catch (err) {
      const errorMessage = `Dockerの可用性確認中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      if (!isRetryMode.value) {
        handleDockerError('connection-failed', errorMessage);
      }
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * Dockerが実行中かどうかを確認
   */
  async function checkDockerRunning(): Promise<DockerResponse<boolean>> {
    if (!isDockerAvailable.value) {
      return { success: false, error: 'Dockerが利用できません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      const running = await invoke<boolean>('is_docker_running');
      isDockerRunning.value = running;
      
      if (!running && !isRetryMode.value) {
        handleDockerError('not-running');
      }
      
      return { success: true, data: running };
    } catch (err) {
      const errorMessage = `Docker実行状態の確認中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      if (!isRetryMode.value) {
        handleDockerError('connection-failed', errorMessage);
      }
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * Dockerのバージョン情報を取得
   */
  async function getDockerVersion(): Promise<DockerResponse<string>> {
    if (!isDockerAvailable.value) {
      return { success: false, error: 'Dockerが利用できません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      const version = await invoke<string>('get_docker_version');
      dockerVersion.value = version;
      return { success: true, data: version };
    } catch (err) {
      const errorMessage = `Dockerバージョンの取得中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * MCP Serverコンテナの状態を確認
   */
  async function checkMcpServerStatus(): Promise<DockerResponse<ContainerStatus>> {
    if (!isDockerAvailable.value || !isDockerRunning.value) {
      return { success: false, error: 'Dockerが利用できないか実行されていません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      const status = await invoke<ContainerStatus>('check_mcp_server_status');
      mcpServerStatus.value = status;
      return { success: true, data: status };
    } catch (err) {
      const errorMessage = `MCP Serverの状態確認中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * MCP Serverコンテナを起動
   */
  async function startMcpServer(): Promise<DockerResponse<void>> {
    if (!isDockerAvailable.value || !isDockerRunning.value) {
      return { success: false, error: 'Dockerが利用できないか実行されていません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      await invoke('start_mcp_server');
      // 起動後に状態を更新
      await checkMcpServerStatus();
      return { success: true };
    } catch (err) {
      const errorMessage = `MCP Serverの起動中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * MCP Serverコンテナを停止
   */
  async function stopMcpServer(): Promise<DockerResponse<void>> {
    if (!isDockerAvailable.value || !isDockerRunning.value) {
      return { success: false, error: 'Dockerが利用できないか実行されていません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      await invoke('stop_mcp_server');
      // 停止後に状態を更新
      await checkMcpServerStatus();
      return { success: true };
    } catch (err) {
      const errorMessage = `MCP Serverの停止中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * MCP Serverコンテナが存在するかどうかを確認
   */
  async function checkMcpServerExists(): Promise<DockerResponse<boolean>> {
    if (!isDockerAvailable.value || !isDockerRunning.value) {
      return { success: false, error: 'Dockerが利用できないか実行されていません' };
    }

    isLoading.value = true;
    error.value = null;

    try {
      const exists = await invoke<boolean>('check_mcp_server_exists');
      return { success: true, data: exists };
    } catch (err) {
      const errorMessage = `MCP Serverの存在確認中にエラーが発生しました: ${err}`;
      error.value = errorMessage;
      return { success: false, error: errorMessage };
    } finally {
      isLoading.value = false;
    }
  }

  /**
   * Docker環境の初期化（アプリ起動時に呼び出す）
   */
  async function initializeDockerEnvironment(): Promise<void> {
    // Dockerの可用性を確認
    const dockerAvailableResult = await checkDockerAvailability();
    if (!dockerAvailableResult.success || !dockerAvailableResult.data) {
      return;
    }

    // Dockerが実行中かどうかを確認
    const dockerRunningResult = await checkDockerRunning();
    if (!dockerRunningResult.success || !dockerRunningResult.data) {
      return;
    }

    // Dockerのバージョン情報を取得
    await getDockerVersion();

    // MCP Serverコンテナの状態を確認
    await checkMcpServerStatus();
  }

  /**
   * Dockerエラーを処理
   */
  function handleDockerError(errorType: 'not-installed' | 'not-running' | 'connection-failed', message?: string): void {
    errorDialogType.value = errorType;
    
    // 初回エラー時のみ通知を表示（ダイアログが既に表示されている場合は通知をスキップ）
    if (!showErrorDialog.value) {
      const notificationId = notificationStore.dockerError(errorType, message);
    }
    
    // エラーダイアログの表示を管理
    showErrorDialog.value = true;
  }

  /**
   * エラーダイアログを閉じる
   */
  function closeErrorDialog(): void {
    showErrorDialog.value = false;
  }

  /**
   * インストールガイド表示用（通知のボタンから呼び出される）
   */
  function showInstallationGuide(errorType: 'not-installed' | 'not-running' | 'connection-failed', message?: string): void {
    errorDialogType.value = errorType;
    // 通知は作成せず、ダイアログのみ表示
    showErrorDialog.value = true;
  }

  /**
   * Docker環境の再試行
   */
  async function retryDockerEnvironment(): Promise<boolean> {
    // 再試行モードを有効にして、個別チェック関数からの通知を抑制
    isRetryMode.value = true;
    
    try {
      await initializeDockerEnvironment();
      
      // エラーが解決された場合はダイアログを閉じる
      if (isDockerAvailable.value && isDockerRunning.value) {
        closeErrorDialog();
        notificationStore.success(
          'Docker接続成功',
          'Dockerとの接続が正常に確立されました。'
        );
        return true;
      }
      
      // エラーが継続している場合は通知を表示
      if (!isDockerAvailable.value) {
        notificationStore.error(
          'Docker再試行失敗',
          'Dockerが見つかりません。インストールを確認してください。'
        );
      } else if (!isDockerRunning.value) {
        notificationStore.error(
          'Docker再試行失敗',
          'Dockerが実行されていません。Docker Desktopを起動してください。'
        );
      }
      
      return false;
    } catch (error) {
      console.error('Docker再試行中にエラーが発生しました:', error);
      notificationStore.error(
        'Docker再試行エラー',
        'Docker環境の確認中にエラーが発生しました。'
      );
      return false;
    } finally {
      // 再試行モードを無効にする
      isRetryMode.value = false;
    }
  }

  // カスタムイベントリスナーの設定
  if (typeof window !== 'undefined') {
    window.addEventListener('show-docker-error-dialog', (event) => {
      const { errorType, message } = (event as CustomEvent).detail;
      showInstallationGuide(errorType, message);
    });
  }

  return {
    // 状態
    isDockerAvailable,
    isDockerRunning,
    dockerVersion,
    mcpServerStatus,
    isLoading,
    error,
    showErrorDialog,
    errorDialogType,

    // 算出プロパティ
    isMcpServerRunning,
    dockerStatusText,
    mcpServerStatusText,

    // アクション
    checkDockerAvailability,
    checkDockerRunning,
    getDockerVersion,
    checkMcpServerStatus,
    startMcpServer,
    stopMcpServer,
    checkMcpServerExists,
    initializeDockerEnvironment,
    
    // エラーハンドリング
    handleDockerError,
    closeErrorDialog,
    showInstallationGuide,
    retryDockerEnvironment,
  };
});