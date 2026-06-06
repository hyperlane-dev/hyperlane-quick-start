/// File path to the server environment configuration file in debug mode.
#[cfg(debug_assertions)]
pub const SERVER_ENV_FILE_PATH: &str = "./resources/env/dev/server.env";
/// File path to the server environment configuration file in release mode.
#[cfg(not(debug_assertions))]
pub const SERVER_ENV_FILE_PATH: &str = "./resources/env/release/server.env";
