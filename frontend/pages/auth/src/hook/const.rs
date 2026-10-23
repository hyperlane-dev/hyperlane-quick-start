/// The form that holds the login fields.
pub(crate) const MODE_LOGIN: &str = "login";

/// The form that holds the register fields.
pub(crate) const MODE_REGISTER: &str = "register";

/// The endpoint of the public key the credentials are encrypted with.
pub(crate) const PATH_PUBLIC_KEY: &str = "/api/auth/rsa/public-key";

/// The endpoint that logs a user in.
pub(crate) const PATH_LOGIN: &str = "/api/auth/login";

/// The endpoint that registers a user.
pub(crate) const PATH_REGISTER: &str = "/api/auth/register";

/// The empty value of a field.
pub(crate) const EMPTY: &str = "";

/// The shortest password a registration accepts.
pub(crate) const MIN_PASSWORD_LENGTH: usize = 6;

/// The name of the query parameter that holds the page to return to.
pub(crate) const QUERY_LOCATION: &str = "location";

/// The page a visitor lands on when no return page is requested.
pub(crate) const DEFAULT_LOCATION: &str = "/";

/// The failure reported when a required field is empty.
pub(crate) const ERROR_EMPTY_FIELDS: &str = "Please fill in all fields";

/// The failure reported when the password is too short.
pub(crate) const ERROR_PASSWORD_TOO_SHORT: &str = "Password must be at least 6 characters";

/// The failure reported when the public key cannot be fetched.
pub(crate) const ERROR_KEY_FAILED: &str = "Encryption failed, please refresh and try again";

/// The notice reported after a successful registration.
pub(crate) const NOTICE_REGISTERED: &str = "Registration successful! Please login.";
