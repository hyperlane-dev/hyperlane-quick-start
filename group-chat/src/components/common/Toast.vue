<template>
  <transition name="toast">
    <div v-if="visible" :class="['toast', `toast-${type}`]">
      <div class="toast-content">
        <i :class="iconClass"></i>
        <span class="toast-message">{{ message }}</span>
      </div>
    </div>
  </transition>
</template>

<script>
export default {
  name: 'ToastNotification',
  props: {
    message: {
      type: String,
      required: true,
    },
    type: {
      type: String,
      default: 'info', // info, success, warning, error
      validator: (value) =>
        ['info', 'success', 'warning', 'error'].includes(value),
    },
    duration: {
      type: Number,
      default: 1000,
    },
    visible: {
      type: Boolean,
      default: false,
    },
  },
  data() {
    return {
      timeoutId: null,
    };
  },
  computed: {
    iconClass() {
      const icons = {
        info: 'toast-icon-info',
        success: 'toast-icon-success',
        warning: 'toast-icon-warning',
        error: 'toast-icon-error',
      };
      return icons[this.type] || icons.info;
    },
  },
  watch: {
    visible(newVal) {
      // 清除之前的定时器
      if (this.timeoutId) {
        clearTimeout(this.timeoutId);
        this.timeoutId = null;
      }

      if (newVal && this.duration > 0) {
        this.timeoutId = setTimeout(() => {
          this.$emit('close');
          this.timeoutId = null;
        }, this.duration);
      }
    },
  },
  beforeUnmount() {
    // 组件销毁时清除定时器
    if (this.timeoutId) {
      clearTimeout(this.timeoutId);
    }
  },
};
</script>

<style scoped>
.toast {
  position: fixed;
  top: 20px;
  right: 20px;
  z-index: 9999;
  min-width: 300px;
  max-width: 500px;
  background: #ffffff;
  border-radius: 8px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  border-left: 4px solid;
  overflow: hidden;
}

.toast-info {
  border-left-color: #007bff;
}

.toast-success {
  border-left-color: #28a745;
}

.toast-warning {
  border-left-color: #ffc107;
}

.toast-error {
  border-left-color: #dc3545;
}

.toast-content {
  display: flex;
  align-items: center;
  padding: 16px 20px;
  gap: 12px;
}

.toast-icon-info::before {
  content: 'ℹ️';
  font-size: 18px;
}

.toast-icon-success::before {
  content: '✅';
  font-size: 18px;
}

.toast-icon-warning::before {
  content: '⚠️';
  font-size: 18px;
}

.toast-icon-error::before {
  content: '❌';
  font-size: 18px;
}

.toast-message {
  flex: 1;
  font-size: 14px;
  line-height: 1.4;
  color: #2c3e50;
  font-weight: 500;
}

/* 动画效果 */
.toast-enter-active {
  transition: all 0.3s ease-out;
}

.toast-leave-active {
  transition: all 0.3s ease-in;
}

.toast-enter-from {
  transform: translateX(100%);
  opacity: 0;
}

.toast-leave-to {
  transform: translateX(100%);
  opacity: 0;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .toast {
    top: 10px;
    right: 10px;
    left: 10px;
    min-width: auto;
    max-width: none;
  }

  .toast-content {
    padding: 12px 16px;
  }

  .toast-message {
    font-size: 13px;
  }
}
</style>
