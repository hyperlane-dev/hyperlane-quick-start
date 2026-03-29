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
          background: rgba(102, 126, 234, 0.1);
          border: 1px solid rgba(102, 126, 234, 0.2);
          border-radius: 8px;
          font-family: 'Courier New', monospace;
          font-size: 13px;
          word-break: break-all;
          cursor: pointer;
          transition: all 0.3s ease;
          color: #667eea;
          font-weight: 500;
        }
        .request-url:hover {
          background: rgba(102, 126, 234, 0.15);
          border-color: rgba(102, 126, 234, 0.4);
          transform: translateY(-2px);
          box-shadow: 0 4px 12px rgba(102, 126, 234, 0.2);
        }
        .status-message {
          padding: 12px 16px;
          border-radius: 8px;
          margin-bottom: 16px;
          font-weight: 600;
          animation: fadeIn 0.3s ease;
        }
        .status-message.success {
          background: #d4edda;
          color: #155724;
          border-left: 4px solid #28a745;
        }
        .status-message.error {
          background: #f8d7da;
          color: #721c24;
          border-left: 4px solid #dc3545;
        }
        .result-content {
          background: #f8f9fa;
          border: 1px solid #e9ecef;
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
          color: #2c3e50;
        }
        .result-content::-webkit-scrollbar {
          width: 6px;
        }
        .result-content::-webkit-scrollbar-track {
          background: rgba(0, 0, 0, 0.05);
        }
        .result-content::-webkit-scrollbar-thumb {
          background: rgba(102, 126, 234, 0.3);
          border-radius: 3px;
        }
        .empty-result {
          color: #6c757d;
          font-style: italic;
          text-align: center;
          padding: 40px 20px;
        }
        .loading-container {
          text-align: center;
          padding: 40px 20px;
          color: #6c757d;
        }
        .spinner {
          width: 40px;
          height: 40px;
          border: 3px solid rgba(102, 126, 234, 0.3);
          border-top: 3px solid #667eea;
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
