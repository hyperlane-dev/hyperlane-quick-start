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
      const toast = {
        id: this.nextId++,
        message: options.message || '',
        type: options.type || 'info',
        duration: options.duration || 3000,
        visible: true,
      };

      this.toasts.push(toast);

      // 限制最多显示5个Toast
      if (this.toasts.length > 5) {
        this.removeToast(this.toasts[0].id);
      }

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
      this.toasts.forEach((toast) => {
        toast.visible = false;
      });
      setTimeout(() => {
        this.toasts = [];
      }, 300);
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
