<template lang="pug">
.urgent-tasks
  h3 緊急対応セクション
  //- 緊急タスクの表示コンポーネント
  p 緊急対応が必要なタスクを表示します
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'UrgentTasksSection',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.urgent-tasks {
  margin-bottom: 20px;
  border-left: 4px solid #ff4d4f;
  padding-left: 16px;
}
</style>