use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    components(schemas(
        FileChunkData,
        UploadResponse,
        WebSocketReqData,
        WebSocketRespData,
        OnlineUser,
        UserListResponse,
        ServerStatus,
        SystemInfo,
        NetworkPacket,
        NetworkStats,
        ConnectionInfo,
        NetworkCaptureRequest,
        NetworkCaptureResponse
    )),
    info(
        title = "Hyperlane",
        version = "latest",
        description = "A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility."
    ),
    paths(
        controller::favicon_ico::handle,
        controller::hello::handle,
        controller::openapi::html,
        controller::openapi::json,
        controller::root::handle,
        controller::upload::html,
        controller::upload::static_file,
        controller::upload::merge,
        controller::upload::register,
        controller::upload::save,
        controller::users::online_users,
        controller::chat::html,
        controller::chat::handle,
        controller::server_status::status_sse,
        controller::server_status::system_info,
        controller::server_status::monitor_dashboard,
        controller::server_status::network_capture_data,
        controller::server_status::network_capture_stream,
    )
)]
pub struct ApiDoc;
