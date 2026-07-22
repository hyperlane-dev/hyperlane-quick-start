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
      e.stopPropagation();
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
        background: var(--hl-accent);
        color: var(--hl-accent-fg);
        border: var(--hl-border-w-thin) solid var(--hl-border);
      `,
      success: `
        background: var(--hl-success-bg);
        color: var(--hl-success-fg);
        border: var(--hl-border-w-thin) solid var(--hl-border);
      `,
      danger: `
        background: var(--hl-error-bg);
        color: var(--hl-error-fg);
        border: var(--hl-border-w-thin) solid var(--hl-border);
      `,
      default: `
        background: var(--hl-surface);
        color: var(--hl-fg);
        border: var(--hl-border-w-thin) solid var(--hl-border);
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
        :host([hidden]) {
          display: none;
        }
        button {
          position: relative;
          overflow: hidden;
          border: none;
          border-radius: var(--hl-radius-md);
          font-weight: 600;
          cursor: ${isDisabled ? 'not-allowed' : 'pointer'};
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
        }
        .spinner {
          width: 16px;
          height: 16px;
          border: 2px solid var(--hl-border-strong);
          border-top: 2px solid currentColor;
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
    this.addEventListeners();
  }
}

customElements.define('hyperlane-button', HyperlaneButton);
