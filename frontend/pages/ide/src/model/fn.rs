use super::*;

/// Resolves the language the query parameter of the page names.
///
/// An alias the table does not hold names the default language of the
/// scratchpad.
///
/// # Arguments
///
/// - `&str` - The value of the query parameter of the page.
///
/// # Returns
///
/// - `IdeLanguage` - The language the scratchpad opens with.
pub fn resolve_language(parameter: &str) -> IdeLanguage {
    let alias: String = parameter.trim().to_lowercase();
    let mut id: &str = DEFAULT_LANGUAGE;
    for (name, language) in ALIASES {
        if *name == alias.as_str() {
            id = *language;
            break;
        }
    }
    IdeLanguage {
        id: id.to_string(),
        name: language_name(id),
    }
}

/// Returns the name a language is shown under.
///
/// # Arguments
///
/// - `&str` - The identifier of the language.
///
/// # Returns
///
/// - `String` - The name of the language.
pub fn language_name(id: &str) -> String {
    for (language, name) in LANGUAGES {
        if *language == id {
            return (*name).to_string();
        }
    }
    id.to_string()
}

/// Returns the code the scratchpad kept for a language.
///
/// # Arguments
///
/// - `&str` - The identifier of the language.
///
/// # Returns
///
/// - `String` - The code of the language, empty when it was never written.
pub fn stored_code(id: &str) -> String {
    let Some(window): Option<Window> = window() else {
        return String::new();
    };
    let Ok(Some(storage)): Result<Option<Storage>, JsValue> = window.local_storage() else {
        return String::new();
    };
    let key: String = format!("{STORAGE_PREFIX}{id}");
    storage.get_item(&key).unwrap_or_default().unwrap_or_default()
}

/// Keeps the code of a language in the storage of the browser.
///
/// # Arguments
///
/// - `&str` - The identifier of the language.
/// - `&str` - The code of the language.
///
/// # Returns
///
/// - `()` - The code is in the storage, or the browser refused it.
pub fn store_code(id: &str, code: &str) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let Ok(Some(storage)): Result<Option<Storage>, JsValue> = window.local_storage() else {
        return;
    };
    let key: String = format!("{STORAGE_PREFIX}{id}");
    let _: Result<(), JsValue> = storage.set_item(&key, code);
}

/// Returns the language the query parameter of the page names.
///
/// # Returns
///
/// - `IdeLanguage` - The language the scratchpad opens with.
pub fn page_language() -> IdeLanguage {
    let Some(window): Option<Window> = window() else {
        return resolve_language(DEFAULT_LANGUAGE);
    };
    let location: Location = window.location();
    let search: String = location.search().unwrap_or_default();
    resolve_language(&parameter_of(&search))
}

/// Returns the value of the language parameter of a query string.
///
/// # Arguments
///
/// - `&str` - The query string of the page.
///
/// # Returns
///
/// - `String` - The value of the parameter, empty when it is absent.
fn parameter_of(search: &str) -> String {
    for pair in search.trim_start_matches('?').split('&') {
        let Some((key, value)) = pair.split_once('=') else {
            continue;
        };
        if key != QUERY_LANGUAGE {
            continue;
        }
        return decode(value);
    }
    String::new()
}

/// Decodes a value of a query string.
///
/// # Arguments
///
/// - `&str` - The value to decode.
///
/// # Returns
///
/// - `String` - The decoded value.
fn decode(value: &str) -> String {
    let replaced: String = value.replace('+', " ");
    js_sys::decode_uri(&replaced).unwrap_or_default().into()
}
