class HyperlaneTraceResult extends HTMLElement {
  static get observedAttributes() {
    return ['data', 'url', 'status', 'loading'];
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
    const urlElement = this.shadowRoot.querySelector('.request-url');
    if (urlElement) {
      urlElement.addEventListener('click', () => {
        const url = this.getAttribute('url');
        if (url) {
          window.open(url, '_blank');
        }
      });
    }
  }

  get data() {
    return this.getAttribute('data') || '';
  }

  get url() {
    return this.getAttribute('url') || '';
  }

  get status() {
    return this.getAttribute('status') || 'empty';
  }

  get loading() {
    return this.hasAttribute('loading');
  }

  set data(value) {
    this.setAttribute('data', value);
  }

  set url(value) {
    this.setAttribute('url', value);
  }

  set status(value) {
    this.setAttribute('status', value);
  }

  render() {
    const data = this.data;
    const url = this.url;
    const status = this.status;
    const loading = this.loading;

    const statusMessages = {
      success: { text: '✓ Search completed successfully', class: 'success' },
      error: { text: '✗ Search failed', class: 'error' },
      notfound: {
        text: '✗ Search failed: Trace record not found',
        class: 'error',
      },
      network: {
        text: '✗ Search failed: Network request error',
        class: 'error',
      },
      empty: {
        text: 'Please enter a Trace ID and click the search button',
        class: 'empty',
      },
    };

    const statusInfo = statusMessages[status] || statusMessages.empty;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .request-url {
          margin-bottom: 16px;
          padding: 12px 16px;
          background: var(--hl-bg-muted);
          border: 1px solid var(--hl-border);
          border-radius: 8px;
          font-family: 'Courier New', monospace;
          font-size: 13px;
          word-break: break-all;
          cursor: pointer;
          transition: all 0.3s ease;
          color: var(--hl-accent);
          font-weight: 500;
        }
        .status-message {
          padding: 12px 16px;
          border-radius: 8px;
          margin-bottom: 16px;
          font-weight: 600;
          animation: fadeIn 0.3s ease;
        }
        .status-message.success {
          background: var(--hl-success-bg);
          color: var(--hl-success);
          border-left: 4px solid var(--hl-success);
        }
        .status-message.error {
          background: var(--hl-error-bg);
          color: var(--hl-error);
          border-left: 4px solid var(--hl-border-strong);
        }
        .result-content {
          background: var(--hl-gray-50);
          border: 1px solid var(--hl-gray-150);
          border-radius: 8px;
          padding: 16px;
          white-space: pre-wrap;
          word-wrap: break-word;
          overflow-wrap: break-word;
          font-family: 'Courier New', monospace;
          font-size: 14px;
          line-height: 1.6;
          min-height: 100px;
          max-height: 600px;
          overflow-y: auto;
          color: var(--hl-fg);
        }
        .result-content::-webkit-scrollbar {
          width: 6px;
        }
        .result-content::-webkit-scrollbar-track {
          background: var(--hl-overlay-muted);
        }
        .result-content::-webkit-scrollbar-thumb {
          background: var(--hl-overlay-fade);
          border-radius: 3px;
        }
        .empty-result {
          color: var(--hl-fg-muted);
          font-style: italic;
          text-align: center;
          padding: 40px 20px;
        }
        .loading-container {
          text-align: center;
          padding: 40px 20px;
          color: var(--hl-fg-muted);
        }
        .spinner {
          width: 40px;
          height: 40px;
          border: 3px solid var(--hl-overlay-fade);
          border-top: 3px solid var(--hl-accent);
          border-radius: 50%;
          animation: spin 1s linear infinite;
          margin: 0 auto 20px;
        }
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
        @keyframes fadeIn {
          from { opacity: 0; }
          to { opacity: 1; }
        }
      </style>
      ${url ? `<div class="request-url" title="Click to open request URL in new window">${url}</div>` : ''}
      ${status !== 'empty' && !loading ? `<div class="status-message ${statusInfo.class}">${statusInfo.text}</div>` : ''}
      ${
        loading
          ? `
        <div class="loading-container">
          <div class="spinner"></div>
          <div>Searching...</div>
        </div>
      `
          : `
        <pre class="result-content ${status === 'empty' ? 'empty-result' : ''}">${data || statusInfo.text}</pre>
      `
      }
    `;
  }
}

customElements.define('hyperlane-trace-result', HyperlaneTraceResult);
