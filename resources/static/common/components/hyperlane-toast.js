class HyperlaneToast extends HTMLElement {
  static get observedAttributes() {
    return ['message', 'type', 'duration'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._timeout = null;
  }

  connectedCallback() {
    this.render();
  }

  disconnectedCallback() {
    if (this._timeout) {
      clearTimeout(this._timeout);
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }

  get message() {
    return this.getAttribute('message') || '';
  }

  get type() {
    return this.getAttribute('type') || 'success';
  }

  get duration() {
    return parseInt(this.getAttribute('duration')) || 3000;
  }

  show(message, type = 'success', duration = 3000) {
    this.setAttribute('message', message);
    this.setAttribute('type', type);
    this.setAttribute('duration', duration);

    if (this._timeout) {
      clearTimeout(this._timeout);
    }

    this._timeout = setTimeout(() => {
      this.hide();
    }, duration);
  }

  hide() {
    const toast = this.shadowRoot.querySelector('.toast');
    if (toast) {
      toast.style.animation = 'slideOut 0.3s ease forwards';
      setTimeout(() => {
        this.removeAttribute('message');
      }, 300);
    }
  }

  render() {
    const message = this.message;
    const type = this.type;

    if (!message) {
      this.shadowRoot.innerHTML = '';
      return;
    }

    const typeStyles = {
      success: 'background: #28a745; color: white;',
      error: 'background: #dc3545; color: white;',
      warning: 'background: #ffc107; color: #212529;',
      info: 'background: #17a2b8; color: white;',
    };

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          position: fixed;
          top: 20px;
          right: 20px;
          z-index: 10000;
        }
        .toast {
          padding: 12px 24px;
          border-radius: 8px;
          font-weight: 500;
          box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
          animation: slideIn 0.3s ease;
          min-width: 200px;
          max-width: 400px;
          word-wrap: break-word;
          ${typeStyles[type] || typeStyles.success}
        }
        @keyframes slideIn {
          from {
            opacity: 0;
            transform: translateX(100%);
          }
          to {
            opacity: 1;
            transform: translateX(0);
          }
        }
        @keyframes slideOut {
          from {
            opacity: 1;
            transform: translateX(0);
          }
          to {
            opacity: 0;
            transform: translateX(100%);
          }
        }
      </style>
      <div class="toast">${message}</div>
    `;

    if (this._timeout) {
      clearTimeout(this._timeout);
    }
    this._timeout = setTimeout(() => {
      this.hide();
    }, this.duration);
  }
}

customElements.define('hyperlane-toast', HyperlaneToast);

window.HLToast = {
  container: null,

  init() {
    if (!this.container) {
      this.container = document.createElement('div');
      this.container.id = 'hyperlane-toast-container';
      this.container.style.cssText =
        'position:fixed;top:20px;right:20px;z-index:10000;';
      document.body.appendChild(this.container);
    }
  },

  show(message, type = 'success', duration = 3000) {
    this.init();
    const toast = document.createElement('hyperlane-toast');
    toast.show(message, type, duration);
    this.container.appendChild(toast);

    setTimeout(() => {
      if (toast.parentNode) {
        toast.parentNode.removeChild(toast);
      }
    }, duration + 500);
  },

  success(message, duration) {
    this.show(message, 'success', duration);
  },

  error(message, duration) {
    this.show(message, 'error', duration);
  },

  warning(message, duration) {
    this.show(message, 'warning', duration);
  },

  info(message, duration) {
    this.show(message, 'info', duration);
  },
};
