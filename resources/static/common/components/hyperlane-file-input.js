class HyperlaneFileInput extends HTMLElement {
  static get observedAttributes() {
    return ['accept', 'multiple', 'capture', 'disabled'];
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
      this.addEventListeners();
    }
  }

  addEventListeners() {
    const input = this.shadowRoot.querySelector('input[type="file"]');
    const label = this.shadowRoot.querySelector('.file-label');

    if (!input) return;

    input.addEventListener('change', (e) => {
      const files = Array.from(e.target.files);
      this.dispatchEvent(
        new CustomEvent('hyperlane-files-selected', {
          bubbles: true,
          composed: true,
          detail: { files, originalEvent: e },
        }),
      );
    });

    if (label) {
      label.addEventListener('click', (e) => {
        if (this.hasAttribute('disabled')) {
          e.preventDefault();
          return;
        }
      });
    }
  }

  get accept() {
    return this.getAttribute('accept') || '';
  }

  get multiple() {
    return this.hasAttribute('multiple');
  }

  get capture() {
    return this.getAttribute('capture') || '';
  }

  get disabled() {
    return this.hasAttribute('disabled');
  }

  clear() {
    const input = this.shadowRoot.querySelector('input[type="file"]');
    if (input) input.value = '';
  }

  render() {
    const accept = this.accept;
    const multiple = this.multiple;
    const capture = this.capture;
    const disabled = this.disabled;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          width: 100%;
          height: 100%;
        }
        .file-input-wrapper {
          position: relative;
          display: block;
          width: 100%;
          height: 100%;
        }
        input[type="file"] {
          display: none;
        }
        .file-label {
          padding: 12px 24px;
          border-radius: 12px;
          border: none;
          font-weight: 600;
          cursor: ${disabled ? 'not-allowed' : 'pointer'};
          transition: all 0.3s ease;
          text-align: center;
          font-size: 1rem;
          background: #28a745;
          color: white;
          box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
          display: inline-flex;
          align-items: center;
          justify-content: center;
          gap: 8px;
          white-space: nowrap;
          opacity: ${disabled ? '0.7' : '1'};
          width: 100%;
          height: 100%;
          box-sizing: border-box;
        }
        .file-label:hover:not(:disabled) {
          background: #218838;
          transform: translateY(-1px);
          box-shadow: 0 8px 30px rgba(0, 0, 0, 0.15);
        }
        .file-label:active:not(:disabled) {
          transform: translateY(0);
        }
        ::slotted(svg) {
          width: 20px;
          height: 20px;
        }
      </style>
      <div class="file-input-wrapper">
        <label class="file-label" part="label">
          <slot></slot>
          <input
            type="file"
            ${accept ? `accept="${accept}"` : ''}
            ${multiple ? 'multiple' : ''}
            ${capture ? `capture="${capture}"` : ''}
            ${disabled ? 'disabled' : ''}
          />
        </label>
      </div>
    `;
  }
}

customElements.define('hyperlane-file-input', HyperlaneFileInput);
