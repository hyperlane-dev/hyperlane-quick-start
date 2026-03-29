class HyperlaneInput extends HTMLElement {
  static get observedAttributes() {
    return [
      'type',
      'placeholder',
      'value',
      'disabled',
      'readonly',
      'required',
      'name',
      'id',
    ];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
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
    const input = this.shadowRoot.querySelector('input');
    if (!input) return;

    input.addEventListener('input', (e) => {
      this.dispatchEvent(
        new CustomEvent('hyperlane-input', {
          bubbles: true,
          composed: true,
          detail: { value: e.target.value, originalEvent: e },
        }),
      );
    });

    input.addEventListener('change', (e) => {
      this.dispatchEvent(
        new CustomEvent('hyperlane-change', {
          bubbles: true,
          composed: true,
          detail: { value: e.target.value, originalEvent: e },
        }),
      );
    });

    input.addEventListener('keydown', (e) => {
      this.dispatchEvent(
        new CustomEvent('hyperlane-keydown', {
          bubbles: true,
          composed: true,
          detail: { key: e.key, originalEvent: e },
        }),
      );
    });

    input.addEventListener('blur', (e) => {
      this.dispatchEvent(
        new CustomEvent('hyperlane-blur', {
          bubbles: true,
          composed: true,
          detail: { value: e.target.value, originalEvent: e },
        }),
      );
    });
  }

  get value() {
    const input = this.shadowRoot?.querySelector('input');
    return input ? input.value : this.getAttribute('value') || '';
  }

  set value(val) {
    const input = this.shadowRoot?.querySelector('input');
    if (input) {
      input.value = val;
    }
    this.setAttribute('value', val);
  }

  focus() {
    const input = this.shadowRoot?.querySelector('input');
    if (input) input.focus();
  }

  render() {
    const type = this.getAttribute('type') || 'text';
    const placeholder = this.getAttribute('placeholder') || '';
    const value = this.getAttribute('value') || '';
    const disabled = this.hasAttribute('disabled');
    const readonly = this.hasAttribute('readonly');
    const required = this.hasAttribute('required');
    const name = this.getAttribute('name') || '';
    const id = this.getAttribute('id') || '';

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          width: 100%;
        }
        .input-wrapper {
          position: relative;
          display: flex;
          align-items: center;
          width: 100%;
        }
        input {
          width: 100%;
          padding: 12px 16px;
          border: 2px solid #e9ecef;
          border-radius: 8px;
          font-size: 16px;
          transition: all 0.3s ease;
          background: white;
          box-sizing: border-box;
          font-family: inherit;
        }
        input:focus {
          outline: none;
          border-color: #667eea;
          box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
        }
        input:disabled {
          background: #e9ecef;
          cursor: not-allowed;
        }
        input[type="url"]:valid:not(:placeholder-shown) {
          border-color: #28a745;
        }
        ::slotted([slot="icon"]) {
          position: absolute;
          left: 16px;
          font-size: 1.2rem;
          color: #6c757d;
          pointer-events: none;
        }
        input.has-icon {
          padding-left: 50px;
        }
      </style>
      <div class="input-wrapper">
        <slot name="icon"></slot>
        <input
          type="${type}"
          placeholder="${placeholder}"
          value="${value}"
          ${disabled ? 'disabled' : ''}
          ${readonly ? 'readonly' : ''}
          ${required ? 'required' : ''}
          ${name ? `name="${name}"` : ''}
          ${id ? `id="${id}"` : ''}
          class="${this.querySelector('[slot=icon]') ? 'has-icon' : ''}"
          part="input"
        />
      </div>
    `;
  }
}

customElements.define('hyperlane-input', HyperlaneInput);
