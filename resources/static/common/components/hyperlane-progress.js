class HyperlaneProgress extends HTMLElement {
  static get observedAttributes() {
    return ['value', 'max', 'height', 'animated'];
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

  get value() {
    return parseFloat(this.getAttribute('value')) || 0;
  }

  get max() {
    return parseFloat(this.getAttribute('max')) || 100;
  }

  get height() {
    return this.getAttribute('height') || '6px';
  }

  get animated() {
    return this.hasAttribute('animated');
  }

  get percentage() {
    const value = this.value;
    const max = this.max;
    if (max <= 0) return 0;
    return Math.min(100, Math.max(0, (value / max) * 100));
  }

  render() {
    const percentage = this.percentage;
    const height = this.height;
    const animated = this.animated;

    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: block;
          width: 100%;
        }
        .progress-bar {
          height: ${height};
          background: var(--hl-bg-muted);
          border-radius: ${parseInt(height) / 2}px;
          overflow: hidden;
        }
        .progress-fill {
          height: 100%;
          background: var(--hl-gray-900);
          border-radius: ${parseInt(height) / 2}px;
          transition: width 0.3s ease;
          width: ${percentage}%;
          ${
            animated
              ? `
            opacity: 0.85;
            animation: pulse 2s ease-in-out infinite;
          `
              : ''
          }
        }
        @keyframes pulse {
          0%, 100% { opacity: 0.85; }
          50% { opacity: 0.6; }
        }
      </style>
      <div class="progress-bar">
        <div class="progress-fill"></div>
      </div>
    `;
  }
}

customElements.define('hyperlane-progress', HyperlaneProgress);
