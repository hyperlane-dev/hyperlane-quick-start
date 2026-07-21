// euv language module for Monaco editor.
// Registers a brand-new 'euv' language (not alias of rust) with:
//   - euv-specific keywords  : html!, class!, computed!, watch!, vars!, var!, #[component]
//   - euv-specific types     : Signal, VirtualNode, Css, App, ...
//   - macro-aware tokenizer  : recognizes `html! { ... }` body as embedded DSL
//                              (tags, attrs, {Signal} placeholders), `class! { ... }`
//                              body as embedded CSS (properties, pseudo-elements,
//                              media queries), `var!(name)` calls, `c_xxx()` calls,
//                              `r#type` raw identifiers, `data_*` / `aria_*` /
//                              `on*` attribute conventions.
//   - completion provider    : 3 trigger characters (! : () for macros, attrs, calls)
//                              offering the full euv API surface from
//                              euv-standards SKILL.md.
//
// Safe to load BEFORE monaco: it just exposes window.loadEuvLanguage.
// Safe to load AFTER monaco: same entry point calls register* APIs.

(function () {
  // ---------- keyword sets ----------
  // euv macros (rust doesn't have these as keywords)
  const EUV_MACROS = [
    'html!',
    'class!',
    'computed!',
    'watch!',
    'vars!',
    'var!',
  ];
  // euv attribute macros / proc macros
  const EUV_ATTR_MACROS = ['component'];
  // euv control-flow keywords inside html! / match blocks
  const EUV_DSL_KEYWORDS = ['for', 'in', 'if', 'else', 'match', 'key'];
  // HTML attribute names used in the html sub-state. Combined with
  // EUV_EVENT_ATTRS at tokenizer setup time so the `cases` lookup has
  // a single comprehensive set.
  const EUV_HTML_ATTRS = [
    'class',
    'style',
    'key',
    'id',
    'href',
    'src',
    'alt',
    'title',
    'type',
    'value',
    'placeholder',
    'checked',
    'disabled',
    'hidden',
    'target',
    'rel',
    'role',
    'tabindex',
    'name',
    'for',
    'min',
    'max',
    'step',
    'pattern',
    'maxlength',
    'minlength',
    'required',
    'readonly',
    'autofocus',
    'autocomplete',
    'action',
    'method',
    'enctype',
    'accept',
    'multiple',
    'selected',
    'open',
    'controls',
    'loop',
    'muted',
    'preload',
    'poster',
    'crossorigin',
    'integrity',
    'defer',
    'charset',
    'content',
    'cols',
    'rows',
    'span',
    'datetime',
    'cite',
    'sandbox',
    'allow',
    'allowfullscreen',
    'formaction',
    'formenctype',
    'formmethod',
    'formnovalidate',
    'formtarget',
    'height',
    'width',
    'wrap',
    'default',
    'kind',
    'srcset',
    'sizes',
    'media',
    'loading',
    'decoding',
    'fetchpriority',
  ];
  // euv-specific event attribute names (curated from euv-standards §3.1 / §11)
  const EUV_EVENT_ATTRS = [
    'onclick',
    'ondblclick',
    'onmousedown',
    'onmouseup',
    'onmouseover',
    'onmouseout',
    'onmousemove',
    'onkeydown',
    'onkeyup',
    'onkeypress',
    'oninput',
    'onchange',
    'onsubmit',
    'onreset',
    'onfocus',
    'onblur',
    'onload',
    'onunload',
    'onresize',
    'onscroll',
    'oncontextmenu',
    'onpointerdown',
    'onpointerup',
    'onpointermove',
    'ontouchstart',
    'ontouchend',
    'ontouchmove',
    'oncopy',
    'oncut',
    'onpaste',
    'ondrag',
    'ondragstart',
    'ondragend',
    'ondrop',
    'onanimationstart',
    'onanimationend',
    'ontransitionend',
    'onhashchange',
  ];
  // HTML element names (curated subset — the most-used ones in euv examples)
  const EUV_HTML_TAGS = [
    'a',
    'abbr',
    'address',
    'article',
    'aside',
    'audio',
    'b',
    'blockquote',
    'body',
    'br',
    'button',
    'canvas',
    'caption',
    'cite',
    'code',
    'col',
    'colgroup',
    'data',
    'datalist',
    'dd',
    'del',
    'details',
    'dfn',
    'dialog',
    'div',
    'dl',
    'dt',
    'em',
    'embed',
    'fieldset',
    'figcaption',
    'figure',
    'footer',
    'form',
    'h1',
    'h2',
    'h3',
    'h4',
    'h5',
    'h6',
    'head',
    'header',
    'hgroup',
    'hr',
    'html',
    'i',
    'iframe',
    'img',
    'input',
    'ins',
    'kbd',
    'label',
    'legend',
    'li',
    'link',
    'main',
    'map',
    'mark',
    'menu',
    'meta',
    'meter',
    'nav',
    'noscript',
    'object',
    'ol',
    'optgroup',
    'option',
    'output',
    'p',
    'param',
    'picture',
    'pre',
    'progress',
    'q',
    'rp',
    'rt',
    'ruby',
    's',
    'samp',
    'script',
    'section',
    'select',
    'slot',
    'small',
    'source',
    'span',
    'strong',
    'style',
    'sub',
    'summary',
    'sup',
    'table',
    'tbody',
    'td',
    'template',
    'textarea',
    'tfoot',
    'th',
    'thead',
    'time',
    'title',
    'tr',
    'track',
    'u',
    'ul',
    'var',
    'video',
    'wbr',
  ];
  // standard rust reserved + future-reserved keywords (subset)
  const RUST_KEYWORDS = [
    'as',
    'async',
    'await',
    'break',
    'const',
    'continue',
    'crate',
    'dyn',
    'else',
    'enum',
    'extern',
    'false',
    'fn',
    'for',
    'if',
    'impl',
    'in',
    'let',
    'loop',
    'match',
    'mod',
    'move',
    'mut',
    'pub',
    'ref',
    'return',
    'self',
    'Self',
    'static',
    'struct',
    'super',
    'trait',
    'true',
    'type',
    'union',
    'unsafe',
    'use',
    'where',
    'while',
  ];
  // euv core types / API names — drawn from euv-standards §3-§13
  const EUV_TYPES = [
    'Signal',
    'VirtualNode',
    'VirtualNodeChildren',
    'Css',
    'Engine',
    'EngineHandle',
    'NativeEventHandler',
    'IntervalHandle',
    'HookContext',
    'SignalCell',
    'AutoValue',
    'Renderer',
    'WebGpuRenderer',
    'CanvasRenderer',
    'RenderConfig',
    'EngineConfig',
    'SchedulerHandle',
    'TickHandlerRc',
  ];
  // euv core functions / API surface
  const EUV_FUNCTIONS = [
    // App:: methods
    'App::mount',
    'App::mount_node',
    'App::mount_into',
    'App::use_signal',
    'App::use_cleanup',
    'App::use_window_event',
    'App::use_interval',
    'App::use_effect_once',
    'App::batch',
    'App::spawn',
    // Signal methods
    'Signal::create',
    'Signal::default',
    'Signal::auto_value',
    // Engine methods
    'Engine::run',
    'Engine::new_handle',
    // Event utilities
    'NativeEventHandler::create',
  ];
  // common class! name prefixes (from euv-ui-standards prefix taxonomy)
  const EUV_CLASS_PREFIXES = [
    'c_page',
    'c_app',
    'c_nav',
    'c_main',
    'c_container',
    'c_layout',
    'c_card',
    'c_button',
    'c_btn',
    'c_input',
    'c_select',
    'c_form',
    'c_modal',
    'c_dialog',
    'c_drawer',
    'c_tab',
    'c_tabs',
    'c_menu',
    'c_list',
    'c_item',
    'c_badge',
    'c_tag',
    'c_chip',
    'c_avatar',
    'c_alert',
    'c_toast',
    'c_loading',
    'c_spinner',
    'c_progress',
    'c_header',
    'c_footer',
    'c_sidebar',
    'c_toolbar',
    'c_title',
    'c_subtitle',
    'c_label',
    'c_text',
    'c_icon',
    'c_image',
    'c_thumb',
    'c_grid',
    'c_row',
    'c_col',
    'c_flex',
    'c_gap',
    'c_padding',
    'c_margin',
    'c_bg',
    'c_text_color',
    'c_border',
    'c_shadow',
    'c_radius',
    'c_transition',
    'c_hover',
    'c_focus',
    'c_active',
    'c_disabled',
    'c_hidden',
    'c_visible',
    'c_responsive',
    'c_mobile',
    'c_desktop',
    'c_dark',
    'c_light',
    'euv_button',
    'euv_card',
    'euv_input',
    'euv_checkbox',
    'euv_nav_item',
    'euv_badge',
    'euv_tag',
    'euv_alert',
    'euv_select',
    'euv_pagination',
    'euv_progress',
    'euv_loading',
    'euv_toast',
    'euv_modal',
    'euv_dropdown',
    'euv_tabs',
    'euv_home',
    'euv_header',
    'euv_footer',
    'euv_sidebar',
  ];
  // CSS property names (the ones most commonly used inside class! blocks)
  const CSS_PROPERTIES = [
    'display',
    'position',
    'top',
    'right',
    'bottom',
    'left',
    'z_index',
    'flex',
    'flex_direction',
    'flex_wrap',
    'flex_grow',
    'flex_shrink',
    'flex_basis',
    'justify_content',
    'align_items',
    'align_self',
    'align_content',
    'gap',
    'row_gap',
    'column_gap',
    'order',
    'grid',
    'grid_template',
    'grid_area',
    'grid_column',
    'grid_row',
    'width',
    'height',
    'min_width',
    'min_height',
    'max_width',
    'max_height',
    'margin',
    'margin_top',
    'margin_right',
    'margin_bottom',
    'margin_left',
    'padding',
    'padding_top',
    'padding_right',
    'padding_bottom',
    'padding_left',
    'border',
    'border_top',
    'border_right',
    'border_bottom',
    'border_left',
    'border_radius',
    'border_color',
    'border_style',
    'border_width',
    'background',
    'background_color',
    'background_image',
    'background_size',
    'background_position',
    'background_repeat',
    'color',
    'font',
    'font_size',
    'font_weight',
    'font_family',
    'font_style',
    'line_height',
    'letter_spacing',
    'text_align',
    'text_decoration',
    'text_transform',
    'text_overflow',
    'white_space',
    'word_break',
    'word_wrap',
    'opacity',
    'visibility',
    'overflow',
    'overflow_x',
    'overflow_y',
    'cursor',
    'pointer_events',
    'user_select',
    'box_shadow',
    'text_shadow',
    'filter',
    'backdrop_filter',
    'transition',
    'transition_property',
    'transition_duration',
    'transition_timing_function',
    'transition_delay',
    'animation',
    'animation_name',
    'animation_duration',
    'transform',
    'transform_origin',
  ];
  // CSS pseudo-class / pseudo-element names (curated)
  const CSS_PSEUDOS = [
    'hover',
    'focus',
    'focus_visible',
    'focus_within',
    'active',
    'visited',
    'link',
    'checked',
    'disabled',
    'enabled',
    'required',
    'optional',
    'valid',
    'invalid',
    'in_range',
    'out_of_range',
    'placeholder_shown',
    'default',
    'read_only',
    'empty',
    'target',
    'first_child',
    'last_child',
    'only_child',
    'first_of_type',
    'last_of_type',
    'only_of_type',
    'nth_child',
    'nth_of_type',
    'not',
    'is',
    'where',
    'before',
    'after',
    'first_letter',
    'first_line',
    'placeholder',
    'selection',
    'marker',
  ];
  // CSS at-rules / media-query variants
  const CSS_AT_RULES = ['media', 'supports', 'container'];

  // ---------- tokenizer ----------
  // Built on top of the same shape used by languages_config.js. The trick for
  // highlighting macro DSLs: when we see `html!` / `class!` etc., enter a
  // dedicated sub-state that uses its own rules, then pop back on the matching
  // closing brace at depth 0.
  function buildTokenizer(monaco) {
    const ID_RE = /[a-zA-Z_][a-zA-Z0-9_]*/;
    return {
      defaultToken: '',
      // Postfix intentionally omitted: the shared `ltpp-theme` defines rules
      // keyed by bare token names (`keyword`, `string`, `comment`, ...) — if we
      // append `.euv` here, Monaco emits `keyword.euv` and the theme rules
      // (which lack the postfix) don't match, so every token falls back to
      // the rule's `defaultToken` and renders in a single color. See
      // `common/css/ide-theme.css` `IDE_THEME_TOKEN_RULES` for the rule keys.
      keywords: RUST_KEYWORDS.concat(EUV_MACROS).concat(EUV_DSL_KEYWORDS),
      typeKeywords: EUV_TYPES,
      operators: [
        '=',
        '>',
        '<',
        '!',
        '~',
        '?',
        ':',
        '==',
        '<=',
        '>=',
        '!=',
        '&&',
        '||',
        '++',
        '--',
        '+',
        '-',
        '*',
        '/',
        '&',
        '|',
        '^',
        '%',
        '<<',
        '>>',
        '>>>',
        '+=',
        '-=',
        '*=',
        '/=',
        '&=',
        '|=',
        '^=',
        '%=',
        '<<=',
        '>>=',
        '>>>=',
      ],
      // Monarch in this Monaco build expects `symbols` (referenced as
      // `@symbols` in a regex rule) to be a list, but rejects the array form
      // with "language definition does not contain attribute 'symbols',
      // used at: ^(?:@symbols)". The combination breaks the whole tokenizer
      // — every token collapses to the rule's `defaultToken` and the editor
      // renders a single color (mtk1).
      //
      // Fix: keep `symbols` as the regex form, BUT replace every `/[=><!~?:&|+\-*/^%]+/`
      // rule below with the literal regex `/[=><!~?:&|+\-*/^%]+/`. Monarch
      // happily matches the inline regex and treats operators in `cases`
      // blocks (so `@operators` continues to work via the `operators` list).
      symbols: /[=><!~?:&|+\-*\/^%]+/,
      escapes:
        /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,
      tokenizer: {
        root: [
          // ===== euv macro invocations — match BEFORE the catch-all
          // identifier rule, otherwise `html` would match as `identifier`
          // and the `!` would be tokenized as `operator` separately. =====
          [/\bhtml!/, { token: 'keyword.macro', next: '@html_dispatch' }],
          [/\bclass!/, { token: 'keyword.macro', next: '@class_dispatch' }],
          [
            /\b(computed|watch)!/,
            { token: 'keyword.macro', next: '@expr_dispatch' },
          ],
          [/\bvars!/, { token: 'keyword.macro', next: '@vars_dispatch' }],
          [/\bvar!/, { token: 'keyword.macro', next: '@var_dispatch' }],
          // ===== identifier token type — coarse `keyword` + fine sub-types =====
          // euv attribute proc-macros: `#[component]`, `#[wasm_bindgen]`, etc.
          [/#!?\[[a-zA-Z_][\w]*/, 'annotation', '@annotation'],
          // rust raw identifier `r#name`
          [/r#[a-zA-Z_][\w$]*/, 'keyword.rawidentifier'],
          // rust lifetime `'a`
          [/'[a-zA-Z_][\w]*(?!')/, 'identifier'],
          // numbers
          [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
          [/0[xX][0-9a-fA-F]+/, 'number.hex'],
          [/0[bB][01]+/, 'number.binary'],
          [/0[oO][0-7]+/, 'number.octal'],
          [/\d+/, 'number'],
          // whitespace + comments
          { include: '@whitespace' },
          // brackets
          [/[{}()[\]]/, '@brackets'],
          // Keep path segments together and color the whole qualified name
          // consistently, including `App::mount` and `wasm_bindgen`. Written
          // WITHOUT a `(?:...)+` group because the bundled Monaco's Monarch
          // wraps every rule in `^(?:...)` and mishandles the nested
          // non-capturing group around `::`, splitting `App::mount` into
          // `App` / `::` / `mount` (and even `A` / `pp`). A flat alternation
          // of one-or-more `::segment` suffixes avoids that and matches the
          // whole path as a single token in every Monarch version.
          [/[A-Za-z_][A-Za-z0-9_]*(::[A-Za-z_][A-Za-z0-9_]*)+/, 'identifier'],
          [/[A-Za-z_][A-Za-z0-9_]*_[A-Za-z0-9_]+/, 'identifier'],
          // identifiers (keywords vs types vs plain)
          [
            /[a-zA-Z_][\w$]*/,
            {
              cases: {
                '@typeKeywords': 'type.keyword',
                '@keywords': 'keyword',
                '@default': 'identifier',
              },
            },
          ],
          // operators
          [
            /[=><!~?:&|+\-*/^%]+/,
            {
              cases: {
                '@operators': 'operator',
                '@default': '',
              },
            },
          ],
          // delimiters
          [/[;,.]/, 'delimiter'],
          // strings
          [/"([^"\\]|\\.)*$/, 'string.invalid'],
          [
            /"/,
            { token: 'string.quote', bracket: '@open', next: '@string_double' },
          ],
          [/'([^'\\]|\\.)*$/, 'string.invalid'],
          [
            /'/,
            { token: 'string.quote', bracket: '@open', next: '@string_single' },
          ],
        ],

        // ---------- dispatch on which macro we entered ----------
        // We captured `html!` / `class!` / `vars!` / `var!` in `root`. After
        // the macro name + `!`, we expect ` { ... }`. Skip whitespace and
        // brace, then hand off to the right sub-state.
        html_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\{/, { token: '@brackets', next: '@html' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        class_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\{/, { token: '@brackets', next: '@class_dsl' }],
          [/\}/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        expr_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\(/, { token: '@brackets', next: '@macro_call' }],
          [/\{/, { token: '@brackets', next: '@class_dsl' }],
          [/\)/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        vars_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\{/, { token: '@brackets', next: '@vars_dsl' }],
          [/\}/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        var_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\(/, { token: '@brackets', next: '@macro_call' }],
          [/\)/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        html_dispatch: [
          { include: '@whitespace' },
          [/[!]/, ''],
          { include: '@whitespace' },
          [/\{/, { token: '@brackets', next: '@html' }],
          [/\}/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        macro_dispatch: [
          { include: '@whitespace' },
          [/\{/, { token: '@brackets', next: '@macro_body' }],
          [/\}/, { token: '@brackets', next: '@pop' }],
          [/\(/, { token: '@brackets', next: '@macro_call' }],
          [/[a-zA-Z_][\w$]*/, 'identifier'],
          [/[;,.]/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],

        // `var!(name)` style call — name is a CSS variable identifier
        macro_call: [
          { include: '@whitespace' },
          [/[a-zA-Z_][\w-]*/, 'identifier'],
          [/\)/, { token: '@brackets', next: '@pop' }],
          [/,/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],

        // ---------- shared body entry — switch on the macro we came from ----------
        macro_body: [
          { include: '@whitespace' },
          // propagate nested braces (for style: { ... } blocks inside html!)
          [/\{/, { token: '@brackets', next: '@macro_nested' }],
          // pop when we close the outer block
          [/\}/, { token: '@brackets', next: '@pop' }],
          // anything else is delegated via sub-state includes below
          { include: 'html' },
          { include: 'class_dsl' },
          { include: 'vars_dsl' },
          // default fall-through
          [/[^]/, ''],
        ],

        // nested body (style: { ... } inside html!)
        macro_nested: [
          [/\{/, { token: '@brackets', next: '@macro_nested' }],
          [/\}/, { token: '@brackets', next: '@pop' }],
          { include: '@style_kv' },
        ],

        // ---------- html! body ----------
        html: [
          // element opening keyword: bare tag name or quoted tag
          // 0. CSS helper function call (c_xxx() / c_xxx(args)) -> class.helper.
          //     The pattern requires `c_` (lowercase c, then underscore)
          //     to match the euv CSS-helper convention. Snake_case names
          //     starting with a different letter (`add_event`, `use_signal`)
          //     fall through to the snake_case rule below and emit a single
          //     `identifier` token instead of being split on the underscore.
          [/c_[a-zA-Z0-9_]*\(\s*\)/, 'class.helper'],
          [/c_[a-zA-Z0-9_]+\s*\(/, 'class.helper'],
          [/c_[a-zA-Z0-9_]+/, 'class.helper'],
          // Keep complete snake_case identifiers intact. This must run before
          // the shorter tag/keyword matcher, otherwise `wasm_bindgen` is
          // split at `_` and the separator receives the editor default color.
          [/[a-zA-Z_][a-zA-Z0-9_]*_[a-zA-Z0-9_]+/, 'identifier'],
          [
            /[a-z][a-zA-Z0-9-]*/,
            {
              cases: {
                '@htmlTags': 'tag',
                '@dslKeywords': 'keyword.html',
                '@eventAttrs': 'attribute.name',
                '@default': 'identifier',
              },
            },
          ],
          [/"([a-z][a-zA-Z0-9-]*)"/, { token: 'tag' }],
          [
            /(r#)?(data_[a-zA-Z0-9_]+|aria_[a-zA-Z0-9_]+|on[a-z]+|class|style|key|id|href|src|alt|title|type|value|placeholder|checked|disabled|hidden|target|rel|role|tabindex|name|for|min|max|step|pattern|maxlength|minlength|required|readonly|autofocus|autocomplete|action|method|enctype|accept|multiple|selected|open|controls|loop|muted|preload|poster|crossorigin|integrity|defer|async|charset|content|cols|rows|span|datetime|cite|sandbox|allow|allowfullscreen|formaction|formenctype|formmethod|formnovalidate|formtarget|height|width|wrap|default|kind|srcset|sizes|media|loading|decoding|fetchpriority)\x3a/,
            'attribute.name',
          ],
          // dynamic signal/expr placeholders {expr}
          [/\{\s*\{/, { token: '@brackets', next: '@html_dyn_expr' }],
          [/\}/, { token: '@brackets' }],
          [/\{/, { token: '@brackets' }],
          // string literals (must come BEFORE any other rule that
          // could match a leading `"`).
          [/"([^"\\]|\\.)*"/, 'string'],
          [/'([^'\\]|\\.)*'/, 'string'],
          // numbers / operators / punctuation
          [/\d+/, 'number'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
          [/[;,.]/, 'delimiter'],
        ],

        // expression inside html! { ... } — supports `if { cond } { ... }`,
        // `match { val } { "a" => ... }`, `for x in { items }`, `class: if { ... }`
        html_dyn_expr: [
          { include: '@whitespace' },
          [
            /\b(if|match|for|in|else|key)\b/,
            {
              cases: {
                '@keywords': 'keyword',
                '@dslKeywords': 'keyword.html',
                '@default': 'identifier',
              },
            },
          ],
          [/\}/, { token: '@brackets', next: '@pop' }],
          [/\}/, { token: '@brackets' }],
          [/\{/, { token: '@brackets', next: '@macro_nested' }],
          { include: '@rust_expr' },
        ],

        // ---------- class! body (CSS-ish) ----------
        class_dsl: [
          // 0a. CSS helper function call (c_xxx() / c_xxx(args)) -> class.helper.
          //     The leading `c_` rule only matches identifiers that START with
          //     `c_` (the euv CSS-helper convention). Snake_case identifiers
          //     like `add_event` / `use_signal` / `count` fall through to
          //     the bare-identifier rule below and emit `identifier`.
          [/c_[a-zA-Z0-9_]*\(\s*\)/, 'class.helper'],
          [/c_[a-zA-Z0-9_]+\s*\(/, 'class.helper'],
          [/c_[a-zA-Z0-9_]+/, 'class.helper'],
          // 0b. CSS property/hyphenated-name token. Matches `flex`,
          //     `flex-direction`, `min-height`, etc. as a single token
          //     emitting `attribute.name` so CSS properties get a
          //     consistent blue color regardless of hyphens.
          [/[a-z][a-zA-Z-]*[a-zA-Z]/, 'attribute.name'],
          // class name declaration: `pub foo` / `pub(crate) bar` / `c_button { ... }`
          [
            /(pub\s*\(\s*(crate|super|self|self\s*\(\s*[a-zA-Z_][\w]*\s*\)\s*)\)\s+)?(pub\s+)?[a-z_][\w]*/,
            {
              cases: {
                '@cssProperties': 'attribute.name',
                '@pseudoNames': 'keyword.pseudo',
                '@atRules': 'keyword.directive',
                '@default': 'tag',
              },
            },
          ],
          // parameter list: `(name: &str)`
          [/\(/, { token: '@brackets', next: '@class_params' }],
          // value strings
          [/"([^"\\]|\\.)*"/, 'string'],
          [/'([^'\\]|\\.)*'/, 'string'],
          // numbers
          [
            /\d*\.?\d+(px|rem|em|%|vh|vw|s|ms|deg|rgb|rgba|hsl|hsla)?/,
            'number',
          ],
          // var!(name) — captured by trigger `!`
          [/\bvar!/, { token: 'keyword.macro', next: '@macro_call' }],
          // closing class_dsl block — pop back to dispatcher so subsequent
          // lines tokenize in root (Monarch otherwise stays in class_dsl
          // and the rest of the file incorrectly uses class_dsl rules such
          // as the hyphenated-CSS-property `attribute.name` rule that then
          // gobbles up keywords/identifiers downstream).
          [/\}/, { token: '@brackets', next: '@pop' }],
          // separators
          [/;/, 'delimiter'],
          [/,/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],
        class_params: [
          { include: '@whitespace' },
          [/[a-zA-Z_][\w]*\s*:/, 'attribute.name'],
          [/[&']?[a-zA-Z_][\w]*/, 'type.keyword'],
          [/\)/, { token: '@brackets', next: '@pop' }],
          [/,/, 'delimiter'],
        ],

        // ---------- vars! body (CSS variable definitions) ----------
        vars_dsl: [
          [/(pub\s+)?[a-zA-Z_][\w-]*/, 'attribute.name'],
          [/"([^"\\]|\\.)*"/, 'string'],
          [/,/, 'delimiter'],
          [/;/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],

        // ---------- shared style key:value pair (used inside style: { ... }) ----------
        style_kv: [
          { include: '@whitespace' },
          [
            /[a-zA-Z_][\w-]*/,
            {
              cases: {
                '@cssProperties': 'attribute.name',
                '@default': 'identifier',
              },
            },
          ],
          [/:/, 'delimiter'],
          [/"([^"\\]|\\.)*"/, 'string'],
          [/'([^'\\]|\\.)*'/, 'string'],
          [/var!/, { token: 'keyword.macro', next: '@macro_call' }],
          [
            /\d*\.?\d+(px|rem|em|%|vh|vw|s|ms|deg|rgb|rgba|hsl|hsla)?/,
            'number',
          ],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
          [/;/, 'delimiter'],
        ],

        // ---------- shared rust-expression inner block (used inside { ... } exprs) ----------
        rust_expr: [
          { include: '@whitespace' },
          [
            /[a-zA-Z_][\w$]*/,
            {
              cases: {
                '@typeKeywords': 'type.keyword',
                '@keywords': 'keyword',
                '@default': 'identifier',
              },
            },
          ],
          [/\d*\.\d+([eE][\-+]?\d+)?/, 'number.float'],
          [/\d+/, 'number'],
          [/"([^"\\]|\\.)*"/, 'string'],
          [/'([^'\\]|\\.)*'/, 'string'],
          [/[{}()[\]]/, '@brackets'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
          [/[;,.]/, 'delimiter'],
        ],

        // ---------- attribute proc-macro body ----------
        annotation: [
          [/\(/, { token: '@brackets', next: '@annotation_args' }],
          [/\]/, { token: '@brackets', next: '@pop' }],
          [/[a-zA-Z_][\w]*/, 'annotation'],
          [/,/, 'delimiter'],
        ],
        annotation_args: [
          { include: '@whitespace' },
          [/[a-zA-Z_][\w]*/, 'attribute.name'],
          [/"([^"\\]|\\.)*"/, 'string'],
          [/\)/, { token: '@brackets', next: '@popall' }],
          [/,/, 'delimiter'],
          [/[=><!~?:&|+\-*/^%]+/, 'operator'],
        ],

        // ---------- standard sub-states (must exist for setMonarchTokensProvider) ----------
        whitespace: [
          [/[ \t\r\n]+/, ''],
          [/\/\*/, 'comment', '@comment'],
          [/\/\/.*$/, 'comment'],
        ],
        comment: [
          [/[^/*]+/, 'comment'],
          [/\*\//, 'comment', '@pop'],
          [/[/*]/, 'comment'],
        ],
        string_double: [
          [/[^\\"]+/, 'string'],
          [/@escapes/, 'string.escape'],
          [/\\./, 'string.escape.invalid'],
          [/"/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
        ],
        string_single: [
          [/[^\\']+/, 'string'],
          [/@escapes/, 'string.escape'],
          [/\\./, 'string.escape.invalid'],
          [/'/, { token: 'string.quote', bracket: '@close', next: '@pop' }],
        ],
      },
    };
  }

  // ---------- theme rules (additions on top of ltpp-theme) ----------
  // The shared ltpp-theme in /static/common/js/ide-theme.js already maps most
  // token types; we only add euv-specific ones here.
  function addEuvThemeRules(monaco) {
    try {
      // IMPORTANT: monaco.editor.defineTheme replaces the entire theme.
      // The previous addEuvThemeRules only registered euv-specific token
      // rules, which meant the IDE's `keyword` / `string` / `comment` /
      // etc. rules were dropped after this call and the euv-playground
      // rendered text in plain black/white (Monaco's base vs editor.foreground).
      // Compose with the shared IDE rule list so we keep every token class
      // the IDE defines and only append euv-specific extras.
      var fg = function (name, fallback) {
        try {
          var v =
            typeof window.getCSSVariable === 'function'
              ? window.getCSSVariable(name)
              : '';
          return v && v.trim() ? v.trim() : fallback;
        } catch (e) {
          return fallback;
        }
      };
      var dark =
        typeof window.matchMediaDark === 'function'
          ? window.matchMediaDark()
          : false;
      var baseRules =
        typeof window.IDE_THEME_TOKEN_RULES_RESOLVED === 'function'
          ? window.IDE_THEME_TOKEN_RULES_RESOLVED()
          : [];
      var euvRules = [
        {
          token: 'keyword.macro',
          foreground: fg('--mtk14', 'bf360c'),
          fontStyle: 'bold',
        },
        { token: 'keyword.html', foreground: fg('--mtk24', '0d47a1') },
        // NOTE: identifier / type.keyword intentionally NOT overridden here.
        // ide-theme.css locks every `.mtkN` to `var(--mtkN) !important`, and
        // Monaco assigns the mtkN index by color *dedup* — so a hex that isn't
        // already in the palette lands on an unrelated slot (e.g. blue on
        // .mtk37) and gets recolored by that slot's variable. The base theme
        // (ide-theme.js) already maps identifier → --mtk2 and type.keyword →
        // --mtk4, both of which render correctly. A qualified path
        // (`App::mount`) is a single identifier token, so it and a bare `App`
        // share the base identifier color consistently.
        { token: 'keyword.rawidentifier', foreground: fg('--mtk15', '9c27b0') },
        { token: 'keyword.pseudo', foreground: fg('--mtk21', '455a64') },
        // CSS helper name (c_xxx) inside html! body. Distinct color from the
        // actual HTML tag (mtk25 red) so `div` and `c_euv_playground_root`
        // don't look identical in the rendered code.
        { token: 'class.helper', foreground: fg('--mtk27', '00695c') },
        // Function call at value position (use_signal(0)). Violet so it
        // reads as a method/event-handler reference, not a tag.
        { token: 'function.call', foreground: fg('--mtk28', '4527a0') },
        {
          token: 'keyword.directive',
          foreground: fg('--mtk29', 'bf360c'),
          fontStyle: 'bold',
        },
        { token: 'tag', foreground: fg('--mtk25', 'c62828') },
        { token: 'attribute.name', foreground: fg('--mtk26', '6d4c41') },
      ];
      // Theme-trie insert overwrites an existing token, so the LAST rule for a
      // given token wins. Putting euvRules after baseRules lets the euv
      // identifier/type.keyword colors override the base theme's --mtk2 mapping.
      var rules = baseRules.concat(euvRules);
      var bg = dark
        ? window.IDE_DARK_BACKGROUND || '#282c34'
        : window.IDE_LIGHT_BACKGROUND || '#ebeef5';
      var name = window.IDE_THEME_NAME || 'ltpp-theme';
      monaco.editor.defineTheme(name, {
        base: dark ? 'vs-dark' : 'vs',
        inherit: true,
        rules: rules,
        colors: { 'editor.background': bg },
      });
      // Re-apply the theme so the new rules take effect. Monaco only
      // refreshes the token table on setTheme; without this call the
      // editor keeps using the rules that were baked in when the
      // editor was first created (which doesn't include the euv
      // extras, and may not even include the IDE's keyword/string/
      // comment rules if defineIdeTheme was called first).
      //
      // We can't read the current theme name from
      // monaco.editor._themeService because that internal moves
      // between Monaco versions, so we just call setTheme
      // unconditionally and let Monaco keep the active theme if it
      // matches. This also handles the case where multiple editors on
      // the same page need to refresh.
      try {
        var editors = monaco.editor.getEditors && monaco.editor.getEditors();
        if (editors && editors.length) {
          // The editors are already using `name` (set by
          // hyperlane-monaco-editor._applyTheme), so this just
          // refreshes the token table on each.
          monaco.editor.setTheme(name);
        } else {
          // No editors yet; nothing to refresh. The editor will pick
          // up the rules when it calls setTheme(name) on its own
          // during _applyTheme.
        }
      } catch (e) {}
    } catch (err) {}
  }

  // ---------- completion provider ----------
  function buildCompletionProvider(monaco) {
    function snippet(label, body, detail, doc) {
      return {
        label,
        kind: monaco.languages.CompletionItemKind.Snippet,
        detail: detail || '',
        documentation: doc || '',
        insertText: body,
        insertTextRules:
          monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      };
    }
    function keyword(label, detail, doc) {
      return {
        label,
        kind: monaco.languages.CompletionItemKind.Keyword,
        detail: detail || '',
        documentation: doc || '',
        insertText: label,
      };
    }
    function typeKw(label, detail) {
      return {
        label,
        kind: monaco.languages.CompletionItemKind.Class,
        detail: detail || '',
        insertText: label,
      };
    }
    function method(label, snippetBody, detail, doc) {
      return {
        label,
        kind: monaco.languages.CompletionItemKind.Method,
        detail: detail || '',
        documentation: doc || '',
        insertText: snippetBody,
        insertTextRules:
          monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
      };
    }
    function attr(label, detail) {
      return {
        label,
        kind: monaco.languages.CompletionItemKind.Property,
        detail: detail || '',
        insertText: label,
      };
    }

    const macroSnippets = EUV_MACROS.map((m) =>
      m === 'html!'
        ? snippet(
            'html! { … }',
            'html! {\n    div { class: ${1:c_container()}\n        ${0:\"content\"}\n    }\n}',
            'euv macro — declarative HTML/JSX-like tree',
            'Build a VirtualNode tree. Tags are bare identifiers or strings; attributes use `attr: value`; class: accepts `c_*()` calls; children can be { Signal } placeholders, text, or nested elements.',
          )
        : m === 'class!'
          ? snippet(
              'class! { … }',
              'class! {\n    pub ${1:c_name} {\n        ${2:property}: \"${3:value}\";\n    }\n}',
              'euv macro — CSS class definition',
              'Define a class! block. Pub visibility is optional. Pseudo-class blocks (hover, focus, nth_child(n)) nest inside.',
            )
          : m === 'computed!'
            ? snippet(
                'computed! { … }',
                'computed!(${1:signal_a}, ${2:signal_b}, |${3:a_val}: ${4:Type}, ${5:b_val}: ${6:Type}| -> ${7:RetType} {\n    ${0:body}\n})',
                'euv macro — derive Signal<T> from other signals',
                'Auto-runs when any dependency signal changes. Return type must be explicit.',
              )
            : m === 'watch!'
              ? snippet(
                  'watch! { … }',
                  'watch!(${1:signal}, |${2:val}: ${3:Type}| {\n    ${0:body}\n});',
                  'euv macro — run side-effect on signal change',
                  'Fires once immediately, then on every change.',
                )
              : m === 'vars!'
                ? snippet(
                    'vars! { … }',
                    'vars! {\n    pub ${1:theme} { ${2:color_bg}: \"${3:#fff}\"; ${4:color_text}: \"${5:#000}\"; }\n}',
                    'euv macro — define CSS variable bundles',
                    'Defines CSS custom properties that other class! / var!() calls can reference.',
                  )
                : snippet(
                    'var!(name)',
                    'var!(${1:name})',
                    'euv macro — reference a CSS variable',
                    'Emits "var(--name)" at the call site.',
                  ),
    );

    const dslKeywords = EUV_DSL_KEYWORDS.map((k) =>
      keyword(k, 'euv DSL keyword', 'Used inside html! / match / for blocks.'),
    );

    const rustKeywords = RUST_KEYWORDS.map((k) => keyword(k, 'rust keyword'));

    const types = EUV_TYPES.map((t) => typeKw(t, 'euv core type'));

    const apiMethods = [
      method(
        'App::use_signal',
        'App::use_signal(|| ${1:initial_value})',
        'create a Signal<T> with an initial value',
        'Returns Signal<T>. Initializer is FnOnce() -> T. In euv-playground, run inside a component fn so App::current() is bound.',
      ),
      method(
        'App::mount',
        'App::mount("${1:#app}", ${2:app_fn})',
        'mount a component fn to a CSS selector',
        'Call from #[wasm_bindgen] pub fn main(). The app fn returns VirtualNode.',
      ),
      method(
        'App::mount_node',
        'App::mount_node(${1:"element_id"}, ${2:app_fn})',
        'mount directly to an element id',
      ),
      method(
        'App::mount_into',
        'App::mount_into("${1:#target}", ${2:child_fn})',
        'embed into an existing node (does not replace content)',
      ),
      method(
        'App::batch',
        'App::batch(|| {\n    ${0:// batched signal.set() calls}\n});',
        'batch signal updates — DOM updates only after the closure returns',
      ),
      method(
        'App::use_cleanup',
        'App::use_cleanup(move || { ${0:body} });',
        'register a cleanup callback for component unmount',
      ),
      method(
        'App::use_window_event',
        'App::use_window_event("${1:hashchange}", move || { ${0:body} });',
        'subscribe to a window-level event',
      ),
      method(
        'App::use_interval',
        'App::use_interval(${1:1000}, move || { ${0:body} });',
        'set up a recurring interval — returns IntervalHandle',
      ),
      method(
        'App::spawn',
        'App::spawn(async move { ${0:body} });',
        'spawn an async task from a sync component fn',
      ),
      method(
        'Signal::create',
        'Signal::create(${1:value})',
        'create a standalone Signal<T> (no HookContext required)',
      ),
      method(
        'Signal::default',
        'Signal::<${1:T}>::default()',
        'create a Signal<T> using T::default()',
      ),
      method(
        '.get()',
        '${1:signal}.get()',
        'read the inner value (tracks dependency inside reactive nodes)',
      ),
      method(
        '.set()',
        '${1:signal}.set(${2:new_value})',
        'update the inner value (no-op if value is unchanged)',
      ),
      method(
        '.auto_value()',
        '${1:signal}.auto_value()',
        'unwraps Signal to inner T (used in chained-call contexts)',
      ),
      method(
        '.subscribe()',
        '${1:signal}.subscribe(${2:|val: T| { /* listener */ }});',
        'append a listener that fires on every set',
      ),
      method(
        'NativeEventHandler::create',
        'NativeEventHandler::create("${1:click}", move |_event: Event| {\n    ${0:body}\n})',
        'build a reusable event handler closure',
      ),
    ];

    const eventAttrs = EUV_EVENT_ATTRS.map((a) =>
      attr(a + ':', 'euv event attribute'),
    );

    const cssPropAttrs = CSS_PROPERTIES.map((p) =>
      attr(p + ':', 'CSS property (in class! / style:)'),
    );

    const dslAttrs = ['class:', 'style:', 'key:'].map((a) =>
      attr(a, 'euv DSL attribute'),
    );

    const tagSnippets = EUV_HTML_TAGS.map((t) =>
      snippet(
        t,
        t === 'input'
          ? 'input { r#type: "${1:text}" ${0:placeholder: \\"\\"} }'
          : t === 'a'
            ? 'a { href: "${1:#}" ${0:\\"link text\\"} }'
            : t + ' { ${0:...} }',
        'html! tag',
        'euv html! tag — ' + t,
      ),
    );

    const classNameSnippets = EUV_CLASS_PREFIXES.map((p) =>
      snippet(
        p + '()',
        `${p}()`,
        'class! reference',
        `Call a class!() function. Add parameters if the class was declared with any (see its definition).`,
      ),
    );

    return {
      triggerCharacters: ['!', ':', '(', '.', '#', '"', '{', ' '],
      provideCompletionItems(model, position) {
        const word = model.getWordUntilPosition(position);
        const line = model.getLineContent(position.lineNumber);
        const before = line.substring(0, position.column - 1);
        // determine context: are we inside html! / class! / vars! body?
        let insideHtml = false;
        let insideClass = false;
        let insideVars = false;
        try {
          const text = model.getValue();
          const upto = text.substring(0, model.getOffsetAt(position));
          // walk top-level tokens to detect which macro body we're in
          let htmlDepth = 0;
          let classDepth = 0;
          let varsDepth = 0;
          let depth = 0;
          const macroRe = /\b(html|class|computed|watch|vars|var)!\s*[\(\{]/g;
          let m;
          while ((m = macroRe.exec(upto)) !== null) {
            const opener = m[0][m[0].length - 1];
            if (opener === '{') {
              if (m[1] === 'html') htmlDepth++;
              else if (m[1] === 'class') classDepth++;
              else if (m[1] === 'vars') varsDepth++;
              depth++;
            }
          }
          // count closing braces — only those that match our depths
          const braceCount = (upto.match(/\}/g) || []).length;
          depth = Math.max(0, depth - braceCount);
          insideHtml = htmlDepth > 0 && depth > 0;
          insideClass = classDepth > 0 && depth > 0;
          insideVars = varsDepth > 0 && depth > 0;
        } catch (e) {}

        // Build range from current word
        const range = {
          startLineNumber: position.lineNumber,
          endLineNumber: position.lineNumber,
          startColumn: word.startColumn,
          endColumn: word.endColumn,
        };

        const items = [];
        // context-specific extras first
        if (insideHtml) {
          items.push(
            ...tagSnippets,
            ...eventAttrs,
            ...cssPropAttrs,
            ...dslAttrs,
            ...dslKeywords,
          );
        }
        if (insideClass) {
          items.push(...classNameSnippets, ...cssPropAttrs);
          items.push(
            ...CSS_PSEUDOS.map((p) =>
              attr(p + ' { … }', 'pseudo-class / pseudo-element block'),
            ),
          );
          items.push(
            ...CSS_AT_RULES.map((a) =>
              attr(a + '(…) { … }', 'CSS at-rule block'),
            ),
          );
        }
        if (insideVars) {
          items.push(
            attr('bg-primary', 'CSS variable name (custom)'),
            attr('text-primary', 'CSS variable name (custom)'),
            attr('border-color', 'CSS variable name (custom)'),
            attr('glass-border', 'CSS variable name (custom)'),
            attr('shadow-card', 'CSS variable name (custom)'),
          );
        }
        // always-available
        items.push(
          ...macroSnippets,
          ...types,
          ...apiMethods,
          ...dslKeywords,
          ...rustKeywords,
        );
        return {
          suggestions: items.map((it) => Object.assign({}, it, { range })),
        };
      },
    };
  }

  // ---------- hover provider ----------
  function buildHoverProvider() {
    const docs = {
      'html!':
        '**html!** — declarative VirtualNode builder. Tags are bare identifiers or strings; attributes use `attr: value` syntax; children can be text, nested elements, or `{ Signal }` placeholders.',
      'class!':
        '**class!** — define CSS class definitions. Pseudo-class blocks (`hover`, `focus`, `nth_child(n)`) nest inside. Properties use snake_case → kebab-case.',
      'computed!':
        '**computed!** — derive a `Signal<T>` from one or more dependency signals. Re-runs when any dependency changes.',
      'watch!':
        '**watch!** — run a side-effect on every signal change. Fires once immediately.',
      'vars!': '**vars!** — define a bundle of CSS custom properties.',
      'var!':
        '**var!(name)** — reference a CSS variable defined by `vars!`. Emits `var(--name)`.',
      '#[component]':
        '**#[component]** — attribute macro that turns a fn into an euv component. Required for `html! { my_component { … } }` to resolve.',
      Signal:
        '**Signal<T>** — reactive value. Methods: `.get()`, `.set()`, `.auto_value()`, `.subscribe(cb)`. Inside `html! { … }` and reactive nodes, `.get()` is auto-called.',
      'App::use_signal':
        '**App::use_signal(|| initial)** — create a Signal<T>. Initializer is FnOnce() -> T. Must be called from within a component fn so App::current() is bound.',
      'App::mount':
        '**App::mount("#selector", app_fn)** — mount `app_fn` to a DOM element selected by the CSS selector.',
    };
    return {
      provideHover(model, position) {
        const word = model.getWordAtPosition(position);
        if (!word) return null;
        const w = word.word;
        // include trailing `!` for macros
        const line = model.getLineContent(position.lineNumber);
        const col = position.column - 1;
        const maybeMacro = w + (line[col + w.length] === '!' ? '!' : '');
        const doc = docs[maybeMacro] || docs[w];
        if (!doc) return null;
        return { contents: [{ value: doc }] };
      },
    };
  }

  // ---------- public entry point ----------
  // Call once monaco is loaded. Idempotent.
  window.loadEuvLanguage = function loadEuvLanguage(monaco) {
    if (!monaco || !monaco.languages) return false;
    if (window.__euvLanguageRegistered) return true;
    const tokenizer = buildTokenizer(monaco);
    // build the lookup tables from our keyword sets and feed them into
    // tokenizer state via cases
    const cases = tokenizer.tokenizer;
    // We populate the case tables indirectly: monaco's Monarch expects the
    // table at tokenizer.<state>.cases.<name> (which is a Set of keywords
    // keyed by token type). Easiest path: assign the sets on the tokenizer
    // object so the `@typeKeywords` / `@keywords` etc. cases can find them.
    tokenizer.typeKeywords = Array.from(new Set(EUV_TYPES));
    tokenizer.keywords = Array.from(
      new Set(RUST_KEYWORDS.concat(EUV_MACROS).concat(EUV_DSL_KEYWORDS)),
    );
    tokenizer.htmlTags = Array.from(new Set(EUV_HTML_TAGS));
    tokenizer.eventAttrs = Array.from(
      new Set(EUV_HTML_ATTRS.concat(EUV_EVENT_ATTRS)),
    );
    tokenizer.cssProperties = Array.from(new Set(CSS_PROPERTIES));
    tokenizer.pseudoNames = Array.from(new Set(CSS_PSEUDOS));
    tokenizer.atRules = Array.from(new Set(CSS_AT_RULES));
    tokenizer.dslKeywords = Array.from(new Set(EUV_DSL_KEYWORDS));
    // NOTE: We do NOT post-process `cases` to expand `@keywordsSet` /
    // `@typeKeywordsSet` / `@symbolsSet` / `@eventAttrsSet` / `@cssPropertiesSet`
    // / `@pseudoNamesSet` / `@atRulesSet` / `@dslKeywordsSet` / `@htmlTagsSet`
    // references into regexes. Monarch natively understands the
    // `@keywords` / `@typeKeywords` / `@symbols` / etc. keywords in
    // `cases` blocks when a matching array property exists on the tokenizer
    // root object (`tokenizer.keywords`, `tokenizer.typeKeywords`,
    // `tokenizer.operators`, ...). Mutating `cases` with regex substitutions
    // confuses Monarch's state-machine runner and makes every token collapse
    // to the rule's `defaultToken` (which euv tokens inherit from the
    // root, i.e. empty), producing a single-color black/white render.
    // See https://github.com/microsoft/monaco-editor/wiki/monarch — the
    // section "Reusing rules" describes this approach.

    try {
      monaco.languages.register({
        id: 'euv',
        extensions: ['.euv.rs'],
        aliases: ['Euv', 'euv'],
      });
    } catch (e) {}
    try {
      monaco.languages.setMonarchTokensProvider('euv', tokenizer);
    } catch (e) {}
    try {
      const comp = buildCompletionProvider(monaco);
      monaco.languages.registerCompletionItemProvider('euv', comp);
    } catch (e) {}
    try {
      const hov = buildHoverProvider();
      monaco.languages.registerHoverProvider('euv', hov);
    } catch (e) {}
    try {
      addEuvThemeRules(monaco);
    } catch (e) {}

    window.__euvLanguageRegistered = true;
    window.__euvTokenizer = tokenizer;
    // Expose theme-rule registration on window so the
    // <hyperlane-monaco-editor> custom element can call it during
    // _applyTheme (after the editor has been mounted and Monaco has
    // injected its hardcoded `.mtkN` rules).
    window.addEuvThemeRules = function (m) {
      return addEuvThemeRules(m);
    };
    return true;
  };
})();
