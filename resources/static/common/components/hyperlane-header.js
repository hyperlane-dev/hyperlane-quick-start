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
          color: ${light ? 'var(--hl-fg)' : 'var(--hl-accent-fg)'};
          padding: 40px 20px;
          animation: fadeInDown 0.6s ease;
        }
        .header-content {
          max-width: 1200px;
          margin: 0 auto;
        }
        h1 a {
          position: relative;
          display: inline-flex;
          align-items: center;
          text-decoration: none;
          color: inherit;
        }
        .nav-logo {
          width: 40px;
          height: 40px;
          border-radius: 8px;
          object-fit: contain;
          vertical-align: middle;
          margin-right: 12px;
        }
        .text {
          color: ${light ? 'var(--hl-gray-900)' : 'var(--hl-accent-fg)'};
        }
        .text.shimmer {
          color: inherit;
        }
        h1 a::after {
          content: '';
          position: absolute;
          left: 0;
          bottom: -2px;
          width: 0;
          height: 2px;
          background: currentColor;
          transition: width 0.4s ease;
        }
        h1 a:hover::after {
          width: 100%;
        }
        .app-title {
          font-size: 2.5rem;
          font-weight: 700;
          margin-bottom: 10px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        .app-subtitle {
          font-size: 1.1rem;
          color: ${light ? 'var(--hl-fg-muted)' : 'var(--hl-fg-muted, rgba(255, 255, 255, 0.9))'};
          font-weight: 400;
          margin-top: 8px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        @keyframes fadeInDown {
          from {
            opacity: 0;
          }
          to {
            opacity: 1;
          }
        }
        @keyframes shimmer {
          to { background-position: 200% center; }
        }
        @media (max-width: 768px) {
          .app-title {
            font-size: 2rem;
          }
        }
        @media (max-width: 480px) {
          .app-title {
            font-size: 1.8rem;
          }
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
