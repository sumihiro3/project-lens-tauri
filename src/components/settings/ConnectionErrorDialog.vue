<template lang="pug">
.connection-error-dialog
  h3 接続エラー
  //- 接続エラーダイアログコンポーネント
  p MCP Server接続エラー表示・再接続オプション
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ConnectionErrorDialog',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.connection-error-dialog {
  padding: 16px;
  border: 1px solid #faad14;
  border-radius: 8px;
  background-color: #fffbe6;
}
</style>