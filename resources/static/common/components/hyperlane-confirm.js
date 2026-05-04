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
          background: rgba(0, 0, 0, 0.6);
          z-index: 10001;
          justify-content: center;
          align-items: center;
        }
        .confirm-box {
          background: #ffffff;
          border-radius: 12px;
          padding: 28px 32px;
          max-width: 400px;
          width: 90%;
          box-shadow: 0 8px 40px rgba(0, 0, 0, 0.2);
          animation: scaleIn 0.2s ease;
        }
        @keyframes scaleIn {
          from { opacity: 0; transform: scale(0.9); }
          to { opacity: 1; transform: scale(1); }
        }
        .confirm-message {
          color: #2c3e50;
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
          border: none;
          border-radius: 8px;
          font-size: 0.95rem;
          font-weight: 600;
          cursor: pointer;
          transition: all 0.25s ease;
        }
        .btn-cancel {
          background: #f1f3f5;
          color: #495057;
          border: 1px solid #dee2e6;
        }
        .btn-cancel:hover {
          background: #e9ecef;
        }
        .btn-confirm {
          background: linear-gradient(135deg, #dc3545 0%, #c82333 100%);
          color: white;
          box-shadow: 0 2px 8px rgba(220, 53, 69, 0.3);
        }
        .btn-confirm:hover {
          box-shadow: 0 6px 20px rgba(220, 53, 69, 0.4);
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
