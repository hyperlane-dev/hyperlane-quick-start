use super::*;

/// Creates the reactive state the authentication page renders from.
///
/// The state holds the fields of the two forms, the message of the last
/// failure and the notice of the last success.
///
/// # Returns
///
/// - `AuthState` - The state the page renders from.
pub fn use_auth_state() -> AuthState {
    let mode: Signal<String> = App::use_signal(|| MODE_LOGIN.to_string());
    let login_username: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let login_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let register_username: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let register_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let register_email: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let register_phone: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let loading: Signal<bool> = App::use_signal(|| false);
    let error: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| EMPTY.to_string());
    AuthState {
        mode,
        login_username,
        login_password,
        register_username,
        register_password,
        register_email,
        register_phone,
        loading,
        error,
        notice,
    }
}

/// Logs the visitor in with the credentials of the login form.
///
/// The credentials are encrypted with the public key of the server, the
/// server sets the session cookie and the visitor is sent to the page the
/// query of the address holds.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The visitor has been logged in, or the failure has been reported.
pub async fn submit_login(state: AuthState) {
    let username: String = state.get_login_username().get().trim().to_string();
    let password: String = state.get_login_password().get();
    if username.is_empty() || password.is_empty() {
        state.get_error().set(ERROR_EMPTY_FIELDS.to_string());
        return;
    }
    state.get_loading().set(true);
    state.get_error().set(EMPTY.to_string());
    let key: PublicKey = match public_key().await {
        Ok(key) => key,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    };
    let encrypted_username: String = match encrypt_field(&username, &key) {
        Ok(value) => value,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    };
    let encrypted_password: String = match encrypt_field(&password, &key) {
        Ok(value) => value,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    };
    let body: LoginRequest = LoginRequest {
        username: encrypted_username,
        password: encrypted_password,
    };
    match post_json::<LoginResponse, LoginRequest>(PATH_LOGIN, &body).await {
        Ok(_) => {
            state.get_loading().set(false);
            navigate(&return_location());
        }
        Err(error) => {
            state.get_loading().set(false);
            state.get_error().set(error.message);
        }
    }
}

/// Registers the visitor with the fields of the register form.
///
/// The credentials are encrypted with the public key of the server, the
/// form is cleared afterwards and the login form is shown again.
///
/// # Arguments
///
/// - `AuthState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The account has been registered, or the failure has been reported.
pub async fn submit_register(state: AuthState) {
    let username: String = state.get_register_username().get().trim().to_string();
    let password: String = state.get_register_password().get();
    let email: String = state.get_register_email().get().trim().to_string();
    let phone: String = state.get_register_phone().get().trim().to_string();
    if username.is_empty() || password.is_empty() {
        state.get_error().set(ERROR_EMPTY_FIELDS.to_string());
        return;
    }
    if password.chars().count() < MIN_PASSWORD_LENGTH {
        state.get_error().set(ERROR_PASSWORD_TOO_SHORT.to_string());
        return;
    }
    state.get_loading().set(true);
    state.get_error().set(EMPTY.to_string());
    let key: PublicKey = match public_key().await {
        Ok(key) => key,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    };
    let mut body: RegisterRequest = RegisterRequest {
        username: EMPTY.to_string(),
        password: EMPTY.to_string(),
        email: None,
        phone: None,
    };
    match encrypt_field(&username, &key) {
        Ok(value) => body.username = value,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    }
    match encrypt_field(&password, &key) {
        Ok(value) => body.password = value,
        Err(message) => {
            state.get_loading().set(false);
            state.get_error().set(message);
            return;
        }
    }
    if !email.is_empty() {
        match encrypt_field(&email, &key) {
            Ok(value) => body.email = Some(value),
            Err(message) => {
                state.get_loading().set(false);
                state.get_error().set(message);
                return;
            }
        }
    }
    if !phone.is_empty() {
        match encrypt_field(&phone, &key) {
            Ok(value) => body.phone = Some(value),
            Err(message) => {
                state.get_loading().set(false);
                state.get_error().set(message);
                return;
            }
        }
    }
    match post_json::<UserProfile, RegisterRequest>(PATH_REGISTER, &body).await {
        Ok(_) => {
            state.get_loading().set(false);
            state.get_register_username().set(EMPTY.to_string());
            state.get_register_password().set(EMPTY.to_string());
            state.get_register_email().set(EMPTY.to_string());
            state.get_register_phone().set(EMPTY.to_string());
            state.get_notice().set(NOTICE_REGISTERED.to_string());
            state.get_mode().set(MODE_LOGIN.to_string());
        }
        Err(error) => {
            state.get_loading().set(false);
            state.get_error().set(error.message);
        }
    }
}

/// Fetches the public key the credentials are encrypted with.
///
/// # Returns
///
/// - `Result<PublicKey, String>` - The key of the server, or the failure that stopped it.
async fn public_key() -> Result<PublicKey, String> {
    let key: RsaPublicKey = match get_json::<RsaPublicKey>(PATH_PUBLIC_KEY).await {
        Ok(key) => key,
        Err(_) => return Err(ERROR_KEY_FAILED.to_string()),
    };
    PublicKey::from_base64(&key.modulus, &key.exponent)
}

/// Reads the page the visitor returns to after the login.
///
/// # Returns
///
/// - `String` - The page of the query of the address, or the home page.
fn return_location() -> String {
    match query_param(QUERY_LOCATION) {
        Some(location) if !location.is_empty() => location,
        _ => DEFAULT_LOCATION.to_string(),
    }
}

/// Reads the value of one query parameter of the address.
///
/// # Arguments
///
/// - `&str` - The name of the parameter.
///
/// # Returns
///
/// - `Option<String>` - The value of the parameter, absent when the address holds none.
fn query_param(name: &str) -> Option<String> {
    let Some(window): Option<Window> = window() else {
        return None;
    };
    let location: Location = window.location();
    let Ok(search): Result<String, JsValue> = location.search() else {
        return None;
    };
    for pair in search.trim_start_matches('?').split('&') {
        let Some((key, value)): Option<(&str, &str)> = pair.split_once('=') else {
            continue;
        };
        if key != name {
            continue;
        }
        return Some(decode_component(value));
    }
    None
}

/// Decodes one percent encoded component of a query.
///
/// # Arguments
///
/// - `&str` - The component to decode.
///
/// # Returns
///
/// - `String` - The decoded component, or the component itself.
fn decode_component(value: &str) -> String {
    js_sys::decode_uri_component(value)
        .map(|decoded: JsString| decoded.as_string().unwrap_or_default())
        .unwrap_or_else(|_| value.to_string())
}

/// Sends the browser to another page.
///
/// # Arguments
///
/// - `&str` - The page to send the browser to.
///
/// # Returns
///
/// - `()` - The browser has been sent to the page.
fn navigate(location: &str) {
    let Some(window): Option<Window> = window() else {
        return;
    };
    let _: Result<(), JsValue> = window.location().set_href(location);
}
