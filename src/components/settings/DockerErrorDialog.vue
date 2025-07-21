<template lang="pug">
.docker-error-dialog
  h3 Dockerエラー
  //- Dockerエラーダイアログコンポーネント
  p Docker未検出時のエラー表示・インストールガイド
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'DockerErrorDialog',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.docker-error-dialog {
  padding: 16px;
  border: 1px solid #ff4d4f;
  border-radius: 8px;
  background-color: #fff1f0;
}
</style>