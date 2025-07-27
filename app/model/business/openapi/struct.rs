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
        description = "Hyperlane is a lightweight and high-performance Rust HTTP server library designed to simplify network service development. It supports HTTP request parsing, response building, and TCP communication, making it ideal for building modern web services. Additionally, it provides support for request and response middleware, WebSocket, and Server-Sent Events (SSE), enabling flexible and efficient real-time communication. Built with pure Rust and standard library, Hyperlane offers true cross-platform compatibility across Windows, Linux and macOS, with the same API experience on all platforms, powered by Tokio's async runtime for seamless networking without platform-specific dependencies."
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
