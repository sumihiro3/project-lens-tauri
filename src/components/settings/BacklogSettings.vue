<template lang="pug">
.backlog-settings
  h3 Backlog設定
  //- Backlog設定コンポーネント
  p 複数Backlogワークスペース設定
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'BacklogSettings',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.backlog-settings {
  margin-bottom: 24px;
}
</style>