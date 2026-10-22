use super::*;

thread_local! {
    /// Whether the stylesheet of the shared crate is already in the document.
    static CSS_INJECTED: Cell<bool> = const { Cell::new(false) };
}

/// The stylesheet the shared components need on top of the library.
///
/// The buttons of the library carry `flex: 1 1 120px`, so a button grows into
/// the whole row it sits in. The rule keeps the buttons of a shared button row
/// at the size of their label. The `html body` prefix raises the specificity
/// and the `!important` flag makes the override win over the rule of the
/// library, whatever order the two rules end up in.
///
/// The editor of the library paints the brackets of the code in colour. The
/// colour comes from a rule the editor writes into the document, and neither
/// the theme of the editor nor the option that turns the colouring off
/// changes it, so the rule of the shared sheet is the only place where the
/// colour can be taken away. `currentColor` makes the brackets take the ink
/// of the theme, so they follow the page into the dark theme as well.
const SHARED_CSS: &str = concat!(
    "html body .c_hls_button_row button { flex: 0 0 auto !important; }",
    "html body .monaco-editor [class*=\"bracket-highlighting-\"] { color: currentColor !important; }",
);

/// Injects the stylesheet of the shared crate into the document.
///
/// The document keeps the sheet, so the function does the work once.
///
/// # Returns
///
/// - `()` - The stylesheet is in the document.
pub fn inject_shared_css() {
    let injected: bool = CSS_INJECTED.with(|flag: &Cell<bool>| flag.replace(true));
    if injected {
        return;
    }
    Css::inject_css(SHARED_CSS);
}

/// Defines the two monochrome themes of the editor.
///
/// The themes are built from the base themes of Monaco, so the layout of the
/// editor stays the one of the library and only the ink changes. The script
/// defines both pairs, because the editor follows the theme of the page and
/// picks the pair that matches.
///
/// The script also records the helper that reads the theme of the page and
/// the helper that watches it, so the editor follows the page into the dark
/// theme without the page having to hand the theme over.
///
/// # Returns
///
/// - `()` - The themes are defined in the editor.
pub fn define_monaco_themes() {
    let script: String = format!(
        "(function () {{ if (!window.monaco || !window.monaco.editor) {{ return; }}; window.monaco.editor.defineTheme('{LIGHT_NAME}', {{ base: 'vs', inherit: true, rules: {LIGHT_RULES}, colors: {LIGHT_COLORS} }}); window.monaco.editor.defineTheme('{DARK_NAME}', {{ base: 'vs-dark', inherit: true, rules: {DARK_RULES}, colors: {DARK_COLORS} }}); window.__euv_monaco_theme = function () {{ var dark = document.querySelector('.c_theme_dark') !== null; return dark ? '{DARK_NAME}' : '{LIGHT_NAME}'; }}; window.__euv_monaco_follow = function () {{ if (window.__euv_monaco_observer) {{ return; }}; window.__euv_monaco_observer = new MutationObserver(function () {{ var theme = window.__euv_monaco_theme(); if (window.monaco.editor.getTheme && window.monaco.editor.getTheme() === theme) {{ return; }}; window.monaco.editor.setTheme(theme); }}); window.__euv_monaco_observer.observe(document.body, {{ attributes: true, subtree: true, attributeFilter: ['class'] }}); }}; }})();",
        LIGHT_NAME = MONACO_THEME_LIGHT,
        DARK_NAME = MONACO_THEME_DARK,
        LIGHT_RULES = MONACO_LIGHT_RULES,
        LIGHT_COLORS = MONACO_LIGHT_COLORS,
        DARK_RULES = MONACO_DARK_RULES,
        DARK_COLORS = MONACO_DARK_COLORS,
    );
    let _: Result<JsValue, JsValue> = js_sys::eval(&script);
}

/// The script that picks the theme of the editor and follows the theme of the
/// page from then on.
///
/// The theme of the page is a class on the root of the application, so the
/// editor reads that class and watches it. The editor of the page follows the
/// theme without the page having to hand the theme over.
///
/// # Returns
///
/// - `String` - The script that applies the theme.
pub fn monaco_theme_script() -> String {
    String::from(
        "(function () { if (!window.monaco || !window.monaco.editor) { return; } if (!window.__euv_monaco_theme) { return; } window.monaco.editor.setTheme(window.__euv_monaco_theme()); if (window.__euv_monaco_follow) { window.__euv_monaco_follow(); } })();",
    )
}
