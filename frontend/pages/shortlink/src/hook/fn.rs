use super::*;

/// Creates the reactive state the shortlink page renders from.
///
/// The state holds the URL the visitor typed, the short link the server
/// answered with and the message of the last failure.
///
/// # Returns
///
/// - `ShortlinkState` - The state the page renders from.
pub fn use_shortlink_state() -> ShortlinkState {
    let url: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let shortlink: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let copied: Signal<bool> = App::use_signal(|| false);
    let loading: Signal<bool> = App::use_signal(|| false);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    ShortlinkState {
        url,
        shortlink,
        copied,
        loading,
        error,
    }
}

/// Asks the server for a short link of the URL the visitor typed.
///
/// The URL has to start with `http://` or `https://`, otherwise the call is
/// refused before it reaches the server.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The short link has been written into the state.
pub async fn create_shortlink(state: ShortlinkState) {
    let url: String = state.get_url().get();
    let trimmed: String = url.trim().to_string();
    if trimmed.is_empty() {
        state
            .get_error()
            .set("Please enter a URL to shorten".to_string());
        return;
    }
    if !trimmed.starts_with(URL_PREFIX_HTTP) && !trimmed.starts_with(URL_PREFIX_HTTPS) {
        state
            .get_error()
            .set("The URL must start with http:// or https://".to_string());
        return;
    }
    state.get_loading().set(true);
    state.get_copied().set(false);
    let body: ShortlinkInsertRequest = ShortlinkInsertRequest { url: trimmed };
    match post_json::<String, ShortlinkInsertRequest>(PATH_SHORTLINK_INSERT, &body).await {
        Ok(id) => {
            state.get_shortlink().set(shortlink_url(&id));
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Copies the short link to the clipboard of the visitor.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The clipboard state has been written into the page state.
pub async fn copy_shortlink(state: ShortlinkState) {
    let shortlink: String = state.get_shortlink().get();
    if shortlink.is_empty() {
        state
            .get_error()
            .set("There is no short link to copy yet".to_string());
        return;
    }
    let Some(window): Option<Window> = window() else {
        state
            .get_error()
            .set("The browser window is not available".to_string());
        return;
    };
    let clipboard: Clipboard = window.navigator().clipboard();
    let promise: Promise = clipboard.write_text(&shortlink);
    match JsFuture::from(promise).await {
        Ok(_) => {
            state.get_copied().set(true);
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(_) => {
            state
                .get_error()
                .set("The short link could not be copied".to_string());
        }
    }
}

/// Clears the short link and the URL of the visitor.
///
/// # Arguments
///
/// - `ShortlinkState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The state has been cleared.
pub fn reset_shortlink(state: ShortlinkState) {
    state.get_url().set(TEXT_EMPTY.to_string());
    state.get_shortlink().set(TEXT_EMPTY.to_string());
    state.get_copied().set(false);
    state.get_error().set(TEXT_EMPTY.to_string());
}

/// Builds the absolute address of a short link.
///
/// The address points at the resolving endpoint of the server, which
/// redirects the visitor to the long URL the short link stands for.
///
/// # Arguments
///
/// - `&str` - The identifier the server answered with.
///
/// # Returns
///
/// - `String` - The absolute address of the short link.
fn shortlink_url(id: &str) -> String {
    let origin: String = window()
        .and_then(|window: Window| window.location().origin().ok())
        .unwrap_or_default();
    format!("{origin}{PATH_SHORTLINK_QUERY}/{id}")
}
