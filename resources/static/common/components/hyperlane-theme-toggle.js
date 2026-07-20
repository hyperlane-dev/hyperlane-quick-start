/*
 * Theme toggle component. The site is light-theme-only; this component still
 * exposes the same API (`toggle()`, `theme`, `hyperlane-theme-change` event)
 * so existing markup keeps working, but it no longer switches to a dark theme.
 */
(function () {
  try {
    document.documentElement.setAttribute('data-theme', 'light');
  } catch (_e) {
    document.documentElement.setAttribute('data-theme', 'light');
  }
})();

class HyperlaneThemeToggle extends HTMLElement {
  constructor() {
    super();
    this.attachShadow({ mode: 'open' });
  }

  connectedCallback() {
    this.render();
  }

  get theme() {
    return 'light';
  }

  _setTheme(_theme) {
    document.documentElement.setAttribute('data-theme', 'light');
    try {
      window.localStorage.setItem('hl-theme', 'light');
    } catch (_e) {}
    this._updateIcon();
    this.dispatchEvent(
      new CustomEvent('hyperlane-theme-change', {
        bubbles: true,
        composed: true,
        detail: { theme: 'light' },
      }),
    );
  }

  toggle() {
    this._setTheme('light');
  }

  _updateIcon() {
    const btn = this.shadowRoot.querySelector('button');
    if (!btn) return;
    btn.setAttribute('aria-label', 'Light theme (locked)');
    btn.innerHTML =
      '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="5"/><line x1="12" y1="1" x2="12" y2="3"/><line x1="12" y1="21" x2="12" y2="23"/><line x1="4.22" y1="4.22" x2="5.64" y2="5.64"/><line x1="18.36" y1="18.36" x2="19.78" y2="19.78"/><line x1="1" y1="12" x2="3" y2="12"/><line x1="21" y1="12" x2="23" y2="12"/><line x1="4.22" y1="19.78" x2="5.64" y2="18.36"/><line x1="18.36" y1="5.64" x2="19.78" y2="4.22"/></svg>';
  }

  render() {
    this.shadowRoot.innerHTML = `
      <style>
        :host {
          display: inline-flex;
        }
        button {
          display: inline-flex;
          align-items: center;
          justify-content: center;
          width: 38px;
          height: 38px;
          padding: 8px;
          border-radius: var(--hl-radius-sm);
          border: 1px solid var(--hl-border);
          background: var(--hl-surface);
          color: var(--hl-fg);
          cursor: pointer;
          transition: var(--hl-transition);
          box-sizing: border-box;
        }
        button:hover {
          background: var(--hl-surface-hover);
          border-color: var(--hl-border-strong);
        }
        button svg {
          width: 20px;
          height: 20px;
        }
      </style>
      <button type="button" part="button"></button>
    `;
    this.shadowRoot
      .querySelector('button')
      .addEventListener('click', () => this.toggle());
    this._updateIcon();
  }
}

customElements.define('hyperlane-theme-toggle', HyperlaneThemeToggle);
