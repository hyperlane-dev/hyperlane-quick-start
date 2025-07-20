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
  background-color: #ffffff;
  color: #2c3e50;
  border: 1px solid #dee2e6;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  z-index: 30;
  transition: all 0.2s ease;
}

.scroll-to-bottom:hover {
  background-color: #f8f9fa;
  transform: translateY(-2px);
  box-shadow: 0 4px 15px rgba(0, 0, 0, 0.15);
}

.scroll-icon {
  font-style: normal;
  font-size: 1.2em;
}

.unread-count {
  position: absolute;
  top: -8px;
  right: -8px;
  background-color: #dc3545;
  color: white;
  font-size: 0.75rem;
  min-width: 18px;
  height: 18px;
  border-radius: 9px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 700;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}
</style>
