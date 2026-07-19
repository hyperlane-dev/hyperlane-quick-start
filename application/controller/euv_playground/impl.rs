use super::*;

/// Implementation of `EuvPlaygroundProjectsListRoute` for `ServerHook`.
impl ServerHook for EuvPlaygroundProjectsListRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let dir: std::path::PathBuf = EuvPlaygroundService::user_dir(user_id);
        let mut items: Vec<EuvPlaygroundProjectListItem> = Vec::new();
        let entries: std::fs::ReadDir = match std::fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => {
                let resp: ApiResponse<Vec<EuvPlaygroundProjectListItem>> =
                    ApiResponse::new(ApiResponseStatus::Success, items);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        for entry in entries.flatten() {
            let p: std::path::PathBuf = entry.path();
            if !p.is_dir() {
                continue;
            }
            let id_str: &str = match p.file_name().and_then(|n: &std::ffi::OsStr| n.to_str()) {
                Some(s) => s,
                None => continue,
            };
            // The on-disk directory name is the URL-encoded form of
            // the project id (via `EuvPlaygroundService::encode_id`).
            // `decode_id` falls back to a plain `i64` parse for
            // backward compatibility with the (legacy) un-encoded
            // layout.
            let id: i64 = match EuvPlaygroundService::decode_id(id_str) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let (name, updated_at_ms): (String, i64) = EuvPlaygroundService::read_metadata(&p)
                .unwrap_or_else(|| ("Untitled".to_string(), 0));
            let code_size: u64 = std::fs::metadata(p.join(EUV_PLAYGROUND_CODE_FILE))
                .map(|m: std::fs::Metadata| m.len())
                .unwrap_or(0);
            let mut item: EuvPlaygroundProjectListItem = EuvPlaygroundProjectListItem::default();
            item.set_id(EuvPlaygroundService::encode_id(id))
                .set_name(name)
                .set_updated_at_ms(updated_at_ms)
                .set_code_size(code_size);
            items.push(item);
        }
        items.sort_by(
            |a: &EuvPlaygroundProjectListItem, b: &EuvPlaygroundProjectListItem| {
                b.get_updated_at_ms().cmp(a.get_updated_at_ms())
            },
        );
        items.truncate(EUV_PLAYGROUND_MAX_LIST_ITEMS);
        let resp: ApiResponse<Vec<EuvPlaygroundProjectListItem>> =
            ApiResponse::new(ApiResponseStatus::Success, items);
        ctx.get_mut_response().set_body(resp.to_json_bytes());
        Status::Continue
    }
}

/// Project create — POST /api/euv/playground/projects/create
impl ServerHook for EuvPlaygroundProjectsCreateRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(post),
        request_body_json_result(request_opt: EuvPlaygroundProjectCreateRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let request: EuvPlaygroundProjectCreateRequest = match request_opt {
            Ok(r) => r,
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, e.to_string());
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let name: String = EuvPlaygroundService::normalize_name(request.get_name());
        let user_root: std::path::PathBuf = EuvPlaygroundService::user_dir(user_id);
        match EuvPlaygroundService::project_name_exists(&user_root, &name) {
            Ok(true) => {
                let error: String = format!("Project name \"{name}\" already exists");
                let mut resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Conflict, error.clone());
                resp.set_message(error);
                ctx.get_mut_response()
                    .set_status_code(i32::from(ApiResponseStatus::Conflict) as usize)
                    .set_body(resp.to_json_bytes());
                return Status::Continue;
            }
            Ok(false) => {}
            Err(error) => {
                let mut resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InternalServerError, error.clone());
                resp.set_message(error);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        }
        let id: i64 = EuvPlaygroundService::next_project_id(&user_root);
        let pdir: std::path::PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        match EuvPlaygroundService::write_project(&pdir, &name, EUV_PLAYGROUND_DEFAULT_CODE) {
            Ok(ts) => {
                let mut payload: EuvPlaygroundProjectMutationResponse =
                    EuvPlaygroundProjectMutationResponse::default();
                payload
                    .set_id(EuvPlaygroundService::encode_id(id))
                    .set_name(name)
                    .set_updated_at_ms(ts)
                    .set_deleted(false);
                let resp: ApiResponse<EuvPlaygroundProjectMutationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, e);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Project get — GET /api/euv/playground/projects/get/{id}
impl ServerHook for EuvPlaygroundProjectsGetRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(s) => s,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("missing project id"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        // `id_str` is the raw path-segment value (already URL-encoded by
        // whichever side produced the id, so decode it back to its
        // numeric form before parsing). The same encoding convention
        // is used by the `auth` and `rss` services.
        let id: i64 = match EuvPlaygroundService::decode_id(&id_str) {
            Ok(v) => v,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("project id is not a number"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: std::path::PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from("project not found"),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        let code: String = match EuvPlaygroundService::read_code(&pdir) {
            Ok(c) => c,
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, e);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let (name, updated_at_ms) = EuvPlaygroundService::read_metadata(&pdir)
            .unwrap_or_else(|| ("Untitled".to_string(), EuvPlaygroundService::now_ms()));
        let mut payload: EuvPlaygroundProjectDetail = EuvPlaygroundProjectDetail::default();
        payload
            .set_id(EuvPlaygroundService::encode_id(id))
            .set_name(name)
            .set_code(code)
            .set_updated_at_ms(updated_at_ms);
        let resp: ApiResponse<EuvPlaygroundProjectDetail> =
            ApiResponse::new(ApiResponseStatus::Success, payload);
        ctx.get_mut_response().set_body(resp.to_json_bytes());
        Status::Continue
    }
}

/// Project save — PUT /api/euv/playground/projects/save/{id}
impl ServerHook for EuvPlaygroundProjectsSaveRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(put),
        try_get_route_param(ID_KEY => id_opt),
        request_body_json_result(request_opt: EuvPlaygroundProjectSaveRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(s) => s,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("missing project id"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        // `id_str` is the raw path-segment value (already URL-encoded by
        // whichever side produced the id, so decode it back to its
        // numeric form before parsing). The same encoding convention
        // is used by the `auth` and `rss` services.
        let id: i64 = match EuvPlaygroundService::decode_id(&id_str) {
            Ok(v) => v,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("project id is not a number"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let request: EuvPlaygroundProjectSaveRequest = match request_opt {
            Ok(r) => r,
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, e.to_string());
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: std::path::PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from("project not found"),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        // Resolve the final name + code from existing state + overrides.
        let (cur_name, _cur_ts): (String, i64) = EuvPlaygroundService::read_metadata(&pdir)
            .unwrap_or_else(|| ("Untitled".to_string(), EuvPlaygroundService::now_ms()));
        let name_trim: String = request.get_name().trim().to_string();
        let new_name: String = if name_trim.is_empty() {
            cur_name
        } else {
            EuvPlaygroundService::normalize_name(&name_trim)
        };
        let user_root: std::path::PathBuf = EuvPlaygroundService::user_dir(user_id);
        match EuvPlaygroundService::project_name_exists_excluding(
            &user_root,
            &new_name,
            Some(&pdir),
        ) {
            Ok(true) => {
                let error: String = format!("Project name \"{new_name}\" already exists");
                let mut resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Conflict, error.clone());
                resp.set_message(error);
                ctx.get_mut_response()
                    .set_status_code(i32::from(ApiResponseStatus::Conflict) as usize)
                    .set_body(resp.to_json_bytes());
                return Status::Continue;
            }
            Ok(false) => {}
            Err(error) => {
                let mut resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InternalServerError, error.clone());
                resp.set_message(error);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        }
        let code_opt: Option<String> = match request.try_get_code() {
            Some(c) if !c.is_empty() => Some(c.to_string()),
            _ => None,
        };
        let final_code: String = match code_opt {
            Some(c) => {
                if c.len() > EUV_PLAYGROUND_MAX_CODE_BYTES {
                    let resp: ApiResponse<String> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        format!(
                            "code exceeds {} bytes (got {})",
                            EUV_PLAYGROUND_MAX_CODE_BYTES,
                            c.len()
                        ),
                    );
                    ctx.get_mut_response().set_body(resp.to_json_bytes());
                    return Status::Continue;
                }
                c
            }
            None => match EuvPlaygroundService::read_code(&pdir) {
                Ok(c) => c,
                Err(e) => {
                    let resp: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, e);
                    ctx.get_mut_response().set_body(resp.to_json_bytes());
                    return Status::Continue;
                }
            },
        };
        match EuvPlaygroundService::write_project(&pdir, &new_name, &final_code) {
            Ok(ts) => {
                let mut payload = EuvPlaygroundProjectMutationResponse::default();
                payload
                    .set_id(EuvPlaygroundService::encode_id(id))
                    .set_name(new_name)
                    .set_updated_at_ms(ts)
                    .set_deleted(false);
                let resp: ApiResponse<EuvPlaygroundProjectMutationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, e);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Project delete — DELETE /api/euv/playground/projects/delete/{id}
impl ServerHook for EuvPlaygroundProjectsDeleteRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(delete),
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(s) => s,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("missing project id"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        // `id_str` is the raw path-segment value (already URL-encoded by
        // whichever side produced the id, so decode it back to its
        // numeric form before parsing). The same encoding convention
        // is used by the `auth` and `rss` services.
        let id: i64 = match EuvPlaygroundService::decode_id(&id_str) {
            Ok(v) => v,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from("project id is not a number"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: std::path::PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from("project not found"),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        let (name, _ts): (String, i64) = EuvPlaygroundService::read_metadata(&pdir)
            .unwrap_or_else(|| ("Untitled".to_string(), EuvPlaygroundService::now_ms()));
        match std::fs::remove_dir_all(&pdir) {
            Ok(_) => {
                let mut payload = EuvPlaygroundProjectMutationResponse::default();
                payload
                    .set_id(EuvPlaygroundService::encode_id(id))
                    .set_name(name)
                    .set_updated_at_ms(EuvPlaygroundService::now_ms())
                    .set_deleted(true);
                let resp: ApiResponse<EuvPlaygroundProjectMutationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(e) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::BusinessLogicError,
                    format!("Failed to delete project: {e}"),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Default code — GET /api/euv/playground/default-code
///
/// Returns the canonical starter template the server uses when seeding a
/// brand-new playground project. The endpoint is unauthenticated because
/// the template is identical for every user — serving it from a logged-in
/// route would only add cookie noise to a request that has to run before
/// the user has selected or created a project.
impl ServerHook for EuvPlaygroundDefaultCodeRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let mut payload: EuvPlaygroundDefaultCodeResponse =
            EuvPlaygroundDefaultCodeResponse::default();
        payload.set_code(EUV_PLAYGROUND_DEFAULT_CODE.to_string());
        let resp: ApiResponse<EuvPlaygroundDefaultCodeResponse> =
            ApiResponse::new(ApiResponseStatus::Success, payload);
        ctx.get_mut_response().set_body(resp.to_json_bytes());
        Status::Continue
    }
}

/// Run — POST /api/euv/playground/run (compile + publish wasm)
impl ServerHook for EuvPlaygroundRunRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(post),
        request_body_json_result(request_opt: EuvPlaygroundRunRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let request: EuvPlaygroundRunRequest = match request_opt {
            Ok(r) => r,
            Err(e) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, e.to_string());
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let project_id: i64 = match EuvPlaygroundService::decode_id(request.get_project_id()) {
            Ok(project_id) => project_id,
            Err(error) => {
                let response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: std::path::PathBuf = EuvPlaygroundService::project_dir(user_id, project_id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from("project not found"),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        // Use the override code if present, else load from disk.
        let code: String = match request.try_get_code() {
            Some(code) => code.clone(),
            None => match EuvPlaygroundService::read_code(&pdir) {
                Ok(code) => code,
                Err(error) => {
                    let response: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, error);
                    ctx.get_mut_response().set_body(response.to_json_bytes());
                    return Status::Continue;
                }
            },
        };
        if code.len() > EUV_PLAYGROUND_MAX_CODE_BYTES {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::InvalidRequest,
                format!(
                    "code exceeds {} bytes (got {})",
                    EUV_PLAYGROUND_MAX_CODE_BYTES,
                    code.len()
                ),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        match EuvPlaygroundService::build_wasm_pack_output(&code, project_id).await {
            Ok(_target_dir) => {
                // `project_id` here is the raw i64 — the on-disk path uses
                // the encoded form (see `build_wasm_pack_output`), so the
                // URL the frontend loads must match.
                let build_url: String = format!(
                    "/static/euv-playground/tmp/{}/index.html",
                    EuvPlaygroundService::encode_id(project_id),
                );
                let mut payload: EuvPlaygroundRunResponse = EuvPlaygroundRunResponse::default();
                payload
                    .set_ok(true)
                    .set_html(String::new())
                    .set_js(String::new())
                    .set_wasm(String::new())
                    .set_stderr(String::new())
                    .set_build_url(build_url);
                let resp: ApiResponse<EuvPlaygroundRunResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(stderr) => {
                let mut payload = EuvPlaygroundRunResponse::default();
                payload
                    .set_ok(false)
                    .set_html(String::new())
                    .set_js(String::new())
                    .set_wasm(String::new())
                    .set_stderr(stderr);
                let resp: ApiResponse<EuvPlaygroundRunResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// EuvPlaygroundHelpers — zero-sized struct whose impl block holds
/// controller-side helpers (`require_user`, request validation, etc.).
/// Methods do not depend on `self`; callers reach them as
/// `EuvPlaygroundHelpers::require_user(ctx)`. The wasm-pack runner
/// helpers below also live in this file (they shell out to
/// `wasm-pack build --target web` and don't fit the `ServerHook` trait).
impl EuvPlaygroundHelpers {
    /// Helper — try to extract the current user id from the cookie. Returns
    /// the id on success, or writes a 401 JSON envelope to the response and
    /// returns `None`.
    pub fn require_user(ctx: &mut Context) -> Option<i32> {
        match AuthService::extract_user_from_cookie(ctx) {
            Ok(id) => Some(id),
            Err(error) => {
                let mut response: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::Unauthorized, error.clone());
                response.set_message(&error);
                ctx.get_mut_response().set_body(response.to_json_bytes());
                None
            }
        }
    }
}
