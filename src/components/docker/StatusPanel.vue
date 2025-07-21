<template lang="pug">
.docker-status-panel
  v-card(variant="outlined")
    v-card-title Docker環境ステータス
    v-card-text
      v-row
        v-col(cols="12")
          v-alert(:type="dockerAvailable ? 'success' : 'error'", density="compact")
            | {{ dockerStatusText }}
        v-col(cols="12" v-if="dockerAvailable && dockerRunning")
          v-alert(:type="mcpServerRunning ? 'success' : 'warning'", density="compact")
            | {{ mcpServerStatusText }}
          .text-caption.mt-2(v-if="dockerVersion") Docker バージョン: {{ dockerVersion }}
      
      v-row(v-if="dockerAvailable && dockerRunning")
        v-col(cols="12")
          v-btn(
            color="primary"
            :loading="isLoading"
            :disabled="mcpServerRunning"
            @click="startMcpServer"
          )
            v-icon(start) mdi-play
            | MCP Serverを起動
          v-btn.ml-2(
            color="error"
            :loading="isLoading"
            :disabled="!mcpServerRunning"
            @click="stopMcpServer"
          )
            v-icon(start) mdi-stop
            | MCP Serverを停止
          v-btn.ml-2(
            color="info"
            :loading="isLoading"
            @click="refreshStatus"
          )
            v-icon(start) mdi-refresh
            | 状態を更新
      
      v-alert.mt-4(
        v-if="error"
        type="error"
        density="compact"
        closable
      ) {{ error }}
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia';
import { useDockerStore } from '~/stores/dockerStore';

// Dockerストア
const dockerStore = useDockerStore();

// ストアの状態をリアクティブに取得
const { 
  isDockerAvailable, 
  isDockerRunning, 
  dockerVersion, 
  isLoading, 
  error, 
  dockerStatusText, 
  mcpServerStatusText, 
  isMcpServerRunning 
} = storeToRefs(dockerStore);

// 算出プロパティ
const dockerAvailable = computed(() => isDockerAvailable.value);
const dockerRunning = computed(() => isDockerRunning.value);
const mcpServerRunning = computed(() => isMcpServerRunning.value);

// メソッド
async function startMcpServer() {
  await dockerStore.startMcpServer();
}

async function stopMcpServer() {
  await dockerStore.stopMcpServer();
}

async function refreshStatus() {
  await dockerStore.checkDockerAvailability();
  if (dockerAvailable.value) {
    await dockerStore.checkDockerRunning();
    if (dockerRunning.value) {
      await dockerStore.getDockerVersion();
      await dockerStore.checkMcpServerStatus();
    }
  }
}

// 初期化
onMounted(async () => {
  await refreshStatus();
});
</script>

<style scoped>
.docker-status-panel {
  margin-bottom: 1rem;
}
</style>