const IDE_EDITOR_OPTIONS_BASE = {
  fontSize: 18,
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
      this._editor.updateOptions({ fontSize: isNaN(v) ? 18 : v });
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

  _systemPrefersDark() {
    if (typeof window.matchMediaDark === 'function') {
      try {
        return window.matchMediaDark();
      } catch (err) {}
    }
    try {
      return window.matchMedia('(prefers-color-scheme: dark)').matches;
    } catch (err) {
      return false;
    }
  }

  _setupSystemThemeListener() {
    if (typeof window.matchMedia !== 'function') return;
    if (this._mediaQuery) return;
    this._mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
    this._mediaListener = () => {
      if (!this.hasAttribute('dark') || this.getAttribute('dark') === 'auto') {
        this._applyTheme(this._systemPrefersDark());
      }
    };
    try {
      this._mediaQuery.addEventListener('change', this._mediaListener);
    } catch (err) {}
  }

  _applyTheme(dark) {
    if (!window.monaco || !this._editor) return;
    if (typeof window.applyIdeTheme === 'function') {
      try {
        window.defineIdeTheme(window.monaco);
      } catch (err) {}
      window.monaco.editor.setTheme(window.IDE_THEME_NAME || 'ltpp-theme');
      return;
    }
    window.monaco.editor.setTheme(dark ? 'vs-dark' : 'vs');
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
