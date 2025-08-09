<!--
  プロジェクト概要表示コンポーネント
  複数のBacklogプロジェクトの概要情報をダッシュボードに表示する
-->
<template lang="pug">
.project-overview
  h2 プロジェクト概要
  //- プロジェクト概要の表示コンポーネント
  p 複数プロジェクトの概要を表示します
</template>

<script lang="ts">
import { defineComponent } from 'vue'

/**
 * プロジェクト概要表示コンポーネント
 * 
 * 機能:
 * - 複数のBacklogプロジェクトの概要情報を表示
 * - プロジェクト別のチケット数、進捗状況の表示
 * - プロジェクト重み付けに基づく優先度表示
 */
export default defineComponent({
  name: 'ProjectOverview',
  
  /**
   * コンポーネントのセットアップ関数
   * @returns リアクティブデータとメソッドを含むオブジェクト
   */
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