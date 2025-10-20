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
        NetworkCaptureResponse,
    )),
    info(
        title = "Hyperlane",
        version = "latest",
        description = "A lightweight, high-performance, and cross-platform Rust HTTP server library built on Tokio. It simplifies modern web service development by providing built-in support for middleware, WebSocket, Server-Sent Events (SSE), and raw TCP communication. With a unified and ergonomic API across Windows, Linux, and MacOS, it enables developers to build robust, scalable, and event-driven network applications with minimal overhead and maximum flexibility."
    ),
    modifiers(&PathsModifier)
)]
pub struct ApiDoc;

struct PathsModifier;

impl utoipa::Modify for PathsModifier {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use std::collections::BTreeMap;
        use utoipa::openapi::*;

        let path_item = |method: &str, summary: &str| -> PathItem {
            let mut responses = ResponsesBuilder::new();
            responses = responses.response("200", ResponseBuilder::new().description("Success"));

            let mut op = path::OperationBuilder::new();
            op = op.summary(Some(summary));
            op = op.responses(responses);

            let operation = op.build();

            match method {
                "get" => PathItem::new(HttpMethod::Get, operation),
                "post" => PathItem::new(HttpMethod::Post, operation),
                _ => PathItem::new(HttpMethod::Get, operation),
            }
        };

        let mut paths_map: BTreeMap<String, PathItem> = BTreeMap::new();

        paths_map.insert(
            "/hello/{name}".to_string(),
            path_item("get", "Hello greeting"),
        );
        paths_map.insert(
            "/api/chat/online-users".to_string(),
            path_item("get", "Get online users list"),
        );
        paths_map.insert(
            "/api/upload/register".to_string(),
            path_item("post", "Register file upload"),
        );
        paths_map.insert(
            "/api/upload/save".to_string(),
            path_item("post", "Save file chunk"),
        );
        paths_map.insert(
            "/api/upload/merge".to_string(),
            path_item("post", "Merge file chunks"),
        );
        paths_map.insert(
            "/api/log/info".to_string(),
            path_item("get", "Get info logs"),
        );
        paths_map.insert(
            "/api/log/warn".to_string(),
            path_item("get", "Get warn logs"),
        );
        paths_map.insert(
            "/api/log/error".to_string(),
            path_item("get", "Get error logs"),
        );
        paths_map.insert(
            "/api/monitor/status-sse".to_string(),
            path_item("get", "Server status SSE stream"),
        );
        paths_map.insert(
            "/api/monitor/system-info".to_string(),
            path_item("get", "Get system information"),
        );
        paths_map.insert(
            "/api/monitor/network-capture-data".to_string(),
            path_item("get", "Get network capture data"),
        );
        paths_map.insert(
            "/api/monitor/network-capture-stream".to_string(),
            path_item("get", "Network capture SSE stream"),
        );
        paths_map.insert(
            "/api/mysql/records".to_string(),
            path_item("get", "List all mysql records"),
        );
        paths_map.insert(
            "/api/mysql/record".to_string(),
            path_item("post", "Create mysql record"),
        );
        paths_map.insert(
            "/api/mysql/record/update".to_string(),
            path_item("post", "Update mysql record"),
        );
        paths_map.insert(
            "/api/mysql/record/delete".to_string(),
            path_item("post", "Delete mysql record"),
        );
        paths_map.insert(
            "/api/postgresql/records".to_string(),
            path_item("get", "List all postgresql records"),
        );
        paths_map.insert(
            "/api/postgresql/record".to_string(),
            path_item("post", "Create postgresql record"),
        );
        paths_map.insert(
            "/api/postgresql/record/update".to_string(),
            path_item("post", "Update postgresql record"),
        );
        paths_map.insert(
            "/api/postgresql/record/delete".to_string(),
            path_item("post", "Delete postgresql record"),
        );
        paths_map.insert(
            "/api/redis/records".to_string(),
            path_item("get", "List all redis records"),
        );
        paths_map.insert(
            "/api/redis/record".to_string(),
            path_item("post", "Create redis record"),
        );
        paths_map.insert(
            "/api/redis/record/update".to_string(),
            path_item("post", "Update redis record"),
        );
        paths_map.insert(
            "/api/redis/record/delete".to_string(),
            path_item("post", "Delete redis record"),
        );
        paths_map.insert(
            "/api/sse".to_string(),
            path_item("get", "Server-Sent Events stream"),
        );
        paths_map.insert(
            "/api/trace/{trace}".to_string(),
            path_item("get", "Search trace logs"),
        );
        paths_map.insert(
            "/api/websocket".to_string(),
            path_item("get", "WebSocket connection"),
        );
        paths_map.insert(
            "/openapi/openapi.json".to_string(),
            path_item("get", "OpenAPI JSON specification"),
        );

        openapi.paths.paths = paths_map;
    }
}
