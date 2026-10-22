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
const SHARED_CSS: &str = "html body .c_hls_button_row button { flex: 0 0 auto !important; }";

/// Injects the stylesheet of the shared crate into the document.
///
/// The document keeps the sheet, so the function does the work once.
pub fn inject_shared_css() {
    let injected: bool = CSS_INJECTED.with(|flag: &Cell<bool>| flag.replace(true));
    if injected {
        return;
    }
    Css::inject_css(SHARED_CSS);
}
