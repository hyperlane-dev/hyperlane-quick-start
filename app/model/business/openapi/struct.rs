use super::*;

#[derive(utoipa::OpenApi)]
#[openapi(
    components(schemas(
        FileChunkData,
        UploadResponse,
        WebSocketReqData,
        WebSocketRespData,
        OnlineUser,
        UserListResponse
    )),
    info(
        title = "Hyperlane",
        version = "latest",
        description = "Hyperlane 是一个轻量级且高性能的 Rust HTTP 服务器库，旨在简化网络服务开发。它支持 HTTP 请求解析、响应构建和 TCP 通信，非常适合构建现代 Web 服务。此外，它还支持请求和响应中间件、WebSocket 和 Server-Sent Events (SSE)，从而实现灵活高效的实时通信。Hyperlane 使用纯 Rust 和标准库构建，提供跨 Windows、Linux 和 macOS 的真正跨平台兼容性，且所有平台上的 API 体验一致，依托 Tokio 的异步运行时实现无缝网络通信，无需特定于平台的依赖。"
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
        controller::ws::html,
        controller::ws::handle,
    )
)]
pub struct ApiDoc;
