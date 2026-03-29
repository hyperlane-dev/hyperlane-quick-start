class HyperlaneCard extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'padding', 'shadow'];
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

  get title() {
    return this.getAttribute('title') || '';
  }

  get padding() {
    return this.getAttribute('padding') || '24px';
  }

  get shadow() {
    return this.getAttribute('shadow') !== 'none';
  }

  render() {
    const title = this.title;
    const padding = this.padding;
    const shadow = this.shadow;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .card {
          background: white;
          border-radius: 12px;
          padding: var(--hyperlane-card-padding, ${padding});
          ${shadow ? 'box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);' : ''}
          animation: fadeInUp 0.6s ease;
        }
        .card-header {
          margin-bottom: 16px;
        }
        .card-title {
          font-size: 1.1rem;
          font-weight: 600;
          color: #2c3e50;
          margin: 0;
        }
        .card-body {
          color: #495057;
        }
        @keyframes fadeInUp {
          from {
            opacity: 0;
            transform: translateY(20px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
          }
        }
      </style>
      <div class="card">
        ${
          title
            ? `
          <div class="card-header">
            <h3 class="card-title">${title}</h3>
          </div>
        `
            : ''
        }
        <div class="card-body">
          <slot></slot>
        </div>
      </div>
    `;
  }
}

customElements.define('hyperlane-card', HyperlaneCard);
