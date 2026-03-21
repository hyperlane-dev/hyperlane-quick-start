#[cfg(debug_assertions)]
pub const SERVER_DOCKER_COMPOSE_FILE_PATH: &str =
    "./resources/docker/dev/server_docker_compose.yml";
#[cfg(not(debug_assertions))]
pub const SERVER_DOCKER_COMPOSE_FILE_PATH: &str =
    "./resources/docker/release/server_docker_compose.yml";
#[cfg(debug_assertions)]
pub const SERVER_DOCKERFILE_PATH: &str = "./resources/docker/dev/server.dockerfile";
#[cfg(not(debug_assertions))]
pub const SERVER_DOCKERFILE_PATH: &str = "./resources/docker/release/server.dockerfile";
