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
          background: var(--hl-surface);
          border: 0;
          border-radius: var(--hl-radius-md);
          padding: var(--hyperlane-card-padding, ${padding});
          ${shadow ? 'border: var(--hl-border-w-medium) solid var(--hl-border);' : ''}
        }
        .card-header {
          margin-bottom: 16px;
        }
        .card-title {
          font-size: 1.1rem;
          font-weight: 600;
          color: var(--hl-fg);
          margin: 0;
        }
        .card-body {
          color: var(--hl-fg-muted);
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
