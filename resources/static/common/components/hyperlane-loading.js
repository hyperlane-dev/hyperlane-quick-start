class HyperlaneLoading extends HTMLElement {
  static get observedAttributes() {
    return ['visible', 'text', 'size'];
  }

  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue !== newValue) {
      this.render();
    }
  }

  get visible() {
    return this.hasAttribute('visible');
  }

  get text() {
    return this.getAttribute('text') || 'Loading...';
  }

  get size() {
    return this.getAttribute('size') || 'medium';
  }

  show(text) {
    if (text) this.setAttribute('text', text);
    this.setAttribute('visible', '');
  }

  hide() {
    this.removeAttribute('visible');
  }

  render() {
    const visible = this.visible;
    const text = this.text;
    const size = this.size;

    const sizeStyles = {
      small: { spinner: '30px', fontSize: '0.9rem' },
      medium: { spinner: '50px', fontSize: '1.1rem' },
      large: { spinner: '70px', fontSize: '1.3rem' },
    };

    const styles = sizeStyles[size] || sizeStyles.medium;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: ${visible ? 'block' : 'none'};
        }
        .loading-container {
          text-align: center;
          background: rgba(255, 255, 255, 0.95);
          backdrop-filter: blur(10px);
          border-radius: 20px;
          padding: 60px 40px;
          box-shadow: 0 10px 40px rgba(0, 0, 0, 0.1);
          border: 1px solid rgba(255, 255, 255, 0.2);
          animation: fadeIn 0.3s ease;
        }
        .spinner {
          width: ${styles.spinner};
          height: ${styles.spinner};
          border: 4px solid #f3f3f3;
          border-top: 4px solid #667eea;
          border-radius: 50%;
          animation: spin 1s linear infinite;
          margin: 0 auto 20px;
        }
        .loading-text {
          color: #6c757d;
          font-size: ${styles.fontSize};
          font-weight: 500;
        }
        @keyframes spin {
          to { transform: rotate(360deg); }
        }
        @keyframes fadeIn {
          from { opacity: 0; }
          to { opacity: 1; }
        }
      </style>
      <div class="loading-container">
        <div class="spinner"></div>
        <p class="loading-text">${text}</p>
      </div>
    `;
  }
}

customElements.define('hyperlane-loading', HyperlaneLoading);
