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
    DiffState {
        original,
        modified,
        view,
        rows,
        added,
        removed,
        unchanged,
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
