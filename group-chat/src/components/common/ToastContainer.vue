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
    };
  },
  methods: {
    addToast(options) {
      // 单例模式：清除所有现有的toast
      this.clearAll();

      const toast = {
        id: this.nextId++,
        message: options.message || '',
        type: options.type || 'info',
        duration: options.duration || 1000, // 默认1秒
        visible: true,
      };

      this.toasts.push(toast);

      return toast.id;
    },

    removeToast(id) {
      const index = this.toasts.findIndex((toast) => toast.id === id);
      if (index > -1) {
        this.toasts[index].visible = false;
        // 等待动画完成后移除
        setTimeout(() => {
          const currentIndex = this.toasts.findIndex(
            (toast) => toast.id === id
          );
          if (currentIndex > -1) {
            this.toasts.splice(currentIndex, 1);
          }
        }, 300);
      }
    },

    clearAll() {
      // 立即清除所有toast，不等待动画
      this.toasts = [];
    },
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
