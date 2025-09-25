use super::*;

#[route("/static/{path:.*}")]
#[utoipa::path(
    get,
    path = "/static/{path:.*}",
    params(
        ("path" = String, Path, description = "资源文件路径")
    ),
    responses(
        (status = 200, description = "静态资源文件", body = Vec<u8>),
        (status = 304, description = "未修改"),
        (status = 400, description = "请求错误 - 无效路径"),
        (status = 403, description = "禁止访问"),
        (status = 404, description = "文件未找到"),
        (status = 413, description = "文件过大"),
        (status = 500, description = "服务器内部错误")
    )
)]
#[prologue_hooks(
    methods(get),
    route_param(STATIC_PATH_KEY => path_opt),
    request_header("Range" => range_opt)
)]
pub async fn serve_static_resource(ctx: Context) {
    let path: String = match path_opt {
        Some(p) => p,
        None => {
            println_warning!("❌ 路径参数为空");
            send_error_response(&ctx, 400, ERROR_INVALID_PATH).await;
            return;
        }
    };

    println_success!("📁 处理静态文件请求: ", path);

    let resource_config: ResourceConfig = ResourceConfig::static_resources();

    println_success!("🔧 使用配置: ", resource_config.base_dir);
    match serve_unified_resource_file(&resource_config, &path).await {
        Ok(response) => {
            println_success!(
                "✅ 文件读取成功: ",
                response.file_size,
                " 字节, Content-Type: ",
                response.content_type
            );

            if let Some(range_header) = range_opt {
                println_success!("📊 处理范围请求: ", range_header);
                handle_range_request(&ctx, &response, &range_header).await;
                return;
            }

            println_success!("📤 发送完整响应");
            send_success_response(&ctx, &response).await;
        }

        Err(error) => {
            println_warning!("❌ 静态文件错误: ", error);
            handle_static_file_error(&ctx, error).await;
        }
    }
}

async fn send_success_response(ctx: &Context, response: &StaticFileResponse) {
    ctx.set_response_status_code(200).await;
    ctx.set_response_body(&response.content).await;
    ctx.set_response_header("Content-Type", &response.content_type)
        .await;
    ctx.set_response_header("Content-Length", &response.file_size.to_string())
        .await;
    ctx.set_response_header("Accept-Ranges", "bytes").await;
}

async fn handle_range_request(ctx: &Context, response: &StaticFileResponse, range_header: &str) {
    match parse_range_header(range_header, response.file_size) {
        Ok(ranges) => {
            if ranges.len() == 1 {
                let range = &ranges[0];
                let content_slice: &[u8] = &response.content[range.start..=range.end];

                ctx.set_response_status_code(206).await;
                ctx.set_response_body(content_slice).await;

                ctx.set_response_header("Content-Type", &response.content_type)
                    .await;
                ctx.set_response_header("Content-Length", &content_slice.len().to_string())
                    .await;
                ctx.set_response_header(
                    "Content-Range",
                    &format!("bytes {}-{}/{}", range.start, range.end, response.file_size),
                )
                .await;
            } else {
                send_success_response(ctx, response).await;
            }
        }
        Err(_) => {
            ctx.set_response_status_code(416).await;
            ctx.set_response_header("Content-Range", &format!("bytes */{}", response.file_size))
                .await;
        }
    }
}

async fn handle_static_file_error(ctx: &Context, error: StaticFileError) {
    let (status_code, error_message): (u16, &str) = match error {
        StaticFileError::NotFound => (404, ERROR_FILE_NOT_FOUND),
        StaticFileError::Forbidden => (403, ERROR_ACCESS_DENIED),
        StaticFileError::SecurityViolation => (400, ERROR_PATH_TRAVERSAL),
        StaticFileError::FileTooLarge => (413, ERROR_FILE_TOO_LARGE),
        StaticFileError::InvalidPath => (400, ERROR_INVALID_PATH),
        StaticFileError::IoError(_) => (500, "Internal Server Error"),
    };

    send_error_response(ctx, status_code, error_message).await;
}

async fn send_error_response(ctx: &Context, status_code: u16, message: &str) {
    ctx.set_response_status_code(status_code as usize).await;
    ctx.set_response_header("Content-Type", "text/plain; charset=utf-8")
        .await;
    ctx.set_response_header("Cache-Control", CACHE_CONTROL_NO_CACHE)
        .await;
    ctx.set_response_body(message.as_bytes()).await;
}

#[route("/")]
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "首页", body = String)
    )
)]
#[prologue_hooks(methods(get))]
pub async fn serve_index_page(ctx: Context) {
    println_success!("🏠 首页请求");
    serve_template_html(&ctx, "resources/templates/html/index.html").await;
}

#[route("/upload")]
#[utoipa::path(
    get,
    path = "/upload",
    responses(
        (status = 200, description = "上传页面", body = String)
    )
)]
#[prologue_hooks(methods(get, post))]
pub async fn serve_upload_page(ctx: Context) {
    println_success!("📤 上传页面请求");
    serve_upload_html(&ctx).await;
}

#[route("/monitor")]
#[utoipa::path(
    get,
    path = "/monitor",
    responses(
        (status = 200, description = "监控面板", body = String)
    )
)]
#[prologue_hooks(methods(get, post))]
pub async fn serve_monitor_page(ctx: Context) {
    println_success!("📊 监控面板请求");
    serve_monitor_html(&ctx).await;
}

#[route("/openapi")]
#[utoipa::path(
    get,
    path = "/openapi",
    responses(
        (status = 200, description = "API文档", body = String)
    )
)]
#[prologue_hooks(methods(get, post))]
pub async fn serve_openapi_page(ctx: Context) {
    println_success!("📚 OpenAPI文档请求");
    serve_openapi_html(&ctx).await;
}

#[route("/{chat_path:^chat.*}")]
#[utoipa::path(
    get,
    path = "/chat/{path}",
    params(
        ("path" = String, Path, description = "聊天页面路径")
    ),
    responses(
        (status = 200, description = "聊天页面", body = String),
        (status = 301, description = "重定向")
    )
)]
#[prologue_hooks(
    methods(get, post),
    route_param("chat_path" => chat_path_opt)
)]
pub async fn serve_chat_page(ctx: Context) {
    let chat_path: String = chat_path_opt.unwrap_or_default();
    println_success!("💬 聊天页面请求: ", chat_path);
    serve_chat_html(&ctx, &chat_path).await;
}

#[route("/{page_path:.*}")]
#[utoipa::path(
    get,
    path = "/{page_path}",
    params(
        ("page_path" = String, Path, description = "页面路径")
    ),
    responses(
        (status = 200, description = "HTML页面", body = String),
        (status = 404, description = "页面未找到")
    )
)]
#[prologue_hooks(
    methods(get, post),
    route_param("page_path" => page_path_opt)
)]
pub async fn serve_fallback_page(ctx: Context) {
    let page_path: String = page_path_opt.unwrap_or_default();
    println_success!("🌐 通用页面请求: ", page_path);

    let html_path: String = if page_path.ends_with(".html") {
        format!("resources/static/{}", page_path)
    } else {
        format!("resources/static/{}.html", page_path)
    };

    if std::path::Path::new(&html_path).exists() {
        serve_template_html(&ctx, &html_path).await;
    } else {
        ctx.set_response_status_code(404).await;
        serve_template_html(&ctx, "resources/static/html/404.html").await;
    }
}

async fn serve_template_html(ctx: &Context, file_path: &str) {
    println_success!("📄 服务HTML文件: ", file_path);
    match std::fs::read_to_string(file_path) {
        Ok(content) => {
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
                .await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(content.as_bytes()).await;
        }
        Err(e) => {
            println_warning!("❌ 读取HTML文件失败 ", file_path, ": ", e);
            ctx.set_response_status_code(404).await;
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
                .await;
            ctx.set_response_body(b"<h1>404 Not Found</h1>").await;
        }
    }
}

async fn serve_upload_html(ctx: &Context) {
    let upload_html: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>文件上传 - Hyperlane</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; margin-bottom: 20px; }
        .upload-area { border: 2px dashed #ddd; padding: 40px; text-align: center; border-radius: 8px; margin: 20px 0; }
        .upload-area:hover { border-color: #007bff; background: #f8f9fa; }
        .btn { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        .btn:hover { background: #0056b3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📤 文件上传</h1>
        <div class="upload-area">
            <p>拖拽文件到此处或点击选择文件</p>
            <input type="file" id="fileInput" multiple style="display: none;">
            <button class="btn" onclick="document.getElementById('fileInput').click()">选择文件</button>
        </div>
        <div id="fileList"></div>
        <p><a href="/">← 返回首页</a></p>
    </div>
    <script>
        document.getElementById('fileInput').addEventListener('change', function(e) {
            const files = e.target.files;
            const fileList = document.getElementById('fileList');
            fileList.innerHTML = '<h3>选中的文件:</h3>';
            for (let i = 0; i < files.length; i++) {
                fileList.innerHTML += '<p>' + files[i].name + ' (' + (files[i].size / 1024).toFixed(2) + ' KB)</p>';
            }
        });
    </script>
</body>
</html>"#;

    ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
        .await;
    ctx.set_response_status_code(200).await;
    ctx.set_response_body(upload_html.as_bytes()).await;
}

async fn serve_monitor_html(ctx: &Context) {
    let monitor_html: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>服务器监控 - Hyperlane</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; }
        .card { background: white; padding: 20px; margin: 10px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        h1 { color: #333; text-align: center; }
        .metric { display: flex; justify-content: space-between; margin: 10px 0; }
        .status { padding: 4px 8px; border-radius: 4px; color: white; background: #28a745; }
        .refresh-btn { background: #007bff; color: white; padding: 8px 16px; border: none; border-radius: 4px; cursor: pointer; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📊 服务器监控面板</h1>
        <div class="card" style="text-align: center; margin-bottom: 20px;">
            <span class="status">🟢 服务器运行正常</span>
            <button class="refresh-btn" onclick="location.reload()">刷新数据</button>
        </div>
        <div class="grid">
            <div class="card">
                <h3>系统信息</h3>
                <div class="metric"><span>操作系统:</span><span>Windows</span></div>
                <div class="metric"><span>运行时间:</span><span id="uptime">计算中...</span></div>
                <div class="metric"><span>CPU使用率:</span><span id="cpu">-</span></div>
                <div class="metric"><span>内存使用:</span><span id="memory">-</span></div>
            </div>
            <div class="card">
                <h3>网络状态</h3>
                <div class="metric"><span>活跃连接:</span><span id="connections">-</span></div>
                <div class="metric"><span>请求总数:</span><span id="requests">-</span></div>
                <div class="metric"><span>响应时间:</span><span id="response-time">-</span></div>
            </div>
            <div class="card">
                <h3>服务状态</h3>
                <div class="metric"><span>HTTP服务:</span><span class="status">运行中</span></div>
                <div class="metric"><span>WebSocket:</span><span class="status">运行中</span></div>
                <div class="metric"><span>静态文件:</span><span class="status">运行中</span></div>
            </div>
        </div>
        <div class="card" style="text-align: center; margin-top: 20px;">
            <p><a href="/">← 返回首页</a> | <a href="/api/server/status">实时数据流</a></p>
        </div>
    </div>
    <script>
        // 简单的客户端监控逻辑
        function updateUptime() {
            const start = Date.now();
            document.getElementById('uptime').textContent = '刚刚启动';
        }
        updateUptime();
        
        // 模拟数据更新
        setInterval(() => {
            document.getElementById('cpu').textContent = (Math.random() * 20 + 5).toFixed(1) + '%';
            document.getElementById('memory').textContent = (Math.random() * 30 + 40).toFixed(1) + '%';
            document.getElementById('connections').textContent = Math.floor(Math.random() * 50 + 10);
            document.getElementById('requests').textContent = Math.floor(Math.random() * 1000 + 500);
            document.getElementById('response-time').textContent = (Math.random() * 50 + 10).toFixed(0) + 'ms';
        }, 2000);
    </script>
</body>
</html>"#;

    ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
        .await;
    ctx.set_response_status_code(200).await;
    ctx.set_response_body(monitor_html.as_bytes()).await;
}

async fn serve_openapi_html(ctx: &Context) {
    let openapi_html: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>API 文档 - Hyperlane</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 1000px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); }
        h1 { color: #333; margin-bottom: 20px; }
        .api-section { margin: 20px 0; padding: 20px; border: 1px solid #ddd; border-radius: 8px; }
        .endpoint { background: #f8f9fa; padding: 15px; margin: 10px 0; border-radius: 4px; }
        .method { display: inline-block; padding: 4px 8px; border-radius: 4px; color: white; font-weight: bold; margin-right: 10px; }
        .get { background: #28a745; }
        .post { background: #007bff; }
        .put { background: #ffc107; color: #333; }
        .delete { background: #dc3545; }
        .link-btn { display: inline-block; background: #007bff; color: white; padding: 8px 16px; text-decoration: none; border-radius: 4px; margin: 5px; }
        .link-btn:hover { background: #0056b3; }
    </style>
</head>
<body>
    <div class="container">
        <h1>📚 API 文档</h1>
        
        <div class="api-section">
            <h2>快速链接</h2>
            <a href="/openapi/openapi.json" class="link-btn">📄 OpenAPI JSON</a>
            <a href="/api/server/status" class="link-btn">📊 服务器状态</a>
            <a href="/api/server/info" class="link-btn">ℹ️ 系统信息</a>
        </div>

        <div class="api-section">
            <h2>主要 API 端点</h2>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/api/server/status</strong>
                <p>获取服务器实时状态 (SSE 流)</p>
            </div>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/api/server/info</strong>
                <p>获取系统信息</p>
            </div>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/api/users/online</strong>
                <p>获取在线用户列表</p>
            </div>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <strong>/api/upload/register</strong>
                <p>注册文件上传</p>
            </div>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <strong>/api/upload/save</strong>
                <p>保存文件块</p>
            </div>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <strong>/api/upload/merge</strong>
                <p>合并文件块</p>
            </div>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/api/chat</strong>
                <p>WebSocket 聊天连接</p>
            </div>
        </div>

        <div class="api-section">
            <h2>认证 API</h2>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <strong>/auth/login</strong>
                <p>用户登录</p>
            </div>
            
            <div class="endpoint">
                <span class="method post">POST</span>
                <strong>/auth/logout</strong>
                <p>用户登出</p>
            </div>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/auth/session</strong>
                <p>获取会话信息</p>
            </div>
        </div>

        <div class="api-section">
            <h2>静态资源</h2>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/static/{path}</strong>
                <p>访问静态资源文件</p>
            </div>
            
            <div class="endpoint">
                <span class="method get">GET</span>
                <strong>/static/{upload_dir}/{upload_file}</strong>
                <p>访问上传的文件</p>
            </div>
        </div>

        <div style="text-align: center; margin-top: 30px;">
            <p><a href="/">← 返回首页</a></p>
        </div>
    </div>
</body>
</html>"#;

    ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
        .await;
    ctx.set_response_status_code(200).await;
    ctx.set_response_body(openapi_html.as_bytes()).await;
}

async fn serve_chat_html(ctx: &Context, path: &str) {
    if path.len() <= 5 {
        ctx.set_response_status_code(301).await;
        ctx.set_response_header("Location", "/chat/index").await;
        ctx.set_response_body(&vec![]).await;
        return;
    }

    let file_path: String = format!("./chat/{}", path);
    match std::fs::read(&file_path) {
        Ok(content) => {
            let content_type: &str = if path.ends_with(".html") {
                "text/html; charset=utf-8"
            } else if path.ends_with(".css") {
                "text/css; charset=utf-8"
            } else if path.ends_with(".js") {
                "application/javascript; charset=utf-8"
            } else {
                "application/octet-stream"
            };

            ctx.set_response_header("Content-Type", content_type).await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(&content).await;
        }
        Err(_) => {
            let chat_html: &str = r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>聊天室 - Hyperlane</title>
    <style>
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background: #f5f5f5; }
        .container { max-width: 800px; margin: 0 auto; background: white; border-radius: 8px; box-shadow: 0 2px 10px rgba(0,0,0,0.1); overflow: hidden; }
        .header { background: #007bff; color: white; padding: 20px; text-align: center; }
        .chat-area { height: 400px; padding: 20px; overflow-y: auto; border-bottom: 1px solid #ddd; }
        .input-area { padding: 20px; display: flex; gap: 10px; }
        .message-input { flex: 1; padding: 10px; border: 1px solid #ddd; border-radius: 4px; }
        .send-btn { background: #007bff; color: white; padding: 10px 20px; border: none; border-radius: 4px; cursor: pointer; }
        .send-btn:hover { background: #0056b3; }
        .message { margin: 10px 0; padding: 10px; background: #f8f9fa; border-radius: 4px; }
        .status { text-align: center; color: #666; margin: 20px 0; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>💬 聊天室</h1>
            <p>实时聊天应用</p>
        </div>
        <div class="chat-area" id="chatArea">
            <div class="status">连接到聊天服务器...</div>
        </div>
        <div class="input-area">
            <input type="text" class="message-input" id="messageInput" placeholder="输入消息..." disabled>
            <button class="send-btn" id="sendBtn" onclick="sendMessage()" disabled>发送</button>
        </div>
    </div>
    
    <div style="text-align: center; margin-top: 20px;">
        <p><a href="/">← 返回首页</a></p>
    </div>

    <script>
        let ws = null;
        const chatArea = document.getElementById('chatArea');
        const messageInput = document.getElementById('messageInput');
        const sendBtn = document.getElementById('sendBtn');

        function connectWebSocket() {
            try {
                const protocol = window.location.protocol === 'https:' ? 'wss:' : 'ws:';
                ws = new WebSocket(protocol + '//' + window.location.host + '/api/chat');
                
                ws.onopen = function() {
                    chatArea.innerHTML = '<div class="status">✅ 已连接到聊天服务器</div>';
                    messageInput.disabled = false;
                    sendBtn.disabled = false;
                };
                
                ws.onmessage = function(event) {
                    const message = document.createElement('div');
                    message.className = 'message';
                    message.textContent = event.data;
                    chatArea.appendChild(message);
                    chatArea.scrollTop = chatArea.scrollHeight;
                };
                
                ws.onclose = function() {
                    chatArea.innerHTML += '<div class="status">❌ 连接已断开</div>';
                    messageInput.disabled = true;
                    sendBtn.disabled = true;
                };
                
                ws.onerror = function() {
                    chatArea.innerHTML += '<div class="status">⚠️ 连接错误</div>';
                };
            } catch (e) {
                chatArea.innerHTML = '<div class="status">❌ WebSocket 不可用，请检查服务器配置</div>';
            }
        }

        function sendMessage() {
            const message = messageInput.value.trim();
            if (message && ws && ws.readyState === WebSocket.OPEN) {
                ws.send(message);
                messageInput.value = '';
            }
        }

        messageInput.addEventListener('keypress', function(e) {
            if (e.key === 'Enter') {
                sendMessage();
            }
        });

        // 尝试连接WebSocket
        connectWebSocket();
    </script>
</body>
</html>"#;
            ctx.set_response_header("Content-Type", "text/html; charset=utf-8")
                .await;
            ctx.set_response_status_code(200).await;
            ctx.set_response_body(chat_html.as_bytes()).await;
        }
    }
}
