use super::*;

#[cfg(debug_assertions)]
/// Returns the `SERVER_DOCKER_COMPOSE_FILE_PATH` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn server_docker_compose_file_path() -> &'static str {
    SERVER_DOCKER_COMPOSE_FILE_PATH
}

#[cfg(not(debug_assertions))]
/// Returns the `SERVER_DOCKER_COMPOSE_FILE_PATH` configuration value.
///
/// # Returns
///
/// - `&'static str` - The configured value.
pub fn server_docker_compose_file_path() -> &'static str {
    SERVER_DOCKER_COMPOSE_FILE_PATH
}
