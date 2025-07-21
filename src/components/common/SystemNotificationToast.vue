<template lang="pug">
Teleport(to="body")
  .notification-container(v-if="notifications.length > 0")
    TransitionGroup(
      name="notification"
      tag="div"
      class="notification-list"
    )
      .notification-toast(
        v-for="notification in notifications"
        :key="notification.id"
        :class="[
          `notification--${notification.type}`,
          { 'notification--dismissible': notification.dismissible }
        ]"
        @click="handleToastClick(notification)"
      )
        .notification-content
          .notification-icon
            Icon(:name="getIconName(notification.type)")
          .notification-text
            .notification-title {{ notification.title }}
            .notification-message(v-if="notification.message") {{ notification.message }}
        
        .notification-actions(v-if="notification.actions && notification.actions.length > 0")
          button.action-btn(
            v-for="action in notification.actions"
            :key="action.label"
            :class="[`action-btn--${action.type || 'default'}`]"
            @click.stop="handleActionClick(notification, action)"
          ) {{ action.label }}
        
        button.close-btn(
          v-if="notification.dismissible"
          @click.stop="dismiss(notification.id)"
          :aria-label="'通知を閉じる'"
        )
          Icon(name="ph:x")
        
        .notification-progress(
          v-if="notification.duration && notification.duration > 0"
          :class="{ 'notification-progress--paused': notification.paused }"
          :style="{ animationDuration: `${notification.duration}ms` }"
        )
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useNotificationStore } from '~/stores/notificationStore'

const notificationStore = useNotificationStore()
const notifications = notificationStore.notifications

// アイコンマップ
const getIconName = (type: string): string => {
  const iconMap: Record<string, string> = {
    success: 'ph:check-circle-fill',
    error: 'ph:x-circle-fill',
    warning: 'ph:warning-circle-fill',
    info: 'ph:info-fill',
    loading: 'ph:spinner'
  }
  return iconMap[type] || iconMap.info
}

// トースト全体のクリック処理
const handleToastClick = (notification: any) => {
  if (notification.dismissible && !notification.actions?.length) {
    dismiss(notification.id)
  }
}

// アクションボタンのクリック処理
const handleActionClick = (notification: any, action: any) => {
  if (action.handler) {
    action.handler()
  }
  
  if (action.dismissOnClick !== false) {
    dismiss(notification.id)
  }
}

// 通知の削除
const dismiss = (id: string) => {
  notificationStore.dismiss(id)
}

// キーボードイベントの処理
const handleKeydown = (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    // Escapeキーで最新の通知を閉じる
    const latestNotification = notifications.value[0]
    if (latestNotification?.dismissible) {
      dismiss(latestNotification.id)
    }
  }
}

onMounted(() => {
  document.addEventListener('keydown', handleKeydown)
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
})
</script>

<style scoped>
.notification-container {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  pointer-events: none;
}

.notification-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
  max-width: 400px;
}

.notification-toast {
  position: relative;
  padding: 16px;
  border-radius: 12px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.12);
  backdrop-filter: blur(16px);
  pointer-events: auto;
  overflow: hidden;
  min-width: 320px;
  cursor: default;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.notification-toast:hover {
  transform: translateY(-2px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.16);
}

.notification-toast--dismissible {
  cursor: pointer;
}

/* 通知タイプ別スタイル */
.notification--success {
  background: linear-gradient(135deg, rgba(76, 175, 80, 0.95), rgba(129, 199, 132, 0.9));
  color: white;
  border: 1px solid rgba(76, 175, 80, 0.3);
}

.notification--error {
  background: linear-gradient(135deg, rgba(244, 67, 54, 0.95), rgba(239, 83, 80, 0.9));
  color: white;
  border: 1px solid rgba(244, 67, 54, 0.3);
}

.notification--warning {
  background: linear-gradient(135deg, rgba(255, 193, 7, 0.95), rgba(255, 206, 84, 0.9));
  color: rgba(0, 0, 0, 0.87);
  border: 1px solid rgba(255, 193, 7, 0.3);
}

.notification--info {
  background: linear-gradient(135deg, rgba(33, 150, 243, 0.95), rgba(100, 181, 246, 0.9));
  color: white;
  border: 1px solid rgba(33, 150, 243, 0.3);
}

.notification--loading {
  background: linear-gradient(135deg, rgba(158, 158, 158, 0.95), rgba(189, 189, 189, 0.9));
  color: white;
  border: 1px solid rgba(158, 158, 158, 0.3);
}

.notification-content {
  display: flex;
  align-items: flex-start;
  gap: 12px;
  margin-bottom: 12px;
}

.notification-content:last-child {
  margin-bottom: 0;
}

.notification-icon {
  font-size: 24px;
  flex-shrink: 0;
  margin-top: 2px;
}

.notification--loading .notification-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.notification-text {
  flex: 1;
  min-width: 0;
}

.notification-title {
  font-size: 16px;
  font-weight: 600;
  line-height: 1.4;
  margin-bottom: 4px;
}

.notification-message {
  font-size: 14px;
  line-height: 1.5;
  opacity: 0.9;
}

.notification-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
  flex-wrap: wrap;
}

.action-btn {
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  background: rgba(255, 255, 255, 0.2);
  color: currentColor;
}

.action-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  transform: translateY(-1px);
}

.action-btn--primary {
  background: rgba(255, 255, 255, 0.9);
  color: #1976d2;
}

.action-btn--primary:hover {
  background: white;
}

.notification--warning .action-btn {
  background: rgba(0, 0, 0, 0.1);
}

.notification--warning .action-btn:hover {
  background: rgba(0, 0, 0, 0.2);
}

.notification--warning .action-btn--primary {
  background: rgba(0, 0, 0, 0.8);
  color: white;
}

.close-btn {
  position: absolute;
  top: 12px;
  right: 12px;
  width: 24px;
  height: 24px;
  border: none;
  border-radius: 50%;
  background: rgba(255, 255, 255, 0.2);
  color: currentColor;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 14px;
  transition: all 0.2s;
  opacity: 0.7;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.3);
  opacity: 1;
  transform: scale(1.1);
}

.notification--warning .close-btn {
  background: rgba(0, 0, 0, 0.1);
}

.notification--warning .close-btn:hover {
  background: rgba(0, 0, 0, 0.2);
}

.notification-progress {
  position: absolute;
  bottom: 0;
  left: 0;
  height: 3px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 0 0 12px 12px;
  animation: notification-countdown linear forwards;
  transform-origin: left center;
}

.notification--warning .notification-progress {
  background: rgba(0, 0, 0, 0.3);
}

.notification-progress--paused {
  animation-play-state: paused;
}

@keyframes notification-countdown {
  from { transform: scaleX(1); }
  to { transform: scaleX(0); }
}

/* アニメーション */
.notification-enter-active {
  transition: all 0.4s cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.notification-leave-active {
  transition: all 0.3s cubic-bezier(0.55, 0, 0.1, 1);
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%) scale(0.9);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%) scale(0.9);
}

.notification-move {
  transition: transform 0.3s cubic-bezier(0.55, 0, 0.1, 1);
}

/* モバイル対応 */
@media (max-width: 768px) {
  .notification-container {
    top: 10px;
    right: 10px;
    left: 10px;
  }
  
  .notification-list {
    max-width: none;
  }
  
  .notification-toast {
    min-width: unset;
    margin: 0 10px;
  }
  
  .notification-actions {
    flex-direction: column;
  }
  
  .action-btn {
    width: 100%;
    justify-content: center;
  }
}

/* アクセシビリティ */
@media (prefers-reduced-motion: reduce) {
  .notification-toast,
  .action-btn,
  .close-btn,
  .notification-enter-active,
  .notification-leave-active,
  .notification-move {
    transition: none;
  }
  
  .notification--loading .notification-icon {
    animation: none;
  }
  
  .notification-progress {
    animation: none;
    transform: scaleX(0);
  }
}

/* ダークモード対応 */
@media (prefers-color-scheme: dark) {
  .notification-toast {
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
  }
  
  .notification-toast:hover {
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.5);
  }
}
</style>