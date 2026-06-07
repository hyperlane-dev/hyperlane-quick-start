/// File path to the Docker Compose configuration file in debug mode.
#[cfg(debug_assertions)]
pub const SERVER_DOCKER_COMPOSE_FILE_PATH: &str =
    "./resources/docker/dev/server_docker_compose.yml";

/// File path to the Docker Compose configuration file in release mode.
#[cfg(not(debug_assertions))]
pub const SERVER_DOCKER_COMPOSE_FILE_PATH: &str =
    "./resources/docker/release/server_docker_compose.yml";

/// File path to the server Dockerfile in debug mode.
#[cfg(debug_assertions)]
pub const SERVER_DOCKERFILE_PATH: &str = "./resources/docker/dev/server.dockerfile";

/// File path to the server Dockerfile in release mode.
#[cfg(not(debug_assertions))]
pub const SERVER_DOCKERFILE_PATH: &str = "./resources/docker/release/server.dockerfile";
