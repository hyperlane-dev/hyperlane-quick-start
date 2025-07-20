<template>
  <div class="toast-container">
    <ToastNotification
      v-for="toast in toasts"
      :key="toast.id"
      :message="toast.message"
      :type="toast.type"
      :duration="toast.duration"
      :visible="toast.visible"
      @close="removeToast(toast.id)"
    />
  </div>
</template>

<script>
import ToastNotification from './Toast.vue';

export default {
  name: 'ToastContainer',
  components: {
    ToastNotification,
  },
  data() {
    return {
      toasts: [],
      nextId: 1,
      currentToastTimer: null, // 当前toast的定时器
    };
  },
  methods: {
    addToast(options) {
      // 清除当前的定时器
      if (this.currentToastTimer) {
        clearTimeout(this.currentToastTimer);
        this.currentToastTimer = null;
      }

      // 如果已有toast，更新内容；否则创建新的
      if (this.toasts.length > 0) {
        // 更新现有toast的内容
        this.toasts[0].message = options.message || '';
        this.toasts[0].type = options.type || 'info';
        this.toasts[0].duration = options.duration || 1000;
      } else {
        // 创建新toast
        const toast = {
          id: this.nextId++,
          message: options.message || '',
          type: options.type || 'info',
          duration: options.duration || 1000,
          visible: true,
        };
        this.toasts.push(toast);
      }

      const duration = options.duration || 1000;
      this.currentToastTimer = setTimeout(() => {
        this.clearAll();
        this.currentToastTimer = null;
      }, duration);

      return this.toasts[0].id;
    },

    removeToast() {
      // 清除定时器并移除toast
      if (this.currentToastTimer) {
        clearTimeout(this.currentToastTimer);
        this.currentToastTimer = null;
      }
      this.clearAll();
    },

    clearAll() {
      // 立即清除所有toast
      this.toasts = [];
      if (this.currentToastTimer) {
        clearTimeout(this.currentToastTimer);
        this.currentToastTimer = null;
      }
    },
  },
  beforeUnmount() {
    // 组件销毁时清除定时器
    if (this.currentToastTimer) {
      clearTimeout(this.currentToastTimer);
    }
  },
};
</script>

<style scoped>
.toast-container {
  position: fixed;
  top: 0;
  right: 0;
  z-index: 9999;
  pointer-events: none;
}

.toast-container > * {
  pointer-events: auto;
  margin-bottom: 10px;
}
</style>
