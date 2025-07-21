<template lang="pug">
.project-overview
  h2 プロジェクト概要
  //- プロジェクト概要の表示コンポーネント
  p 複数プロジェクトの概要を表示します
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'ProjectOverview',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.project-overview {
  margin-bottom: 20px;
}
</style>