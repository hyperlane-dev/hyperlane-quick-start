// Toast service for global toast management
class ToastService {
  constructor() {
    this.container = null;
  }

  // Set the toast container reference
  setContainer(container) {
    this.container = container;
  }

  // Show info toast
  info(message, duration = 1000) {
    return this.show({
      message,
      type: 'info',
      duration,
    });
  }

  // Show success toast
  success(message, duration = 1000) {
    return this.show({
      message,
      type: 'success',
      duration,
    });
  }

  // Show warning toast
  warning(message, duration = 1000) {
    return this.show({
      message,
      type: 'warning',
      duration,
    });
  }

  // Show error toast
  error(message, duration = 1000) {
    return this.show({
      message,
      type: 'error',
      duration,
    });
  }

  // Show toast with custom options
  show(options) {
    if (!this.container) {
      console.warn('Toast container not initialized');
      return null;
    }
    return this.container.addToast(options);
  }

  // Remove specific toast
  remove(id) {
    if (!this.container) {
      return;
    }
    this.container.removeToast(id);
  }

  // Clear all toasts
  clear() {
    if (!this.container) {
      return;
    }
    this.container.clearAll();
  }
}

// Create and export singleton instance
export const toast = new ToastService();

// Vue plugin for easy integration
export default {
  install(app) {
    app.config.globalProperties.$toast = toast;
    app.provide('toast', toast);
  },
};
