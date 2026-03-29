class HyperlaneHeader extends HTMLElement {
  static get observedAttributes() {
    return ['title', 'subtitle', 'logo', 'href', 'shimmer', 'light'];
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
      this.getAttribute('logo') || 'https://docs.ltpp.vip/img/hyperlane.png'
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

  render() {
    const title = this.title;
    const subtitle = this.subtitle;
    const logo = this.logo;
    const href = this.href;
    const shimmer = this.shimmer;
    const light = this.light;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
        }
        .app-header {
          text-align: center;
          color: ${light ? '#2c3e50' : 'white'};
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
          filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
        }
        .text {
          color: ${light ? '#667eea' : '#ffd700'};
        }
        .text.shimmer {
          background: linear-gradient(90deg, ${light ? '#667eea, #764ba2, #667eea, #764ba2' : '#fff, #ffd700, #fff, #ffd700'});
          background-size: 200% auto;
          -webkit-background-clip: text;
          -webkit-text-fill-color: transparent;
          background-clip: text;
          animation: shimmer 3s linear infinite;
        }
        .shimmer-header h1 a::after {
          background: linear-gradient(90deg, ${light ? '#667eea, #764ba2, #667eea' : '#ffd700, #fff, #ffd700'});
        }
        h1 a::after {
          content: '';
          position: absolute;
          left: 0;
          bottom: -2px;
          width: 0;
          height: 2px;
          background: ${light ? '#667eea' : '#ffd700'};
          transition: width 0.4s ease;
        }
        h1 a:hover::after {
          width: 100%;
        }
        .app-title {
          font-size: 2.5rem;
          font-weight: 700;
          margin-bottom: 10px;
          ${light ? '' : 'text-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);'}
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        .app-subtitle {
          font-size: 1.1rem;
          color: ${light ? '#5a6c7d' : 'rgba(255, 255, 255, 0.9)'};
          font-weight: 400;
          margin-top: 8px;
          white-space: nowrap;
          overflow: hidden;
          text-overflow: ellipsis;
        }
        @keyframes fadeInDown {
          from {
            opacity: 0;
            transform: translateY(-20px);
          }
          to {
            opacity: 1;
            transform: translateY(0);
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
