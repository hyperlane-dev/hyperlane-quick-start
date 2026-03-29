class HyperlaneButton extends HTMLElement {
  static get observedAttributes() {
    return ['variant', 'size', 'disabled', 'loading', 'type'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._internals = this.attachInternals?.() || null;
  }

  connectedCallback() {
    this.render();
    this.addEventListeners();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }

  addEventListeners() {
    const button = this.shadowRoot.querySelector('button');
    if (!button) return;

    button.addEventListener('click', (e) => {
      if (this.disabled || this.loading) {
        e.preventDefault();
        e.stopPropagation();
        return;
      }
      this.dispatchEvent(
        new CustomEvent('hyperlane-click', {
          bubbles: true,
          composed: true,
          detail: { originalEvent: e },
        }),
      );
    });
  }

  get variant() {
    return this.getAttribute('variant') || 'primary';
  }

  get size() {
    return this.getAttribute('size') || 'medium';
  }

  get disabled() {
    return this.hasAttribute('disabled');
  }

  get loading() {
    return this.hasAttribute('loading');
  }

  get type() {
    return this.getAttribute('type') || 'button';
  }

  render() {
    const variant = this.variant;
    const size = this.size;
    const isDisabled = this.disabled || this.loading;

    const variantStyles = {
      primary: `
        background: #667eea;
        color: white;
        box-shadow: 0 4px 15px rgba(102, 126, 234, 0.3);
      `,
      success: `
        background: #28a745;
        color: white;
        box-shadow: 0 4px 15px rgba(40, 167, 69, 0.3);
      `,
      danger: `
        background: #dc3545;
        color: white;
        box-shadow: 0 4px 15px rgba(220, 53, 69, 0.3);
      `,
      default: `
        background: #28a745;
        color: white;
        box-shadow: 0 4px 15px rgba(0, 0, 0, 0.1);
      `,
    };

    const sizeStyles = {
      small: 'padding: 8px 16px; font-size: 0.875rem;',
      medium: 'padding: 12px 24px; font-size: 1rem;',
      large: 'padding: 16px 32px; font-size: 1.1rem;',
    };

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          width: 100%;
          height: 100%;
        }
        button {
          position: relative;
          overflow: hidden;
          border: none;
          border-radius: 12px;
          font-weight: 600;
          cursor: ${isDisabled ? 'not-allowed' : 'pointer'};
          transition: all 0.3s ease;
          display: inline-flex;
          align-items: center;
          justify-content: center;
          gap: 8px;
          white-space: nowrap;
          width: 100%;
          height: 100%;
          box-sizing: border-box;
          ${variantStyles[variant] || variantStyles.default}
          ${sizeStyles[size] || sizeStyles.medium}
          opacity: ${isDisabled ? '0.7' : '1'};
          transform: ${isDisabled ? 'none' : 'translateY(0)'};
        }
        button:hover:not(:disabled) {
          transform: translateY(-2px);
          filter: brightness(1.1);
        }
        button:active:not(:disabled) {
          transform: translateY(0);
        }
        .spinner {
          width: 16px;
          height: 16px;
          border: 2px solid rgba(255, 255, 255, 0.3);
          border-top: 2px solid white;
          border-radius: 50%;
          animation: spin 1s linear infinite;
        }
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
        ::slotted(svg) {
          width: 20px;
          height: 20px;
        }
      </style>
      <button type="${this.type}" ?disabled="${isDisabled}" part="button">
        ${this.loading ? '<span class="spinner"></span>' : ''}
        <slot></slot>
      </button>
    `;
  }
}

customElements.define('hyperlane-button', HyperlaneButton);
