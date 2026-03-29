class HyperlaneSelect extends HTMLElement {
  static get observedAttributes() {
    return ['value', 'disabled', 'options'];
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
    const select = this.shadowRoot.querySelector('select');
    if (!select) return;

    select.addEventListener('change', (e) => {
      this.dispatchEvent(
        new CustomEvent('hyperlane-change', {
          bubbles: true,
          composed: true,
          detail: { value: e.target.value, originalEvent: e },
        }),
      );
    });
  }

  get value() {
    const select = this.shadowRoot?.querySelector('select');
    return select ? select.value : this.getAttribute('value') || '';
  }

  set value(val) {
    const select = this.shadowRoot?.querySelector('select');
    if (select) {
      select.value = val;
    }
    this.setAttribute('value', val);
  }

  get disabled() {
    return this.hasAttribute('disabled');
  }

  get options() {
    const optionsAttr = this.getAttribute('options');
    if (optionsAttr) {
      try {
        return JSON.parse(optionsAttr);
      } catch (e) {
        return [];
      }
    }
    return [];
  }

  renderOptions() {
    const options = this.options;
    const currentValue = this.getAttribute('value') || '';

    if (options.length > 0) {
      return options
        .map((opt) => {
          const selected = opt.value === currentValue ? 'selected' : '';
          return `<option value="${opt.value}" ${selected}>${opt.label}</option>`;
        })
        .join('');
    }

    const lightDomOptions = this.querySelectorAll('option');
    if (lightDomOptions.length > 0) {
      return Array.from(lightDomOptions)
        .map((opt) => {
          const selected = opt.value === currentValue ? 'selected' : '';
          return `<option value="${opt.value}" ${selected}>${opt.textContent}</option>`;
        })
        .join('');
    }

    return '';
  }

  render() {
    const disabled = this.disabled;
    const value = this.getAttribute('value') || '';
    const optionsHtml = this.renderOptions();

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        select {
          background: white;
          color: #667eea;
          border: none;
          padding: 8px 32px 8px 12px;
          border-radius: 6px;
          cursor: ${disabled ? 'not-allowed' : 'pointer'};
          font-weight: 600;
          font-size: 0.95rem;
          outline: none;
          appearance: none;
          -webkit-appearance: none;
          -moz-appearance: none;
          background-image: url("data:image/svg+xml;charset=UTF-8,%3csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23667eea' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3e%3cpolyline points='6 9 12 15 18 9'%3e%3c/polyline%3e%3c/svg%3e");
          background-repeat: no-repeat;
          background-position: right 8px center;
          background-size: 16px;
          opacity: ${disabled ? '0.7' : '1'};
        }
      </style>
      <select ${disabled ? 'disabled' : ''} ${value ? `value="${value}"` : ''}>
        ${optionsHtml}
      </select>
    `;
  }
}

customElements.define('hyperlane-select', HyperlaneSelect);
