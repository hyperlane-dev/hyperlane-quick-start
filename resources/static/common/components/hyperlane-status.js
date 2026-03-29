class HyperlaneStatus extends HTMLElement {
  static get observedAttributes() {
    return ['type', 'message', 'visible'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }

  get type() {
    return this.getAttribute('type') || 'info';
  }

  get message() {
    return this.getAttribute('message') || '';
  }

  get visible() {
    return this.hasAttribute('visible');
  }

  show(message, type = 'info') {
    this.setAttribute('message', message);
    this.setAttribute('type', type);
    this.setAttribute('visible', '');
  }

  hide() {
    this.removeAttribute('visible');
  }

  render() {
    const type = this.type;
    const message = this.message;
    const visible = this.visible;

    const typeStyles = {
      success: `
        background-color: #f0fff4;
        color: #2f855a;
        border-left: 4px solid #2f855a;
      `,
      error: `
        background-color: #fff5f5;
        color: #c53030;
        border-left: 4px solid #c53030;
      `,
      warning: `
        background-color: #fffbeb;
        color: #d97706;
        border-left: 4px solid #d97706;
      `,
      info: `
        background-color: #eff6ff;
        color: #2563eb;
        border-left: 4px solid #2563eb;
      `,
    };

    const icons = {
      success: `<svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/></svg>`,
      error: `<svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/></svg>`,
      warning: `<svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd"/></svg>`,
      info: `<svg width="16" height="16" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/></svg>`,
    };

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: ${visible ? 'flex' : 'none'};
        }
        .status {
          position: relative;
          padding: 8px 24px;
          border-radius: 12px;
          font-size: 0.875rem;
          animation: slideIn 0.3s ease;
          z-index: 1000;
          box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
          width: 100%;
          display: flex;
          align-items: center;
          justify-content: center;
          gap: 8px;
          background: white;
          margin-bottom: 16px;
          ${typeStyles[type] || typeStyles.info}
        }
        @keyframes slideIn {
          from {
            opacity: 0;
            transform: translateY(-10px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }
      </style>
      <div class="status">
        ${icons[type] || icons.info}
        <span>${message}</span>
      </div>
    `;
  }
}

customElements.define('hyperlane-status', HyperlaneStatus);
