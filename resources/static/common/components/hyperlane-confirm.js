class HyperlaneConfirm extends HTMLElement {
  static get observedAttributes() {
    return ['message', 'visible'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._resolve = null;
    this._reject = null;
  }

  connectedCallback() {
    this.render();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }

  get message() {
    return this.getAttribute('message') || 'Are you sure?';
  }

  get visible() {
    return this.getAttribute('visible') === 'true';
  }

  show(message) {
    this.setAttribute('message', message);
    this.setAttribute('visible', 'true');
    return new Promise((resolve) => {
      this._resolve = resolve;
    });
  }

  hide() {
    this.setAttribute('visible', 'false');
    this._resolve = null;
  }

  confirm() {
    if (this._resolve) {
      this._resolve(true);
    }
    this.hide();
  }

  cancel() {
    if (this._resolve) {
      this._resolve(false);
    }
    this.hide();
  }

  render() {
    const visible = this.visible;
    const message = this.message;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: ${visible ? 'flex' : 'none'};
          position: fixed;
          top: 0;
          left: 0;
          width: 100%;
          height: 100%;
          background: var(--hl-overlay);
          z-index: 10001;
          justify-content: center;
          align-items: center;
        }
        .confirm-box {
          background: var(--hl-surface);
          border: var(--hl-border-w-medium) solid var(--hl-border);
          border-radius: var(--hl-radius-md);
          padding: 28px 32px;
          max-width: 400px;
          width: 90%;
        }
        .confirm-message {
          color: var(--hl-fg);
          font-size: 1rem;
          line-height: 1.5;
          margin-bottom: 24px;
          text-align: center;
        }
        .confirm-actions {
          display: flex;
          justify-content: center;
          gap: 12px;
        }
        .btn {
          padding: 10px 24px;
          border: var(--hl-border-w-thin) solid var(--hl-border);
          border-radius: var(--hl-radius-sm);
          font-size: 0.95rem;
          font-weight: 600;
          cursor: pointer;
        }
        .btn-cancel {
          background: var(--hl-surface);
          color: var(--hl-fg);
        }
        .btn-confirm {
          background: var(--hl-accent);
          color: var(--hl-accent-fg);
        }
      </style>
      <div class="confirm-box">
        <div class="confirm-message">${message}</div>
        <div class="confirm-actions">
          <button class="btn btn-cancel" id="cancelBtn">Cancel</button>
          <button class="btn btn-confirm" id="confirmBtn">Confirm</button>
        </div>
      </div>
    `;

    if (visible) {
      const cancelBtn = this.shadowRoot.getElementById('cancelBtn');
      const confirmBtn = this.shadowRoot.getElementById('confirmBtn');
      if (cancelBtn) {
        cancelBtn.addEventListener('click', () => this.cancel());
      }
      if (confirmBtn) {
        confirmBtn.addEventListener('click', () => this.confirm());
      }
      this.addEventListener('click', (e) => {
        if (e.target === this) {
          this.cancel();
        }
      });
    }
  }
}

customElements.define('hyperlane-confirm', HyperlaneConfirm);

window.HLConfirm = {
  instance: null,

  init() {
    if (!this.instance) {
      this.instance = document.createElement('hyperlane-confirm');
      document.body.appendChild(this.instance);
    }
  },

  async show(message) {
    this.init();
    return this.instance.show(message);
  },
};
