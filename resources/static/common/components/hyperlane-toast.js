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
      toast.style.opacity = '0';
      this.removeAttribute('message');
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
      success:
        'background: var(--hl-accent); color: var(--hl-accent-fg); border: var(--hl-border-w-thin) solid var(--hl-border);',
      error:
        'background: var(--hl-error-bg); color: var(--hl-error-fg); border: var(--hl-border-w-thin) solid var(--hl-border);',
      warning:
        'background: var(--hl-warning-bg); color: var(--hl-warning-fg); border: var(--hl-border-w-thin) solid var(--hl-border);',
      info: 'background: var(--hl-info-bg); color: var(--hl-info-fg); border: var(--hl-border-w-thin) solid var(--hl-border);',
    };

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          position: static;
          width: 100%;
          pointer-events: auto;
        }
        .toast {
          padding: 12px 24px;
          border-radius: var(--hl-radius-sm);
          font-weight: 500;
          min-width: 200px;
          max-width: 400px;
          word-wrap: break-word;
          box-sizing: border-box;
          ${typeStyles[type] || typeStyles.success}
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
      this.container.className = 'toast-container';
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
