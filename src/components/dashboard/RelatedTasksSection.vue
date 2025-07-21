<template lang="pug">
.related-tasks
  h3 関連タスク
  //- 関連タスクの表示コンポーネント
  p 自分が関連するタスクを表示します
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'RelatedTasksSection',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.related-tasks {
  margin-bottom: 20px;
  border-left: 4px solid #1890ff;
  padding-left: 16px;
}
</style>