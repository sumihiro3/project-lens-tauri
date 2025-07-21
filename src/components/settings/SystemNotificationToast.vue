<template lang="pug">
.system-notification-toast
  //- システム通知トーストコンポーネント
  p システム状態変更の通知表示
</template>

<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'SystemNotificationToast',
  setup() {
    // コンポーネントのロジック
    return {}
  }
})
</script>

<style scoped>
.system-notification-toast {
  padding: 12px 16px;
  border-radius: 4px;
  background-color: #e6f7ff;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
  margin-bottom: 16px;
}
</style>