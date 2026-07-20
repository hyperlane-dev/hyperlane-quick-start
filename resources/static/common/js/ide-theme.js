window.IDE_THEME_NAME = 'ltpp-theme';
// Light editor background — matches the `--theme_bk_color` in ide-theme.css
// so the Monaco surface blends with the surrounding app chrome.
window.IDE_LIGHT_BACKGROUND = '#ebeef5';
// Dark editor background — only used when `<html data-theme="dark">` is set.
// Currently the project forces light via theme-init.js, but keeping this
// distinct from the light value means flipping the theme script is a clean,
// one-line change.
window.IDE_DARK_BACKGROUND = '#1e1e1e';

window.is_dark_theme = false;

window.getCSSVariable = function (varName) {
  return getComputedStyle(document.documentElement)
    .getPropertyValue(varName)
    .replace(/!important\s*$/i, '')
    .trim();
};

window.matchMediaDark = function () {
  return false;
};

// Canonical list of IDE token-color rules. Exposed on `window` so other
// language modules (e.g. euv_language.js) can compose their extra rules
// on top of this same set without replacing the IDE's tokens.
window.IDE_THEME_TOKEN_RULES = [
  { token: 'keyword', foreground: () => window.getCSSVariable('--mtk1') },
  { token: 'keyword.main', foreground: () => window.getCSSVariable('--mtk14') },
  { token: 'function', foreground: () => window.getCSSVariable('--mtk8') },
  { token: 'type.keyword', foreground: () => window.getCSSVariable('--mtk4') },
  {
    token: 'type.identifier',
    foreground: () => window.getCSSVariable('--mtk5'),
  },
  { token: 'identifier', foreground: () => window.getCSSVariable('--mtk2') },
  { token: 'string', foreground: () => window.getCSSVariable('--mtk6') },
  { token: 'string.quote', foreground: () => window.getCSSVariable('--mtk22') },
  {
    token: 'string.escape',
    foreground: () => window.getCSSVariable('--mtk15'),
  },
  {
    token: 'string.invalid',
    foreground: () => window.getCSSVariable('--mtk10'),
  },
  { token: 'number', foreground: () => window.getCSSVariable('--mtk16') },
  { token: 'number.float', foreground: () => window.getCSSVariable('--mtk17') },
  { token: 'number.hex', foreground: () => window.getCSSVariable('--mtk18') },
  {
    token: 'comment',
    foreground: () => window.getCSSVariable('--mtk7'),
    fontStyle: 'italic',
  },
  { token: 'operator', foreground: () => window.getCSSVariable('--mtk21') },
  { token: 'delimiter', foreground: () => window.getCSSVariable('--mtk23') },
  { token: 'annotation', foreground: () => window.getCSSVariable('--mtk9') },
  { token: 'preprocessor', foreground: () => window.getCSSVariable('--mtk11') },
  {
    token: 'number.binary',
    foreground: () => window.getCSSVariable('--mtk12'),
  },
  { token: 'number.octal', foreground: () => window.getCSSVariable('--mtk13') },
  {
    token: 'delimiter.bracket',
    foreground: () => window.getCSSVariable('--mtk19'),
  },
];

window.IDE_THEME_TOKEN_RULES_RESOLVED = function () {
  return window.IDE_THEME_TOKEN_RULES.map(function (r) {
    var v = typeof r.foreground === 'function' ? r.foreground() : r.foreground;
    return {
      token: r.token,
      foreground: v || undefined,
      fontStyle: r.fontStyle,
    };
  });
};

window.defineIdeTheme = function (monaco, force) {
  if (!monaco || !monaco.editor) return;
  if (monaco.editor.__ideThemeDefined && !force) return;
  monaco.editor.__ideThemeDefined = true;
  monaco.editor.defineTheme(window.IDE_THEME_NAME, {
    base: 'vs',
    inherit: true,
    rules: window.IDE_THEME_TOKEN_RULES_RESOLVED(),
    colors: {
      'editor.background': window.IDE_LIGHT_BACKGROUND,
    },
  });
};

window.applyIdeTheme = function (editor) {
  if (!editor || !window.monaco) return;
  window.defineIdeTheme(window.monaco);
  window.monaco.editor.setTheme(window.IDE_THEME_NAME);
};

(function reapplyIdeThemeAfterStyles() {
  function reapply() {
    if (!window.monaco || !window.monaco.editor) return;
    try {
      window.defineIdeTheme(window.monaco, true);
      window.monaco.editor.setTheme(window.IDE_THEME_NAME);
    } catch (err) {}
  }
  if (document.readyState === 'complete') {
    reapply();
  } else {
    window.addEventListener('load', reapply);
  }
})();
