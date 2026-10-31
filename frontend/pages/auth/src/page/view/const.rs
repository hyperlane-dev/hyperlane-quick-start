use super::*;

/// The tabs of the authentication page.
pub(crate) const TAB_ITEMS: &[EuvTabItem] = &[
    EuvTabItem {
        key: MODE_LOGIN,
        label: "Login",
    },
    EuvTabItem {
        key: MODE_REGISTER,
        label: "Register",
    },
];

/// The glyph of the page header.
pub(crate) const ICON_AUTH: &str = "🔑";

/// The title of the page header.
pub(crate) const TITLE_HEADER: &str = "Hyperlane Auth";

/// The subtitle of the page header.
pub(crate) const TEXT_HEADER: &str = "Secure authentication for all services";

/// The title of the login form.
pub(crate) const TITLE_LOGIN: &str = "Login";

/// The title of the register form.
pub(crate) const TITLE_REGISTER: &str = "Create Account";

/// The label of the name field.
pub(crate) const LABEL_USERNAME: &str = "Username";

/// The label of the password field.
pub(crate) const LABEL_PASSWORD: &str = "Password";

/// The label of the address field.
pub(crate) const LABEL_EMAIL: &str = "Email";

/// The label of the phone field.
pub(crate) const LABEL_PHONE: &str = "Phone";

/// The placeholder of the name field.
pub(crate) const PLACEHOLDER_USERNAME: &str = "Enter username";

/// The placeholder of the password field.
pub(crate) const PLACEHOLDER_PASSWORD: &str = "Enter password";

/// The placeholder of the address field.
pub(crate) const PLACEHOLDER_EMAIL: &str = "Your email";

/// The placeholder of the phone field.
pub(crate) const PLACEHOLDER_PHONE: &str = "Your phone number";

/// The identifier of the name field of the login form.
pub(crate) const ID_LOGIN_USERNAME: &str = "auth-login-username";

/// The identifier of the password field of the login form.
pub(crate) const ID_LOGIN_PASSWORD: &str = "auth-login-password";

/// The identifier of the name field of the register form.
pub(crate) const ID_REGISTER_USERNAME: &str = "auth-register-username";

/// The identifier of the password field of the register form.
pub(crate) const ID_REGISTER_PASSWORD: &str = "auth-register-password";

/// The identifier of the address field of the register form.
pub(crate) const ID_REGISTER_EMAIL: &str = "auth-register-email";

/// The identifier of the phone field of the register form.
pub(crate) const ID_REGISTER_PHONE: &str = "auth-register-phone";

/// The type of a text field.
pub(crate) const INPUT_TEXT: &str = "text";

/// The type of a password field.
pub(crate) const INPUT_PASSWORD: &str = "password";

/// The type of an address field.
pub(crate) const INPUT_EMAIL: &str = "email";

/// The type of a phone field.
pub(crate) const INPUT_PHONE: &str = "tel";

/// The label of the login button.
pub(crate) const LABEL_LOGIN: &str = "Login";

/// The label of the register button.
pub(crate) const LABEL_REGISTER: &str = "Register";
