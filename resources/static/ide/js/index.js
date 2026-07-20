window.editor = null;
window.is_dark_theme = false;
window.copy_msg = '';
const show_display_css = 'block';
const hidden_display_css = 'none';
const app_dom_id = 'app';
const dialog_dom_id = 'dialog';
const dialog_overlay_dom_id = 'dialog-overlay';
const localstorage_code_key_prefix = '[code]';
const language_param_key = 'language';
const app_dom = document.getElementById(app_dom_id);
const dialog_dom = document.getElementById(dialog_dom_id);
const dialog_overlay_dom = document.getElementById(dialog_overlay_dom_id);
const default_language = 'cpp';
const s_keycode = 83;

const language = {
  c: 'C',
  cpp: 'C++',
  php: 'PHP',
  java: 'Java',
  javascript: 'JavaScript',
  typescript: 'TypeScript',
  ruby: 'Ruby',
  rust: 'Rust',
  python: 'Python3',
  go: 'Go',
  csharp: 'C#',
};

const code_tips = {
  [language.c]: window.c_tips,
  [language.cpp]: window.cpp_tips,
  [language.csharp]: window.csharp_tips,
  [language.go]: window.golang_tips,
  [language.java]: window.java_tips,
  [language.javascript]: window.javascript_tips,
  [language.php]: window.php_tips,
  [language.python]: window.python_tips,
  [language.ruby]: window.ruby_tips,
  [language.rust]: window.rust_tips,
  [language.typescript]: window.typescript_tips,
};

const language_map = {
  c: language.c,
  'c++': language.cpp,
  cpp: language.cpp,
  rs: language.rust,
  rust: language.rust,
  php: language.php,
  inc: language.php,
  java: language.java,
  js: language.javascript,
  javascript: language.javascript,
  node: language.javascript,
  nodejs: language.javascript,
  typescript: language.typescript,
  ts: language.typescript,
  golang: language.go,
  go: language.go,
  py: language.python,
  python: language.python,
  python2: language.python,
  python3: language.python,
  rusthon: language.python,
  ruby: language.ruby,
  jruby: language.ruby,
  macruby: language.ruby,
  rake: language.ruby,
  rb: language.ruby,
  rbx: language.ruby,
  csharp: language.csharp,
  'c#': language.csharp,
};

function addLanguageMap() {
  for (const key in language) {
    if (Object.hasOwnProperty.call(language, key)) {
      const value = language[key];
      language_map[key] = value;
    }
  }
}

function getMdLanguageMap() {
  const md_list = {};
  for (const key in language) {
    if (Object.hasOwnProperty.call(language, key)) {
      const value = language[key];
      md_list[value] = key;
    }
  }
  return md_list;
}

function getMdCodeTips() {
  const md_list = getMdLanguageMap();
  for (const key in code_tips) {
    if (Object.hasOwnProperty.call(code_tips, key)) {
      const value = code_tips[key];
      if (!value) {
        code_tips[key] = [];
      }
    }
  }
  const res = {};
  for (const key in md_list) {
    if (Object.hasOwnProperty.call(md_list, key)) {
      res[md_list[key]] = code_tips[key];
    }
  }
  return res;
}

function languageToMd(param_language = '') {
  if (param_language.toLowerCase) {
    param_language = param_language.toLowerCase();
  }
  const md_list = getMdLanguageMap();
  const sign = md_list[language_map[param_language]];
  return sign;
}

function getThemeLabel() {
  return 'light';
}

function isMobileDevice() {
  return (
    /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
      navigator.userAgent,
    ) ||
    'ontouchstart' in window ||
    navigator.maxTouchPoints > 0
  );
}

function showCopyTip(message = '复制成功') {
  const tip = document.createElement('div');
  const isSuccess = message.includes('成功');
  const icon = document.createElement('span');
  icon.style.cssText = `
    display: inline-block;
    font-size: 14px;
    font-weight: normal;
    opacity: 0.9;
  `;
  const text = document.createElement('span');
  text.innerText = message;
  text.style.cssText = `
    display: inline-block;
    vertical-align: middle;
  `;
  tip.appendChild(icon);
  tip.appendChild(text);
  const backgroundColor = isSuccess
    ? 'linear-gradient(135deg, rgba(76, 175, 80, 0.9) 0%, rgba(69, 160, 73, 0.9) 100%)'
    : 'linear-gradient(135deg, rgba(244, 67, 54, 0.9) 0%, rgba(211, 47, 47, 0.9) 100%)';
  const shadowColor = isSuccess
    ? 'rgba(76, 175, 80, 0.2)'
    : 'rgba(244, 67, 54, 0.2)';
  tip.style.cssText = `
    position: fixed;
    top: 32px;
    left: 50%;
    transform: translateX(-50%);
    background: ${backgroundColor};
    color: white;
    padding: 8px 12px;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 400;
    z-index: 10000;
    pointer-events: none;
    opacity: 0;
    transition: all 0.25s ease-out;
    box-shadow: 0 3px 12px ${shadowColor},
                0 1px 4px rgba(0, 0, 0, 0.08),
                inset 0 1px 0 rgba(255, 255, 255, 0.15);
    white-space: nowrap;
    user-select: none;
    -webkit-user-select: none;
    -moz-user-select: none;
    -ms-user-select: none;
    -webkit-touch-callout: none;
    -webkit-tap-highlight-color: transparent;
    backdrop-filter: blur(6px);
    -webkit-backdrop-filter: blur(6px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  `;
  document.body.appendChild(tip);
  setTimeout(() => {
    tip.style.opacity = '0.85';
    tip.style.transform = 'translateX(-50%)';
  }, 50);
  setTimeout(() => {
    tip.style.opacity = '0';
    tip.style.transform = 'translateX(-50%)';
    setTimeout(() => {
      if (tip.parentNode) {
        document.body.removeChild(tip);
      }
    }, 250);
  }, 888);
}

async function copyText(value = window.copy_msg, showTip = false) {
  try {
    if (navigator.clipboard) {
      await navigator.clipboard.writeText(value);
      if (showTip) {
        showCopyTip('复制成功');
      }
      return;
    }
    const text_area = document.createElement('textarea');
    text_area.value = value;
    text_area.style.width = 0;
    text_area.style.height = 0;
    text_area.style.display = hidden_display_css;
    text_area.setAttribute('readonly', 'readonly');
    document.body.appendChild(text_area);
    text_area.select();
    document.execCommand('copy');
    document.body.removeChild(text_area);
    if (showTip) {
      showCopyTip('复制成功');
    }
  } catch (err) {
    if (showTip) {
      showCopyTip('复制失败');
    }
  }
}

function showDialog(msg = '') {
  try {
    if (!dialog_dom) {
      return;
    }
    if (!msg) {
      msg = '\n';
    }
    msg = msg.toString();
    forceResetAllDialogs();
    dialog_dom.innerText = msg;
    if (dialog_overlay_dom) {
      dialog_overlay_dom.style.display = show_display_css;
      dialog_overlay_dom.offsetHeight;
      dialog_overlay_dom.classList.add('show');
    }
    dialog_dom.style.display = show_display_css;
    dialog_dom.offsetHeight;
    dialog_dom.style.animation =
      'dialogFadeIn 0.3s cubic-bezier(0.4, 0, 0.2, 1) forwards';
  } catch (err) {}
}

let dialogHiding = false;
let dialogHideTimeout = null;

function resetDialogState() {
  if (dialogHideTimeout) {
    clearTimeout(dialogHideTimeout);
    dialogHideTimeout = null;
  }
  dialogHiding = false;
  if (dialog_dom) {
    dialog_dom.style.animation = '';
  }
  if (dialog_overlay_dom) {
    dialog_overlay_dom.classList.remove('show');
    dialog_overlay_dom.style.display = hidden_display_css;
  }
}

function forceResetAllDialogs() {
  resetDialogState();
  if (dialog_dom) {
    dialog_dom.style.display = hidden_display_css;
    dialog_dom.style.animation = '';
    dialog_dom.innerText = '';
  }
  if (dialog_overlay_dom) {
    dialog_overlay_dom.style.display = hidden_display_css;
    dialog_overlay_dom.classList.remove('show');
  }
}

function hiddenDialog() {
  if (!dialog_dom) {
    return;
  }
  if (dialog_dom.style.display === hidden_display_css) {
    return;
  }
  if (dialogHideTimeout) {
    clearTimeout(dialogHideTimeout);
  }
  dialogHiding = true;
  if (dialog_overlay_dom) {
    dialog_overlay_dom.classList.remove('show');
  }
  dialog_dom.style.animation =
    'dialogFadeOut 0.2s cubic-bezier(0.4, 0, 0.2, 1) forwards';
  dialogHideTimeout = setTimeout(() => {
    if (dialog_dom) {
      dialog_dom.innerText = '';
      dialog_dom.style.display = hidden_display_css;
      dialog_dom.style.animation = '';
    }
    if (dialog_overlay_dom) {
      dialog_overlay_dom.style.display = hidden_display_css;
    }
    dialogHiding = false;
    dialogHideTimeout = null;
  }, 200);
}

function keydownListener() {
  document.addEventListener('keydown', function (e) {
    const keycode = e.keyCode;
    if ((e.ctrlKey || e.metaKey) && keycode === s_keycode) {
      e.preventDefault();
      return;
    }
  });
}

function getLocalStorageCode(language = window.language) {
  try {
    if (!language) {
      return '';
    }
    const key = `${localstorage_code_key_prefix}${language}`;
    return window.localStorage.getItem(key) || '';
  } catch (err) {}
  return '';
}

function setLocalStorageCode(language = window.language, code) {
  try {
    if (!language) {
      return;
    }
    if (!code) {
      code = '';
    }
    const key = `${localstorage_code_key_prefix}${language}`;
    window.localStorage.setItem(key, code);
  } catch (err) {}
}

function clickListener() {
  document.addEventListener('click', function (e) {
    if (dialog_overlay_dom && e.target === dialog_overlay_dom) {
      hiddenDialog();
      return;
    }
    if (
      dialog_dom &&
      e.target != dialog_dom &&
      !dialog_dom.contains(e.target)
    ) {
      hiddenDialog();
    }
  });
}

function dblclickListener() {
  document.addEventListener('dblclick', function (e) {
    if (dialog_dom && e.target === dialog_dom) {
      copyText(window.copy_msg, true);
      return;
    }
  });
}

function touchListener() {
  let touchStartTime = 0;
  let touchCount = 0;
  let touchTimer = null;
  let lastTouchTarget = null;
  document.addEventListener(
    'touchstart',
    function (e) {
      if (
        dialog_dom &&
        (e.target === dialog_dom || dialog_dom.contains(e.target))
      ) {
        const currentTime = Date.now();
        const currentTarget = e.target;
        if (touchTimer) {
          clearTimeout(touchTimer);
          touchTimer = null;
        }
        if (
          currentTime - touchStartTime < 400 &&
          currentTarget === lastTouchTarget
        ) {
          touchCount++;
          if (touchCount >= 2) {
            e.preventDefault();
            e.stopPropagation();
            copyText(window.copy_msg, true);
            touchCount = 0;
            lastTouchTarget = null;
            return;
          }
        } else {
          touchCount = 1;
        }
        touchStartTime = currentTime;
        lastTouchTarget = currentTarget;
        touchTimer = setTimeout(() => {
          touchCount = 0;
          lastTouchTarget = null;
        }, 400);
      }
    },
    { passive: false },
  );
  document.addEventListener(
    'touchend',
    function (e) {
      if (
        dialog_dom &&
        (e.target === dialog_dom || dialog_dom.contains(e.target))
      ) {
        if (touchCount >= 1) {
          e.preventDefault();
        }
      }
    },
    { passive: false },
  );
}

function errorListener() {
  try {
    window.addEventListener('error', function (event) {
      try {
        event.preventDefault();
        const error = (event.message ? event.message : '') || '';
        if (error) {
          window.copy_msg = error;
          showDialog(`【错误】\n${error}`);
        }
      } catch (err) {}
    });
    window.addEventListener('unhandledrejection', function (event) {
      try {
        event.preventDefault();
        const error = (event.reason ? event.reason.toString() : '') || '';
        if (error) {
          window.copy_msg = error;
          showDialog(`【错误】\n${error}`);
        }
      } catch (err) {}
    });
  } catch (err) {}
}

function getURLParameter(name = language_param_key) {
  let res = '';
  try {
    if (!name) {
      return false;
    }
    const regex = new RegExp('[?&]' + name + '(=([^&#]*)|&|#|$)');
    const results = regex.exec(window.location.href);
    if (!results || !results[2]) {
      return false;
    }
    res = decodeURIComponent(results[2]);
  } catch (err) {}
  return res;
}

function getLanguage() {
  addLanguageMap();
  const param = languageToMd(getURLParameter());
  if (!param || !language_map[param]) {
    changeLanguage(default_language);
  }
  window.language = param;
  return param;
}

function changeLanguage(language = window.language) {
  const location = window.location;
  let new_url = `${location.origin}${location.pathname}?${language_param_key}=${language} `;
  window.location.href = new_url;
}

function mdLanguageList() {
  const res = [];
  for (const key in language) {
    if (Object.hasOwnProperty.call(language, key)) {
      res.push(language[key]);
    }
  }
  return res;
}

function resizeListener() {
  window.addEventListener('resize', function () {
    if (window.editor) {
      window.editor.layout();
    } else {
      window.location.reload();
    }
  });
}

function loadIDE() {
  const code = getLocalStorageCode(window.language);
  require.config({ paths: { vs: 'min/vs' } });
  require(['vs/editor/editor.main'], function () {
    try {
      if (window.editor) {
        return;
      }
      window.loadLanguagesConfig(monaco);
      if (typeof window.defineIdeTheme === 'function') {
        window.defineIdeTheme(monaco);
        window.editor = monaco.editor.create(
          document.getElementById('ltpp-editor'),
          {
            value: code,
            language: window.language,
            theme: window.IDE_THEME_NAME,
            fontSize: 16,
            scrollBeyondLastLine: true,
            smoothScrolling: true,
            links: true,
            cursorSmoothCaretAnimation: true,
            readOnly: false,
            folding: true,
            contextmenu: false,
            suggestOnTriggerCharacters: true,
            cursorBlinking: 'smooth',
            cursorWidth: 2,
            automaticLayout: false,
            mouseWheelZoom: true,
            scrollBeyondLastLine: false,
            wordWrap: 'off',
            wrappingStrategy: 'advanced',
            scrollbar: {
              verticalScrollbarSize: 0,
              vertical: 'hidden',
              horizontalSliderSize: 8,
              horizontal: 'auto',
            },
          },
        );
        monaco.editor.setTheme(window.IDE_THEME_NAME);
      } else {
        const ltpp_theme_name = window.IDE_THEME_NAME || 'ltpp-theme';
        const ltpp_light_bk_color = window.IDE_LIGHT_BACKGROUND || '#ebeef5';
        const ltpp_dark_bk_color = window.IDE_DARK_BACKGROUND || '#282c34';
        monaco.editor.defineTheme(ltpp_theme_name, {
          base: window.is_dark_theme ? 'vs-dark' : 'vs',
          inherit: true,
          rules: [
            {
              background: window.is_dark_theme
                ? ltpp_dark_bk_color
                : ltpp_light_bk_color,
            },
            {
              token: 'keyword',
              foreground: window.getCSSVariable('--mtk1'),
            },
            {
              token: 'keyword.main',
              foreground: window.getCSSVariable('--mtk14'),
            },
            {
              token: 'function',
              foreground: window.getCSSVariable('--mtk8'),
            },
            {
              token: 'type.keyword',
              foreground: window.getCSSVariable('--mtk4'),
            },
            {
              token: 'type.identifier',
              foreground: window.getCSSVariable('--mtk5'),
            },
            {
              token: 'identifier',
              foreground: window.getCSSVariable('--mtk2'),
            },
            {
              token: 'string',
              foreground: window.getCSSVariable('--mtk6'),
            },
            {
              token: 'string.quote',
              foreground: window.getCSSVariable('--mtk22'),
            },
            {
              token: 'string.escape',
              foreground: window.getCSSVariable('--mtk15'),
            },
            {
              token: 'string.invalid',
              foreground: window.getCSSVariable('--mtk10'),
            },
            {
              token: 'number',
              foreground: window.getCSSVariable('--mtk16'),
            },
            {
              token: 'number.float',
              foreground: window.getCSSVariable('--mtk17'),
            },
            {
              token: 'number.hex',
              foreground: window.getCSSVariable('--mtk18'),
            },
            {
              token: 'comment',
              foreground: window.getCSSVariable('--mtk7'),
              fontStyle: 'italic',
            },
            {
              token: 'operator',
              foreground: window.getCSSVariable('--mtk21'),
            },
            {
              token: 'delimiter',
              foreground: window.getCSSVariable('--mtk23'),
            },
            {
              token: 'annotation',
              foreground: window.getCSSVariable('--mtk9'),
            },
            {
              token: 'preprocessor',
              foreground: window.getCSSVariable('--mtk11'),
            },
            {
              token: 'number.binary',
              foreground: window.getCSSVariable('--mtk12'),
            },
            {
              token: 'number.octal',
              foreground: window.getCSSVariable('--mtk13'),
            },
            {
              token: 'delimiter.bracket',
              foreground: window.getCSSVariable('--mtk19'),
            },
          ],
          colors: {
            'editor.background': window.is_dark_theme
              ? ltpp_dark_bk_color
              : ltpp_light_bk_color,
          },
        });
        window.editor = monaco.editor.create(
          document.getElementById('ltpp-editor'),
          {
            value: code,
            language: window.language,
            theme: ltpp_theme_name,
            fontSize: 16,
            scrollBeyondLastLine: true,
            smoothScrolling: true,
            links: true,
            cursorSmoothCaretAnimation: true,
            readOnly: false,
            folding: true,
            contextmenu: false,
            suggestOnTriggerCharacters: true,
            cursorBlinking: 'smooth',
            cursorWidth: 2,
            automaticLayout: false,
            mouseWheelZoom: true,
            scrollBeyondLastLine: false,
            wordWrap: 'off',
            wrappingStrategy: 'advanced',
            scrollbar: {
              verticalScrollbarSize: 0,
              vertical: 'hidden',
              horizontalSliderSize: 8,
              horizontal: 'auto',
            },
          },
        );
      }
      const new_code_tips = getMdCodeTips();
      monaco.languages.registerCompletionItemProvider(window.language, {
        provideCompletionItems: function (model, position) {
          const suggestions_list = [];
          if (new_code_tips[window.language]) {
            new_code_tips[window.language].forEach(function (tem) {
              suggestions_list.push({
                label: tem,
                kind: monaco.languages.CompletionItemKind.Text,
                insertText: tem,
                range: {
                  startLineNumber: position.lineNumber,
                  endLineNumber: position.lineNumber,
                  startColumn: position.column,
                  endColumn: position.column,
                },
              });
            });
          }
          const inputText = model.getValueInRange({
            startLineNumber: position.lineNumber,
            endLineNumber: position.lineNumber,
            startColumn: 1,
            endColumn: position.column,
          });
          const filtered_suggestions = new_code_tips[window.language]
            .filter(function (tip) {
              return tip.startsWith(inputText);
            })
            .map(function (tip) {
              return {
                label: tip,
                kind: monaco.languages.CompletionItemKind.Text,
                insertText: tip,
                range: {
                  startLineNumber: position.lineNumber,
                  endLineNumber: position.lineNumber,
                  startColumn: 1,
                  endColumn: position.column,
                },
              };
            });
          return {
            suggestions: filtered_suggestions,
          };
        },
      });
      window.editor = editor;
      editor.onDidChangeModelContent(function (event) {
        const code = editor.getValue();
        setLocalStorageCode(window.language, code);
      });
    } catch (err) {}
  });
}

function init() {
  try {
    window.is_dark_theme = false;
    loadIDE(window.is_dark_theme);
  } catch (err) {}
}

const ide_run_lang_label = document.getElementById('ide-run-lang-label');
if (ide_run_lang_label && window.language) {
  ide_run_lang_label.textContent = window.language;
}

errorListener();
clickListener();
dblclickListener();
touchListener();
keydownListener();
resizeListener();
getLanguage();
init();
