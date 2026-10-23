use super::*;

/// The reactive state the authentication page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct AuthState {
    /// The form being displayed.
    #[get(type(copy))]
    pub mode: Signal<String>,
    /// The name of the account that logs in.
    #[get(type(copy))]
    pub login_username: Signal<String>,
    /// The password of the account that logs in.
    #[get(type(copy))]
    pub login_password: Signal<String>,
    /// The name of the account that is registered.
    #[get(type(copy))]
    pub register_username: Signal<String>,
    /// The password of the account that is registered.
    #[get(type(copy))]
    pub register_password: Signal<String>,
    /// The address of the account that is registered.
    #[get(type(copy))]
    pub register_email: Signal<String>,
    /// The phone number of the account that is registered.
    #[get(type(copy))]
    pub register_phone: Signal<String>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
}
