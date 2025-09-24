use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    components(schemas(
        FileChunkData,
        UploadResponse,
        StaticFileResponse,
        StaticFileError,
        SecurityError,
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
        // controller::openapi::html, // 已移至统一路由
        controller::openapi::json,
        controller::static_files::serve_static_resource,
        controller::static_files::serve_index_page,
        controller::static_files::serve_upload_page,
        controller::static_files::serve_monitor_page,
        controller::static_files::serve_openapi_page,
        controller::static_files::serve_chat_page,
        controller::static_files::serve_fallback_page,
        // controller::upload::html, // 已移至统一路由
        controller::upload::upload_file,
        controller::upload::merge,
        controller::upload::register,
        controller::upload::save,
        controller::users::online_users,
        // controller::chat::html, // 已移至统一路由
        controller::chat::handle,
        controller::server_status::status_sse,
        controller::server_status::system_info,
        // controller::server_status::monitor_dashboard, // 已移至统一路由
        controller::server_status::network_capture_data,
        controller::server_status::network_capture_stream,
    )
)]
pub struct ApiDoc;
