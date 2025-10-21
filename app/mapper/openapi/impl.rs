use super::*;

impl Modify for PathsModifier {
    fn modify(&self, openapi: &mut OpenApi) {
        let success_response = || ResponseBuilder::new().description("Success");
        let bad_request_response = || ResponseBuilder::new().description("Bad Request");
        let not_found_response = || ResponseBuilder::new().description("Not Found");
        let internal_error_response =
            || ResponseBuilder::new().description("Internal Server Error");

        let path_item = |method: &str, summary: &str, description: Option<&str>| -> PathItem {
            let mut responses = ResponsesBuilder::new();
            responses = responses
                .response("200", success_response())
                .response("400", bad_request_response())
                .response("404", not_found_response())
                .response("500", internal_error_response());

            let mut op = OperationBuilder::new();
            op = op.summary(Some(summary));
            if let Some(desc) = description {
                op = op.description(Some(desc));
            }
            op = op.responses(responses);

            let operation = op.build();

            match method {
                "get" => PathItem::new(HttpMethod::Get, operation),
                "post" => PathItem::new(HttpMethod::Post, operation),
                _ => PathItem::new(HttpMethod::Get, operation),
            }
        };

        let path_item_with_body = |method: &str,
                                   summary: &str,
                                   description: Option<&str>,
                                   schema_ref: &str|
         -> PathItem {
            let mut responses = ResponsesBuilder::new();
            responses = responses
                .response("200", success_response())
                .response("400", bad_request_response())
                .response("404", not_found_response())
                .response("500", internal_error_response());

            let request_body = RequestBodyBuilder::new()
                .description(Some("Request body"))
                .content(
                    "application/json",
                    ContentBuilder::new()
                        .schema(Some(Ref::from_schema_name(schema_ref)))
                        .build(),
                )
                .required(Some(Required::True))
                .build();

            let mut op = OperationBuilder::new();
            op = op.summary(Some(summary));
            if let Some(desc) = description {
                op = op.description(Some(desc));
            }
            op = op.request_body(Some(request_body));
            op = op.responses(responses);

            let operation = op.build();

            match method {
                "get" => PathItem::new(HttpMethod::Get, operation),
                "post" => PathItem::new(HttpMethod::Post, operation),
                _ => PathItem::new(HttpMethod::Get, operation),
            }
        };

        let path_item_with_response = |method: &str,
                                       summary: &str,
                                       description: Option<&str>,
                                       response_schema: &str|
         -> PathItem {
            let success_resp = ResponseBuilder::new().description("Success").content(
                "application/json",
                ContentBuilder::new()
                    .schema(Some(Ref::from_schema_name(response_schema)))
                    .build(),
            );

            let mut responses = ResponsesBuilder::new();
            responses = responses
                .response("200", success_resp)
                .response("400", bad_request_response())
                .response("404", not_found_response())
                .response("500", internal_error_response());

            let mut op = OperationBuilder::new();
            op = op.summary(Some(summary));
            if let Some(desc) = description {
                op = op.description(Some(desc));
            }
            op = op.responses(responses);

            let operation = op.build();

            match method {
                "get" => PathItem::new(HttpMethod::Get, operation),
                "post" => PathItem::new(HttpMethod::Post, operation),
                _ => PathItem::new(HttpMethod::Get, operation),
            }
        };

        let path_item_with_param = |method: &str,
                                    summary: &str,
                                    description: Option<&str>,
                                    param_name: &str,
                                    param_desc: &str|
         -> PathItem {
            let mut responses: ResponsesBuilder = ResponsesBuilder::new();
            responses = responses
                .response("200", success_response())
                .response("400", bad_request_response())
                .response("404", not_found_response())
                .response("500", internal_error_response());

            let parameter = ParameterBuilder::new()
                .name(param_name)
                .parameter_in(ParameterIn::Path)
                .description(Some(param_desc))
                .required(Required::True)
                .schema(Some(
                    ObjectBuilder::new()
                        .schema_type(SchemaType::Type(Type::String))
                        .build(),
                ))
                .build();

            let mut op: OperationBuilder = OperationBuilder::new();
            op = op.summary(Some(summary));
            if let Some(desc) = description {
                op = op.description(Some(desc));
            }
            op = op.parameter(parameter);
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
            path_item_with_param(
                "get",
                "Hello greeting",
                Some("Returns a greeting message based on the provided name"),
                "name",
                "User name",
            ),
        );

        paths_map.insert(
            "/api/chat/online-users".to_string(),
            path_item_with_response(
                "get",
                "Get online users list",
                Some("Returns information about all currently online users"),
                "UserListResponse",
            ),
        );

        paths_map.insert(
            "/api/upload/register".to_string(),
            path_item_with_body(
                "post",
                "Register file upload",
                Some("Register a new file upload task"),
                "FileChunkData",
            ),
        );
        paths_map.insert(
            "/api/upload/save".to_string(),
            path_item_with_body(
                "post",
                "Save file chunk",
                Some("Save uploaded file chunk data"),
                "FileChunkData",
            ),
        );
        paths_map.insert(
            "/api/upload/merge".to_string(),
            path_item_with_body(
                "post",
                "Merge file chunks",
                Some("Merge all chunks into a complete file"),
                "FileChunkData",
            ),
        );

        paths_map.insert(
            "/api/log/info".to_string(),
            path_item(
                "get",
                "Get info logs",
                Some("Returns system info level log information"),
            ),
        );
        paths_map.insert(
            "/api/log/warn".to_string(),
            path_item(
                "get",
                "Get warn logs",
                Some("Returns system warn level log information"),
            ),
        );
        paths_map.insert(
            "/api/log/error".to_string(),
            path_item(
                "get",
                "Get error logs",
                Some("Returns system error level log information"),
            ),
        );

        paths_map.insert(
            "/api/monitor/status-sse".to_string(),
            path_item(
                "get",
                "Server status SSE stream",
                Some("Real-time server status updates via Server-Sent Events"),
            ),
        );
        paths_map.insert(
            "/api/monitor/system-info".to_string(),
            path_item_with_response(
                "get",
                "Get system information",
                Some("Returns server system information including CPU, memory, disk, etc."),
                "SystemInfo",
            ),
        );
        paths_map.insert(
            "/api/monitor/network-capture-data".to_string(),
            path_item_with_response(
                "get",
                "Get network capture data",
                Some("Returns network packet capture statistics"),
                "NetworkStats",
            ),
        );
        paths_map.insert(
            "/api/monitor/network-capture-stream".to_string(),
            path_item(
                "get",
                "Network capture SSE stream",
                Some("Real-time network packet information via Server-Sent Events"),
            ),
        );

        paths_map.insert(
            "/api/mysql/records".to_string(),
            path_item(
                "get",
                "List all MySQL records",
                Some("List all MySQL records in the database"),
            ),
        );
        paths_map.insert(
            "/api/mysql/record".to_string(),
            path_item(
                "post",
                "Create MySQL record",
                Some("Create a new MySQL record in the database"),
            ),
        );
        paths_map.insert(
            "/api/mysql/record/update".to_string(),
            path_item(
                "post",
                "Update MySQL record",
                Some("Update an existing MySQL record in the database"),
            ),
        );
        paths_map.insert(
            "/api/mysql/record/delete".to_string(),
            path_item(
                "post",
                "Delete MySQL record",
                Some("Delete a specified MySQL record from the database"),
            ),
        );

        paths_map.insert(
            "/api/postgresql/records".to_string(),
            path_item(
                "get",
                "List all PostgreSQL records",
                Some("List all PostgreSQL records in the database"),
            ),
        );
        paths_map.insert(
            "/api/postgresql/record".to_string(),
            path_item(
                "post",
                "Create PostgreSQL record",
                Some("Create a new PostgreSQL record in the database"),
            ),
        );
        paths_map.insert(
            "/api/postgresql/record/update".to_string(),
            path_item(
                "post",
                "Update PostgreSQL record",
                Some("Update an existing PostgreSQL record in the database"),
            ),
        );
        paths_map.insert(
            "/api/postgresql/record/delete".to_string(),
            path_item(
                "post",
                "Delete PostgreSQL record",
                Some("Delete a specified PostgreSQL record from the database"),
            ),
        );

        paths_map.insert(
            "/api/redis/records".to_string(),
            path_item(
                "get",
                "List all Redis records",
                Some("List all records in Redis"),
            ),
        );
        paths_map.insert(
            "/api/redis/record".to_string(),
            path_item(
                "post",
                "Create Redis record",
                Some("Create a new record in Redis"),
            ),
        );
        paths_map.insert(
            "/api/redis/record/update".to_string(),
            path_item(
                "post",
                "Update Redis record",
                Some("Update an existing record in Redis"),
            ),
        );
        paths_map.insert(
            "/api/redis/record/delete".to_string(),
            path_item(
                "post",
                "Delete Redis record",
                Some("Delete a specified record from Redis"),
            ),
        );

        paths_map.insert(
            "/api/sse".to_string(),
            path_item(
                "get",
                "Server-Sent Events stream",
                Some("Establish SSE connection to receive real-time server events"),
            ),
        );
        paths_map.insert(
            "/api/websocket".to_string(),
            path_item(
                "get",
                "WebSocket connection",
                Some("Establish WebSocket bidirectional communication connection"),
            ),
        );

        paths_map.insert(
            "/api/trace/{trace}".to_string(),
            path_item_with_param(
                "get",
                "Search trace logs",
                Some("Search log information by trace ID"),
                "trace",
                "Trace ID",
            ),
        );

        paths_map.insert(
            "/openapi/openapi.json".to_string(),
            path_item(
                "get",
                "OpenAPI JSON specification",
                Some("Returns the complete OpenAPI 3.0 specification document"),
            ),
        );

        openapi.paths.paths = paths_map;
    }
}
