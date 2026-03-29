class HyperlaneTraceSearch extends HTMLElement {
  static get observedAttributes() {
    return ['trace-id', 'loading'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
    this._inputHandler = null;
    this._keydownHandler = null;
    this._clickHandler = null;
  }

  connectedCallback() {
    this.render();
    this.addEventListeners();
  }

  disconnectedCallback() {
    this.removeEventListeners();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue && this.shadowRoot.innerHTML) {
      if (name === 'loading') {
        this.updateLoadingState();
      }
    }
  }

  addEventListeners() {
    const input = this.shadowRoot.querySelector('#traceId');
    const button = this.shadowRoot.querySelector('#searchBtn');

    if (input) {
      this._keydownHandler = (e) => {
        if (e.detail.key === 'Enter') {
          e.detail.originalEvent.preventDefault();
          this.dispatchSearchEvent();
        }
      };
      this._inputHandler = (e) => {
        this.setAttribute('trace-id', e.detail.value);
      };
      input.addEventListener('hyperlane-keydown', this._keydownHandler);
      input.addEventListener('hyperlane-input', this._inputHandler);
    }

    if (button) {
      this._clickHandler = () => {
        this.dispatchSearchEvent();
      };
      button.addEventListener('hyperlane-click', this._clickHandler);
    }
  }

  removeEventListeners() {
    const input = this.shadowRoot.querySelector('#traceId');
    const button = this.shadowRoot.querySelector('#searchBtn');

    if (input && this._keydownHandler) {
      input.removeEventListener('hyperlane-keydown', this._keydownHandler);
      input.removeEventListener('hyperlane-input', this._inputHandler);
    }
    if (button && this._clickHandler) {
      button.removeEventListener('hyperlane-click', this._clickHandler);
    }
  }

  updateLoadingState() {
    const input = this.shadowRoot.querySelector('#traceId');
    const button = this.shadowRoot.querySelector('#searchBtn');
    const loading = this.hasAttribute('loading');

    if (input) {
      if (loading) {
        input.setAttribute('disabled', '');
      } else {
        input.removeAttribute('disabled');
      }
    }
    if (button) {
      if (loading) {
        button.setAttribute('loading', '');
      } else {
        button.removeAttribute('loading');
      }
    }
  }

  dispatchSearchEvent() {
    const input = this.shadowRoot?.querySelector('#traceId');
    const traceId = input ? input.value : this.getAttribute('trace-id') || '';
    this.dispatchEvent(
      new CustomEvent('hyperlane-trace-search', {
        bubbles: true,
        composed: true,
        detail: { traceId: traceId.trim() },
      }),
    );
  }

  get traceId() {
    return this.getAttribute('trace-id') || '';
  }

  set traceId(value) {
    this.setAttribute('trace-id', value);
    const input = this.shadowRoot?.querySelector('#traceId');
    if (input) {
      input.value = value;
    }
  }

  focus() {
    const input = this.shadowRoot?.querySelector('#traceId');
    if (input) {
      input.focus();
    }
  }

  render() {
    const traceId = this.traceId;
    const loading = this.hasAttribute('loading');

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .search-container {
          display: flex;
          gap: 12px;
          width: 100%;
          align-items: stretch;
        }
        hyperlane-input {
          flex: 1;
          min-width: 0;
        }
        hyperlane-button {
          flex-shrink: 0;
          width: auto;
        }
      </style>
      <div class="search-container">
        <hyperlane-input
          id="traceId"
          placeholder="Enter Trace ID"
          value="${traceId}"
          ${loading ? 'disabled' : ''}
        ></hyperlane-input>
        <hyperlane-button id="searchBtn" variant="primary" ${loading ? 'loading' : ''}>
          Search
        </hyperlane-button>
      </div>
    `;
  }
}

customElements.define('hyperlane-trace-search', HyperlaneTraceSearch);
