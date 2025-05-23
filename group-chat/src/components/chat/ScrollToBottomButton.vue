<template>
  <div class="scroll-to-bottom" @click="$emit('click')">
    <span v-if="unreadCount > 0" class="unread-count">{{ unreadCount }}</span>
    <i class="scroll-icon">↓</i>
  </div>
</template>

<script>
export default {
  name: 'ScrollToBottomButton',
  props: {
    unreadCount: {
      type: Number,
      default: 0,
    },
  },
};
</script>

<style scoped>
.scroll-to-bottom {
  position: fixed;
  bottom: 80px;
  right: 20px;
  width: 40px;
  height: 40px;
  background: linear-gradient(135deg, #4a9c5e 0%, #2e7d32 100%);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  z-index: 30;
  transition: all 0.3s ease;
  animation: bounce 1s infinite alternate;
}

.scroll-to-bottom:hover {
  transform: translateY(-3px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
}

.scroll-icon {
  font-style: normal;
  font-size: 1.2em;
}

.unread-count {
  position: absolute;
  top: -8px;
  right: -8px;
  background-color: #ff4d4f;
  color: white;
  font-size: 0.7em;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: bold;
  box-shadow: 0 2px 5px rgba(0, 0, 0, 0.2);
}

@keyframes bounce {
  from {
    transform: translateY(0);
  }
  to {
    transform: translateY(-5px);
  }
}
</style>
