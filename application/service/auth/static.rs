use super::*;

pub(super) static AUTH_SERVICE: OnceLock<AuthService> = OnceLock::new();
pub static EMAIL_REGEX: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").ok());
pub static PHONE_REGEX_OPT: Lazy<Option<Regex>> =
    Lazy::new(|| Regex::new(r"^\+?[1-9]\d{1,14}$").ok());
