const IDE_EDITOR_OPTIONS_BASE = {
  fontSize: 16,
  scrollBeyondLastLine: false,
  smoothScrolling: true,
  links: true,
  cursorSmoothCaretAnimation: true,
  folding: true,
  contextmenu: true,
  suggestOnTriggerCharacters: true,
  cursorBlinking: 'smooth',
  cursorWidth: 2,
  automaticLayout: true,
  mouseWheelZoom: true,
  wordWrap: 'off',
  wrappingStrategy: 'advanced',
  minimap: { enabled: false },
  insertSpaces: true,
  renderLineHighlight: 'all',
  scrollbar: {
    verticalScrollbarSize: 8,
    horizontalSliderSize: 8,
    vertical: 'auto',
    horizontal: 'auto',
  },
};

class HyperlaneMonacoEditor extends HTMLElement {
  static get observedAttributes() {
    return ['value', 'language', 'readonly', 'tabsize', 'dark', 'font-size'];
  }

  constructor() {
    super();
    this._editor = null;
    this._loaderReady = null;
    this._onChangeCallbacks = [];
    this._mediaQuery = null;
    this._mediaListener = null;
    this._container = document.createElement('div');
    this._container.style.width = '100%';
    this._container.style.height = '100%';
  }

  connectedCallback() {
    this.appendChild(this._container);
    this._setupSystemThemeListener();
    this._mount();
  }

  disconnectedCallback() {
    if (this._mediaListener) {
      try {
        this._mediaQuery.removeEventListener('change', this._mediaListener);
      } catch (err) {}
      this._mediaQuery = null;
      this._mediaListener = null;
    }
    if (this._themeObserver) {
      try {
        this._themeObserver.disconnect();
      } catch (err) {}
      this._themeObserver = null;
    }
    if (this._editor) {
      this._editor.dispose();
      this._editor = null;
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (oldValue === newValue) return;
    if (!this._editor) return;
    if (name === 'value') {
      if (this._editor.getValue() !== newValue) {
        this._editor.setValue(newValue || '');
      }
    } else if (name === 'language') {
      const model = this._editor.getModel();
      if (model) {
        window.monaco.editor.setModelLanguage(model, newValue || 'plaintext');
      }
    } else if (name === 'readonly') {
      this._editor.updateOptions({ readOnly: newValue !== null });
    } else if (name === 'tabsize') {
      this._editor.updateOptions({ tabSize: parseInt(newValue) || 4 });
    } else if (name === 'font-size') {
      const v = parseInt(newValue);
      this._editor.updateOptions({ fontSize: isNaN(v) ? 16 : v });
    } else if (name === 'dark') {
      this._applyTheme(newValue !== 'false');
    }
  }

  get value() {
    return this._editor
      ? this._editor.getValue()
      : this.getAttribute('value') || '';
  }

  set value(v) {
    this.setAttribute('value', v);
  }

  get language() {
    return this.getAttribute('language') || 'plaintext';
  }

  get readonly() {
    return this.hasAttribute('readonly');
  }

  get tabSize() {
    return parseInt(this.getAttribute('tabsize')) || 4;
  }

  get fontSize() {
    const v = parseInt(this.getAttribute('font-size'));
    return isNaN(v) ? 18 : v;
  }

  get dark() {
    if (this.hasAttribute('dark') && this.getAttribute('dark') !== 'auto') {
      return this.getAttribute('dark') !== 'false';
    }
    return this._systemPrefersDark();
  }

  onChange(cb) {
    this._onChangeCallbacks.push(cb);
  }

  focus() {
    if (this._editor) this._editor.focus();
  }

  reset(value) {
    const next = value == null ? '' : String(value);
    if (!this._editor) {
      // Editor not mounted yet (Monaco loader still resolving). Park the value
      // on the attribute; _mount() will pick it up via the constructor's
      // `value: this.getAttribute('value') || ''` option.
      this.setAttribute('value', next);
      return;
    }
    const lang = this.language || 'plaintext';
    let switched = false;
    try {
      if (
        window.monaco &&
        window.monaco.editor &&
        typeof window.monaco.editor.createModel === 'function' &&
        typeof this._editor.setModel === 'function'
      ) {
        // monaco.editor.createModel returns a fresh text model that does
        // NOT carry over the previous editor's undo stack or markers;
        // assigning it to this._editor mirrors what the IDE does on every
        // fresh monaco.editor.create() call.
        const m = window.monaco.editor.createModel(next, lang);
        this._editor.setModel(m);
        switched = true;
      }
    } catch (err) {}
    if (!switched) {
      try {
        this._editor.setValue(next);
      } catch (err) {}
    }
    try {
      this._editor.setPosition({ lineNumber: 1, column: 1 });
      this._editor.revealPosition({ lineNumber: 1, column: 1 });
    } catch (err) {}
    this.setAttribute('value', next);
  }

  _systemPrefersDark() {
    return false;
  }

  _setupSystemThemeListener() {
    if (typeof window.matchMedia !== 'function') return;
    if (this._mediaListener) return;
    this._mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    this._mediaListener = () => {};
    try {
      this._mediaQuery.addEventListener('change', this._mediaListener);
    } catch (err) {}
  }

  _resolveTheme() {
    return false;
  }

  _applyTheme(dark) {
    if (!window.monaco || !this._editor) return;
    if (typeof window.applyIdeTheme === 'function') {
      try {
        window.defineIdeTheme(window.monaco);
      } catch (err) {}
      // If the euv language was loaded, also append the euv-specific token
      // rules (tag, attribute.name, class.helper, function.call, keyword.macro,
      // etc.) so euv source gets distinct colors per category.
      try {
        if (typeof window.addEuvThemeRules === 'function') {
          window.addEuvThemeRules(window.monaco);
        }
      } catch (err) {}
      window.monaco.editor.setTheme(window.IDE_THEME_NAME || 'ltpp-theme');
      // Monaco's tokenStyleMap generation injects hardcoded `.mtkN` rules at
      // the end of <head>; those override `ide-theme.css`'s rules. Re-emit
      // them at the end of <body> using `var(--mtkNN)` so the page variables
      // win (later same-specificity `!important` rules beat earlier ones).
      this._reinjectMtkCss();
      return;
    }
    window.monaco.editor.setTheme(dark ? 'vs-dark' : 'vs');
  }

  // Emit `<style>` for `.mtk1..100 { color: var(--mtkN) !important }` after
  // Monaco has injected its hardcoded tokens. Re-runs on every theme change
  // because Monaco wipes its own <style> on setTheme and adds new ones.
  _reinjectMtkCss() {
    if (!this._editor) return;
    if (!window.__mtkCssInjected) {
      const style = document.createElement('style');
      style.id = 'hyperlane-monaco-mtk-overrides';
      const MAX = 100;
      let css = '';
      for (let i = 1; i <= MAX; i++) {
        css += `.mtk${i} { color: var(--mtk${i}) !important; }\n`;
      }
      style.textContent = css;
      document.body.appendChild(style);
      window.__mtkCssInjected = true;
    }
  }

  _ensureLoader() {
    if (this._loaderReady) return this._loaderReady;
    this._loaderReady = new Promise((resolve, reject) => {
      if (window.__hyperlaneMonacoLoaderReady) {
        resolve();
        return;
      }
      const existing = document.querySelector(
        'script[data-hyperlane-monaco-loader]',
      );
      if (existing) {
        existing.addEventListener('load', () => {
          window.__hyperlaneMonacoLoaderReady = true;
          resolve();
        });
        existing.addEventListener('error', reject);
        return;
      }
      const script = document.createElement('script');
      script.src = '/static/ide/min/vs/loader.js';
      script.async = true;
      script.dataset.hyperlaneMonacoLoader = '1';
      script.addEventListener('load', () => {
        window.__hyperlaneMonacoLoaderReady = true;
        resolve();
      });
      script.addEventListener('error', reject);
      document.head.appendChild(script);
    });
    return this._loaderReady;
  }

  _ensureMain() {
    return this._ensureLoader().then(
      () =>
        new Promise((resolve, reject) => {
          if (window.monaco) {
            resolve();
            return;
          }
          window.require.config({ paths: { vs: '/static/ide/min/vs' } });
          window.require(['vs/editor/editor.main'], resolve, reject);
        }),
    );
  }

  _mount() {
    this._ensureMain()
      .then(() => {
        if (!window.monaco) {
          this._container.textContent = 'Failed to load Monaco editor.';
          return;
        }
        if (typeof window.loadLanguagesConfig === 'function') {
          try {
            window.loadLanguagesConfig(window.monaco);
          } catch (err) {}
        }
        if (typeof window.defineIdeTheme === 'function') {
          try {
            window.defineIdeTheme(window.monaco);
          } catch (err) {}
        }
        this._editor = window.monaco.editor.create(this._container, {
          ...IDE_EDITOR_OPTIONS_BASE,
          value: this.getAttribute('value') || '',
          language: this.language,
          theme: window.IDE_THEME_NAME || 'ltpp-theme',
          fontSize: this.fontSize,
          readOnly: this.readonly,
          tabSize: this.tabSize,
        });
        this._editor.onDidChangeModelContent(() => {
          const v = this._editor.getValue();
          for (const cb of this._onChangeCallbacks) cb(v);
        });
        this._applyTheme(this.dark);
      })
      .catch((err) => {
        this._container.textContent =
          'Failed to initialize Monaco: ' +
          (err && err.message ? err.message : String(err));
      });
  }
}

if (!customElements.get('hyperlane-monaco-editor')) {
  customElements.define('hyperlane-monaco-editor', HyperlaneMonacoEditor);
}
