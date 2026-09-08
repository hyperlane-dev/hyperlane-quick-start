class HyperlaneHeader extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'subtitle', 'logo', 'href', 'shimmer', 'light', 'dark'];
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

  get subtitle() {
    return this.getAttribute('subtitle') || '';
  }

  get logo() {
    return (
      this.getAttribute('logo') ||
      'https://ltpp.vip/github/pages/docs-pages/pages/img/hyperlane.png'
    );
  }

  get href() {
    return (
      this.getAttribute('href') || 'https://github.com/hyperlane-dev/hyperlane'
    );
  }

  get shimmer() {
    return this.hasAttribute('shimmer');
  }

  get light() {
    return this.hasAttribute('light');
  }

  get dark() {
    return this.hasAttribute('dark');
  }

  render() {
    const title = this.title;
    const subtitle = this.subtitle;
    const logo = this.logo;
    const href = this.href;
    const shimmer = this.shimmer;
    const light = this.light || !this.dark;
    const dark = this.dark;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .app-header {
          text-align: center;
          color: var(--hl-fg);
          padding: 40px 20px;
        }
        .header-content {
          max-width: 1200px;
          margin: 0 auto;
        }
        h1 {
          display: flex;
          justify-content: center;
          max-width: 100%;
          margin: 0;
        }
        h1 a {
          position: relative;
          display: flex;
          align-items: center;
          max-width: 100%;
          min-width: 0;
          flex-wrap: wrap;
          justify-content: center;
          gap: 12px;
          text-decoration: none;
          color: inherit;
        }
        .nav-logo {
          width: 40px;
          height: 40px;
          border-radius: var(--hl-radius-md);
          object-fit: contain;
          vertical-align: middle;
          flex-shrink: 0;
        }
        .text {
          color: var(--hl-fg);
          font-weight: 600;
          min-width: 0;
        }
        .text.shimmer {
          color: var(--hl-fg);
          opacity: 1;
        }
        .app-title {
          font-size: clamp(1rem, 4.5vw, 2.5rem);
          font-weight: 700;
          margin-bottom: 10px;
          max-width: 100%;
        }
        .app-subtitle {
          font-size: clamp(0.85rem, 2.6vw, 1.1rem);
          color: var(--hl-fg-muted);
          font-weight: 400;
          margin-top: 8px;
          max-width: 100%;
        }
        @media (max-width: 480px) {
          .app-header {
            padding: 30px 15px;
          }
        }
      </style>
      <header class="app-header ${shimmer ? 'shimmer-header' : ''}">
        <div class="header-content">
          <h1 class="app-title">
            <a href="${href}" target="_blank">
              <img src="${logo}" alt="" class="nav-logo" onerror="this.style.display='none'" />
              <span class="text ${shimmer ? 'shimmer' : ''}">${title}</span>
            </a>
          </h1>
          ${subtitle ? `<p class="app-subtitle">${subtitle}</p>` : ''}
          <slot></slot>
        </div>
      </header>
    `;
  }
}

customElements.define('hyperlane-header', HyperlaneHeader);
