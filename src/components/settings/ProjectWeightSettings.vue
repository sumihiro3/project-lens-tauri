<template lang="pug">
.project-weight-settings
  h3 プロジェクト重み設定
  //- プロジェクト重み設定コンポーネント
  p プロジェクトごとの重要度設定
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ProjectWeightSettings',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.project-weight-settings {
  margin-bottom: 24px;
}
</style>