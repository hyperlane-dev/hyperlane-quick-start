const Toast = {
  show: function (message, type = 'info', duration = 3000) {
    let container = document.getElementById('toast-container');
    if (!container) {
      container = document.createElement('div');
      container.id = 'toast-container';
      container.className = 'toast-container';
      document.body.appendChild(container);
    }
    const toast = document.createElement('div');
    toast.className = 'toast';
    const iconMap = {
      success: '✓',
      error: '✕',
      info: 'ℹ',
      warning: '⚠',
    };
    const icon = iconMap[type] || iconMap.info;
    toast.innerHTML = `
      <span class="toast-icon ${type}">${icon}</span>
      <span class="toast-content">${message}</span>
    `;
    container.appendChild(toast);
    const removeToast = () => {
      toast.classList.add('hiding');
      setTimeout(() => {
        if (toast.parentNode) {
          toast.parentNode.removeChild(toast);
        }
      }, 300);
    };
    toast.addEventListener('click', removeToast);
    setTimeout(removeToast, duration);
  },

  success: function (message, duration) {
    this.show(message, 'success', duration);
  },

  error: function (message, duration) {
    this.show(message, 'error', duration);
  },

  info: function (message, duration) {
    this.show(message, 'info', duration);
  },

  warning: function (message, duration) {
    this.show(message, 'warning', duration);
  },
};

if (typeof module !== 'undefined' && module.exports) {
  module.exports = Toast;
}
