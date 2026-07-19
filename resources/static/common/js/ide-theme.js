window.IDE_THEME_NAME = 'ltpp-theme';
window.IDE_LIGHT_BACKGROUND = '#ebeef5';
window.IDE_DARK_BACKGROUND = '#282c34';

window.is_dark_theme =
  typeof window.matchMedia === 'function' &&
  window.matchMedia('(prefers-color-scheme: dark)').matches;

window.getCSSVariable = function (varName) {
  return getComputedStyle(document.documentElement)
    .getPropertyValue(varName)
    .trim();
};

window.matchMediaDark = function () {
  try {
    return window.matchMedia('(prefers-color-scheme: dark)').matches;
  } catch (err) {
    return false;
  }
};

window.defineIdeTheme = function (monaco) {
  if (!monaco || !monaco.editor || monaco.editor.__ideThemeDefined) return;
  monaco.editor.__ideThemeDefined = true;
  const dark = window.matchMediaDark();
  monaco.editor.defineTheme(window.IDE_THEME_NAME, {
    base: dark ? 'vs-dark' : 'vs',
    inherit: true,
    rules: [
      {
        background: dark
          ? window.IDE_DARK_BACKGROUND
          : window.IDE_LIGHT_BACKGROUND,
      },
      { token: 'keyword', foreground: window.getCSSVariable('--mtk1') },
      { token: 'keyword.main', foreground: window.getCSSVariable('--mtk14') },
      { token: 'function', foreground: window.getCSSVariable('--mtk8') },
      { token: 'type.keyword', foreground: window.getCSSVariable('--mtk4') },
      { token: 'type.identifier', foreground: window.getCSSVariable('--mtk5') },
      { token: 'identifier', foreground: window.getCSSVariable('--mtk2') },
      { token: 'string', foreground: window.getCSSVariable('--mtk6') },
      { token: 'string.quote', foreground: window.getCSSVariable('--mtk22') },
      { token: 'string.escape', foreground: window.getCSSVariable('--mtk15') },
      { token: 'string.invalid', foreground: window.getCSSVariable('--mtk10') },
      { token: 'number', foreground: window.getCSSVariable('--mtk16') },
      { token: 'number.float', foreground: window.getCSSVariable('--mtk17') },
      { token: 'number.hex', foreground: window.getCSSVariable('--mtk18') },
      {
        token: 'comment',
        foreground: window.getCSSVariable('--mtk7'),
        fontStyle: 'italic',
      },
      { token: 'operator', foreground: window.getCSSVariable('--mtk21') },
      { token: 'delimiter', foreground: window.getCSSVariable('--mtk23') },
      { token: 'annotation', foreground: window.getCSSVariable('--mtk9') },
      { token: 'preprocessor', foreground: window.getCSSVariable('--mtk11') },
      { token: 'number.binary', foreground: window.getCSSVariable('--mtk12') },
      { token: 'number.octal', foreground: window.getCSSVariable('--mtk13') },
      {
        token: 'delimiter.bracket',
        foreground: window.getCSSVariable('--mtk19'),
      },
    ],
    colors: {
      'editor.background': dark
        ? window.IDE_DARK_BACKGROUND
        : window.IDE_LIGHT_BACKGROUND,
    },
  });
};

window.applyIdeTheme = function (editor) {
  if (!editor || !window.monaco) return;
  window.defineIdeTheme(window.monaco);
  window.monaco.editor.setTheme(window.IDE_THEME_NAME);
};
