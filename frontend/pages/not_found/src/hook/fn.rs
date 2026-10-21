use super::*;

/// Creates the reactive state the not found page renders from.
///
/// The page calls no endpoint, so the state only holds the status code and
/// the links of the card.
///
/// # Returns
///
/// - `NotFoundState` - The state the page renders from.
pub fn use_not_found_state() -> NotFoundState {
    let code: Signal<String> = App::use_signal(|| CODE_NOT_FOUND.to_string());
    let links: Signal<Vec<NotFoundLink>> = App::use_signal(|| {
        vec![
            NotFoundLink {
                label: LABEL_REPOSITORY.to_string(),
                url: URL_REPOSITORY.to_string(),
                target: TARGET_BLANK.to_string(),
            },
            NotFoundLink {
                label: LABEL_HOME.to_string(),
                url: URL_HOME.to_string(),
                target: String::new(),
            },
        ]
    });
    NotFoundState { code, links }
}
