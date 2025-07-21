<template lang="pug">
.display-settings
  h3 表示設定
  //- 表示設定コンポーネント
  p 表示設定・テーマ
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'DisplaySettings',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.display-settings {
  margin-bottom: 24px;
}
</style>