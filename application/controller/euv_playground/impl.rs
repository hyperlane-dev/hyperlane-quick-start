use super::*;

/// Implementation of `EuvPlaygroundProjectsListRoute` for `ServerHook`.
impl ServerHook for EuvPlaygroundProjectsListRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `EuvPlaygroundProjectsListRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundProjectsListRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let dir: PathBuf = EuvPlaygroundService::user_dir(user_id);
        let mut items: Vec<EuvPlaygroundProjectListItem> = Vec::new();
        let entries: ReadDir = match read_dir(&dir) {
            Ok(read_dir) => read_dir,
            Err(_) => {
                let resp: ApiResponse<Vec<EuvPlaygroundProjectListItem>> =
                    ApiResponse::new(ApiResponseStatus::Success, items);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        for entry in entries.flatten() {
            let path: PathBuf = entry.path();
            if !path.is_dir() {
                continue;
            }
            let id_str: &str = match path
                .file_name()
                .and_then(|os_name: &OsStr| os_name.to_str())
            {
                Some(file_name) => file_name,
                None => continue,
            };
            // The on-disk directory name is the URL-encoded form of
            // the project id (via `EuvPlaygroundService::encode_id`).
            // `decode_id` falls back to a plain `i64` parse for
            // backward compatibility with the (legacy) un-encoded
            // layout.
            let id: i64 = match EuvPlaygroundService::decode_id(id_str) {
                Ok(parsed) => parsed,
                Err(_) => continue,
            };
            let (name, updated_at_ms): (String, i64) = EuvPlaygroundService::read_metadata(&path)
                .unwrap_or_else(|| (CONTROLLER_UNTITLED_PROJECT_NAME.to_string(), 0));
            let code_size: u64 = metadata(path.join(EUV_PLAYGROUND_CODE_FILE))
                .map(|metadata: Metadata| metadata.len())
                .unwrap_or(0);
            let mut item: EuvPlaygroundProjectListItem = EuvPlaygroundProjectListItem::default();
            item.set_id(EuvPlaygroundService::encode_id(id))
                .set_name(name)
                .set_updated_at_ms(updated_at_ms)
                .set_code_size(code_size);
            items.push(item);
        }
        items.sort_by(
            |left: &EuvPlaygroundProjectListItem, right: &EuvPlaygroundProjectListItem| {
                right.get_updated_at_ms().cmp(left.get_updated_at_ms())
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
    /// Builds the `ServerHook` state for the `EuvPlaygroundProjectsCreateRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(post),
        request_body_json_result(request_opt: EuvPlaygroundProjectCreateRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundProjectsCreateRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let request: EuvPlaygroundProjectCreateRequest = match request_opt {
            Ok(request) => request,
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, err.to_string());
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let name: String = EuvPlaygroundService::normalize_name(request.get_name());
        let user_root: PathBuf = EuvPlaygroundService::user_dir(user_id);
        match EuvPlaygroundService::project_name_exists(&user_root, &name) {
            Ok(true) => {
                let error: String = format!(
                    "{ERROR_PROJECT_NAME_TAKEN_PREFIX}{name}{ERROR_PROJECT_NAME_TAKEN_SUFFIX}"
                );
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
        let pdir: PathBuf = EuvPlaygroundService::project_dir(user_id, id);
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
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, err);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Project get — GET /api/euv/playground/projects/get/{id}
impl ServerHook for EuvPlaygroundProjectsGetRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `EuvPlaygroundProjectsGetRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundProjectsGetRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_MISSING_PROJECT_ID),
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
            Ok(id) => id,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_INVALID_PROJECT_ID),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from(ERROR_PROJECT_NOT_FOUND),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        let code: String = match EuvPlaygroundService::read_code(&pdir) {
            Ok(code) => code,
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, err);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let (name, updated_at_ms) =
            EuvPlaygroundService::read_metadata(&pdir).unwrap_or_else(|| {
                (
                    CONTROLLER_UNTITLED_PROJECT_NAME.to_string(),
                    EuvPlaygroundService::now_ms(),
                )
            });
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
    /// Builds the `ServerHook` state for the `EuvPlaygroundProjectsSaveRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
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
    /// Handles the `EuvPlaygroundProjectsSaveRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_MISSING_PROJECT_ID),
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
            Ok(id) => id,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_INVALID_PROJECT_ID),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let request: EuvPlaygroundProjectSaveRequest = match request_opt {
            Ok(request) => request,
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, err.to_string());
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from(ERROR_PROJECT_NOT_FOUND),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        // Resolve the final name + code from existing state + overrides.
        let (cur_name, _cur_ts): (String, i64) = EuvPlaygroundService::read_metadata(&pdir)
            .unwrap_or_else(|| {
                (
                    CONTROLLER_UNTITLED_PROJECT_NAME.to_string(),
                    EuvPlaygroundService::now_ms(),
                )
            });
        let name_trim: String = request.get_name().trim().to_string();
        let new_name: String = if name_trim.is_empty() {
            cur_name
        } else {
            EuvPlaygroundService::normalize_name(&name_trim)
        };
        let user_root: PathBuf = EuvPlaygroundService::user_dir(user_id);
        match EuvPlaygroundService::project_name_exists_excluding(
            &user_root,
            &new_name,
            Some(&pdir),
        ) {
            Ok(true) => {
                let error: String = format!(
                    "{ERROR_PROJECT_NAME_TAKEN_PREFIX}{new_name}{ERROR_PROJECT_NAME_TAKEN_SUFFIX}"
                );
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
            Some(code) if !code.is_empty() => Some(code.to_string()),
            _ => None,
        };
        let final_code: String = match code_opt {
            Some(code) => {
                if code.len() > EUV_PLAYGROUND_MAX_CODE_BYTES {
                    let resp: ApiResponse<String> = ApiResponse::new(
                        ApiResponseStatus::InvalidRequest,
                        format!(
                            "{ERROR_CODE_EXCEEDS_PREFIX}{} bytes (got {})",
                            EUV_PLAYGROUND_MAX_CODE_BYTES,
                            code.len()
                        ),
                    );
                    ctx.get_mut_response().set_body(resp.to_json_bytes());
                    return Status::Continue;
                }
                code
            }
            None => match EuvPlaygroundService::read_code(&pdir) {
                Ok(code) => code,
                Err(err) => {
                    let resp: ApiResponse<String> =
                        ApiResponse::new(ApiResponseStatus::BusinessLogicError, err);
                    ctx.get_mut_response().set_body(resp.to_json_bytes());
                    return Status::Continue;
                }
            },
        };
        match EuvPlaygroundService::write_project(&pdir, &new_name, &final_code) {
            Ok(ts) => {
                let mut payload: EuvPlaygroundProjectMutationResponse =
                    EuvPlaygroundProjectMutationResponse::default();
                payload
                    .set_id(EuvPlaygroundService::encode_id(id))
                    .set_name(new_name)
                    .set_updated_at_ms(ts)
                    .set_deleted(false);
                let resp: ApiResponse<EuvPlaygroundProjectMutationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::BusinessLogicError, err);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Project delete — DELETE /api/euv/playground/projects/delete/{id}
impl ServerHook for EuvPlaygroundProjectsDeleteRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `EuvPlaygroundProjectsDeleteRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(delete),
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundProjectsDeleteRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_MISSING_PROJECT_ID),
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
            Ok(id) => id,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_INVALID_PROJECT_ID),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let pdir: PathBuf = EuvPlaygroundService::project_dir(user_id, id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from(ERROR_PROJECT_NOT_FOUND),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        let (name, _ts): (String, i64) =
            EuvPlaygroundService::read_metadata(&pdir).unwrap_or_else(|| {
                (
                    CONTROLLER_UNTITLED_PROJECT_NAME.to_string(),
                    EuvPlaygroundService::now_ms(),
                )
            });
        match remove_dir_all(&pdir) {
            Ok(_) => {
                let mut payload: EuvPlaygroundProjectMutationResponse =
                    EuvPlaygroundProjectMutationResponse::default();
                payload
                    .set_id(EuvPlaygroundService::encode_id(id))
                    .set_name(name)
                    .set_updated_at_ms(EuvPlaygroundService::now_ms())
                    .set_deleted(true);
                let resp: ApiResponse<EuvPlaygroundProjectMutationResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(io_err) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::BusinessLogicError,
                    format!("{ERROR_DELETE_PROJECT_FAILED} {io_err}"),
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
    /// Builds the `ServerHook` state for the `EuvPlaygroundDefaultCodeRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundDefaultCodeRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
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
    /// Builds the `ServerHook` state for the `EuvPlaygroundRunRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(post),
        request_body_json_result(request_opt: EuvPlaygroundRunRequest),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundRunRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let request: EuvPlaygroundRunRequest = match request_opt {
            Ok(request) => request,
            Err(err) => {
                let resp: ApiResponse<String> =
                    ApiResponse::new(ApiResponseStatus::InvalidRequest, err.to_string());
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
        let pdir: PathBuf = EuvPlaygroundService::project_dir(user_id, project_id);
        if !pdir.exists() {
            let resp: ApiResponse<String> = ApiResponse::new(
                ApiResponseStatus::ResourceNotFound,
                String::from(ERROR_PROJECT_NOT_FOUND),
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
                    "{ERROR_CODE_EXCEEDS_PREFIX}{} bytes (got {})",
                    EUV_PLAYGROUND_MAX_CODE_BYTES,
                    code.len()
                ),
            );
            ctx.get_mut_response().set_body(resp.to_json_bytes());
            return Status::Continue;
        }
        let job_id: BuildJobId =
            EuvPlaygroundService::register_pending_job(user_id, project_id).await;
        match EuvPlaygroundService::publish_build_task(job_id, user_id, project_id, &code).await {
            Ok(()) => {
                let mut payload: EuvPlaygroundRunResponse = EuvPlaygroundRunResponse::default();
                payload
                    .set_ok(true)
                    .set_job_id(EuvPlaygroundService::encode_job_id(job_id))
                    .set_status(build_status::PENDING.to_string())
                    .set_message(String::new())
                    .set_html(String::new())
                    .set_js(String::new())
                    .set_wasm(String::new())
                    .set_stderr(String::new())
                    .set_build_url(String::new());
                let resp: ApiResponse<EuvPlaygroundRunResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
            Err(error) => {
                EuvPlaygroundService::mark_job_failed(job_id, &error).await;
                let mut payload: EuvPlaygroundRunResponse = EuvPlaygroundRunResponse::default();
                payload
                    .set_ok(false)
                    .set_job_id(EuvPlaygroundService::encode_job_id(job_id))
                    .set_status(build_status::FAILED.to_string())
                    .set_message(error.clone())
                    .set_html(String::new())
                    .set_js(String::new())
                    .set_wasm(String::new())
                    .set_stderr(error)
                    .set_build_url(String::new());
                let resp: ApiResponse<EuvPlaygroundRunResponse> =
                    ApiResponse::new(ApiResponseStatus::Success, payload);
                ctx.get_mut_response().set_body(resp.to_json_bytes());
            }
        }
        Status::Continue
    }
}

/// Build status — GET /api/euv/playground/run/status/{id}
///
/// Returns the current state of a previously-enqueued build job. The
/// frontend polls this endpoint until `status` becomes `success` or
/// `failed`; the controller answers 404 when the job id is unknown or
/// belongs to a different user.
impl ServerHook for EuvPlaygroundBuildStatusRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `EuvPlaygroundBuildStatusRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get),
        try_get_route_param(ID_KEY => id_opt),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `EuvPlaygroundBuildStatusRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(user_id) = EuvPlaygroundHelpers::require_user(ctx) else {
            return Status::Continue;
        };
        let id_str: String = match id_opt {
            Some(id_str) => id_str,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_MISSING_JOB_ID),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let job_id: BuildJobId = match EuvPlaygroundService::decode_job_id(&id_str) {
            Ok(parsed_job_id) => parsed_job_id,
            Err(_) => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::InvalidRequest,
                    String::from(ERROR_INVALID_JOB_ID),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let job: BuildJob = match EuvPlaygroundService::get_build_status(job_id, user_id).await {
            Some(job) => job,
            None => {
                let resp: ApiResponse<String> = ApiResponse::new(
                    ApiResponseStatus::ResourceNotFound,
                    String::from(ERROR_JOB_NOT_FOUND),
                );
                ctx.get_mut_response().set_body(resp.to_json_bytes());
                return Status::Continue;
            }
        };
        let mut payload: EuvPlaygroundBuildStatusResponse =
            EuvPlaygroundBuildStatusResponse::default();
        payload
            .set_job_id(EuvPlaygroundService::encode_job_id(*job.get_job_id()))
            .set_project_id(EuvPlaygroundService::encode_id(*job.get_project_id()))
            .set_status(job.get_status().clone())
            .set_build_url(job.get_build_url().clone())
            .set_stderr(job.get_stderr().clone())
            .set_created_at_ms(*job.get_created_at_ms())
            .set_updated_at_ms(*job.get_updated_at_ms());
        let resp: ApiResponse<EuvPlaygroundBuildStatusResponse> =
            ApiResponse::new(ApiResponseStatus::Success, payload);
        ctx.get_mut_response().set_body(resp.to_json_bytes());
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
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Option<i32>` - The current user id, or `None` when the cookie is missing or invalid.
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
