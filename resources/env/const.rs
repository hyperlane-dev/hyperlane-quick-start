#[cfg(debug_assertions)]
pub const SERVER_ENV_FILE_PATH: &str = "./resources/env/dev/server.env";
#[cfg(not(debug_assertions))]
pub const SERVER_ENV_FILE_PATH: &str = "./resources/env/release/server.env";
