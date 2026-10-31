use super::*;

/// Creates the reactive state the diff page renders from.
///
/// The page calls no endpoint, so the state only holds the two texts and the
/// result of their comparison.
///
/// # Returns
///
/// - `DiffState` - The state the page renders from.
pub fn use_diff_state() -> DiffState {
    let original: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let modified: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let view: Signal<String> = App::use_signal(|| VIEW_SPLIT.to_string());
    let rows: Signal<Vec<DiffRow>> = App::use_signal(Vec::new);
    let added: Signal<usize> = App::use_signal(|| 0);
    let removed: Signal<usize> = App::use_signal(|| 0);
    let unchanged: Signal<usize> = App::use_signal(|| 0);
    let notice: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    DiffState {
        original,
        modified,
        view,
        rows,
        added,
        removed,
        unchanged,
        notice,
        error,
    }
}

/// Compares the two texts again and writes the result into the state.
///
/// The comparison runs in the browser, so the page needs no endpoint and the
/// result is ready as soon as the visitor stops typing.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The result has been written into the state.
pub fn recompute(state: DiffState) {
    let original: String = state.get_original().get();
    let modified: String = state.get_modified().get();
    let rows: Vec<DiffRow> = build_rows(&original, &modified);
    let (added, removed, unchanged) = count_rows(&rows);
    state.get_rows().set(rows);
    state.get_added().set(added);
    state.get_removed().set(removed);
    state.get_unchanged().set(unchanged);
}

/// Swaps the two texts and compares them again.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The texts have been swapped and the result has been written.
pub fn swap_texts(state: DiffState) {
    let original: String = state.get_original().get();
    let modified: String = state.get_modified().get();
    state.get_original().set(modified);
    state.get_modified().set(original);
    recompute(state);
}

/// Writes the example pair into the page and compares it again.
///
/// The example shows a text that differs in one word, so the visitor sees
/// what the page does without typing anything.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The example has been written and the result has been written.
pub fn load_example(state: DiffState) {
    state.get_original().set(EXAMPLE_ORIGINAL.to_string());
    state.get_modified().set(EXAMPLE_MODIFIED.to_string());
    state.get_notice().set(TEXT_EMPTY.to_string());
    state.get_error().set(TEXT_EMPTY.to_string());
    recompute(state);
}

/// Copies one of the two texts into the clipboard of the browser.
///
/// The clipboard is written by the browser itself, so the write is a promise
/// that can fail; the failure is reported instead of being swallowed.
///
/// # Arguments
///
/// - `DiffState` - The state the page renders from.
/// - `bool` - Whether the original text is copied instead of the comparison one.
///
/// # Returns
///
/// - `()` - The text has been copied, or the failure has been reported.
pub async fn copy_text(state: DiffState, original: bool) {
    let text: String = if original {
        state.get_original().get()
    } else {
        state.get_modified().get()
    };
    if text.is_empty() {
        state.get_error().set(ERROR_EMPTY_TEXT.to_string());
        state.get_notice().set(TEXT_EMPTY.to_string());
        return;
    }
    let Some(window): Option<Window> = window() else {
        state.get_error().set(ERROR_CLIPBOARD.to_string());
        state.get_notice().set(TEXT_EMPTY.to_string());
        return;
    };
    let promise: Promise = window.navigator().clipboard().write_text(&text);
    if JsFuture::from(promise).await.is_err() {
        state.get_error().set(ERROR_CLIPBOARD.to_string());
        state.get_notice().set(TEXT_EMPTY.to_string());
        return;
    }
    state.get_notice().set(NOTICE_COPIED.to_string());
    state.get_error().set(TEXT_EMPTY.to_string());
}
