/// SSE initial poll delay ms.
pub(crate) const SSE_INITIAL_POLL_DELAY_MS: u64 = 500;

/// SSE connection timeout secs.
pub(crate) const SSE_CONNECTION_TIMEOUT_SECS: u64 = 3600;

/// SSE idle sleep ms.
pub(crate) const SSE_IDLE_SLEEP_MS: u64 = 10;

/// SSE event log.
pub(crate) const SSE_EVENT_LOG: &str = "log";

/// SSE event complete.
pub(crate) const SSE_EVENT_COMPLETE: &str = "complete";

/// SSE reason no active streams.
pub(crate) const SSE_REASON_NO_ACTIVE_STREAMS: &str = "no_active_streams";

/// SSE reason timeout.
pub(crate) const SSE_REASON_TIMEOUT: &str = "timeout";

/// SSE reason run completed.
pub(crate) const SSE_REASON_RUN_COMPLETED: &str = "run_completed";

/// Error message when missing or invalid id.
pub(crate) const ERROR_MISSING_OR_INVALID_ID: &str = "Missing or invalid id parameter";

/// Error message when missing or invalid run id.
pub(crate) const ERROR_MISSING_OR_INVALID_RUN_ID: &str = "Missing or invalid run_id parameter";

/// Error message when pipeline not found.
pub(crate) const ERROR_PIPELINE_NOT_FOUND: &str = "Pipeline not found";

/// Error message when run not found.
pub(crate) const ERROR_RUN_NOT_FOUND: &str = "Run not found";

/// Status code for success job updated.
pub(crate) const SUCCESS_JOB_STATUS_UPDATED: &str = "Job status updated successfully";

/// Status code for success step updated.
pub(crate) const SUCCESS_STEP_STATUS_UPDATED: &str = "Step status updated successfully";

/// JSON escape sequence for a double quote character.
pub(crate) const JSON_ESCAPED_DOUBLE_QUOTE: &str = "\\\"";

/// JSON escape sequence for a backslash character.
pub(crate) const JSON_ESCAPED_BACKSLASH: &str = "\\\\";

/// JSON escape sequence for a line feed character.
pub(crate) const JSON_ESCAPED_LINE_FEED: &str = "\\n";

/// JSON escape sequence for a carriage return character.
pub(crate) const JSON_ESCAPED_CARRIAGE_RETURN: &str = "\\r";

/// JSON escape sequence for a horizontal tab character.
pub(crate) const JSON_ESCAPED_TAB: &str = "\\t";

/// OpenAPI response description for bad request.
pub(crate) const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad request";

/// OpenAPI response description for cicd job status updated.
pub(crate) const OPENAPI_DESCRIPTION_CICD_JOB_STATUS_UPDATED: &str = "Job status updated";

/// OpenAPI response description for cicd management page.
pub(crate) const OPENAPI_DESCRIPTION_CICD_MANAGEMENT_PAGE: &str = "CICD management page";

/// OpenAPI response description for cicd pipeline created.
pub(crate) const OPENAPI_DESCRIPTION_CICD_PIPELINE_CREATED: &str = "Pipeline created successfully";

/// OpenAPI response description for cicd pipeline details.
pub(crate) const OPENAPI_DESCRIPTION_CICD_PIPELINE_DETAILS: &str = "Pipeline details";

/// OpenAPI response description for cicd pipeline list retrieved.
pub(crate) const OPENAPI_DESCRIPTION_CICD_PIPELINE_LIST_RETRIEVED: &str = "List of pipelines";

/// OpenAPI response description for cicd run details.
pub(crate) const OPENAPI_DESCRIPTION_CICD_RUN_DETAILS: &str = "Run details";

/// OpenAPI response description for cicd run details with jobs.
pub(crate) const OPENAPI_DESCRIPTION_CICD_RUN_DETAILS_WITH_JOBS: &str =
    "Run details with jobs and steps";

/// OpenAPI response description for cicd run list retrieved.
pub(crate) const OPENAPI_DESCRIPTION_CICD_RUN_LIST_RETRIEVED: &str = "Paginated list of runs";

/// OpenAPI response description for cicd run triggered.
pub(crate) const OPENAPI_DESCRIPTION_CICD_RUN_TRIGGERED: &str = "Run triggered successfully";

/// OpenAPI response description for cicd step status updated.
pub(crate) const OPENAPI_DESCRIPTION_CICD_STEP_STATUS_UPDATED: &str = "Step status updated";

/// OpenAPI response description for internal server error.
pub(crate) const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal server error";

/// OpenAPI description of the pipeline id parameter.
pub(crate) const OPENAPI_PARAM_DESCRIPTION_PIPELINE_ID: &str = "Pipeline ID";

/// OpenAPI route path for the cicd endpoint.
pub(crate) const OPENAPI_PATH_CICD: &str = "/cicd";

/// OpenAPI route path for the cicd job update endpoint.
pub(crate) const OPENAPI_PATH_CICD_JOB_UPDATE: &str = "/api/cicd/job/update";

/// OpenAPI route path for the cicd pipeline create endpoint.
pub(crate) const OPENAPI_PATH_CICD_PIPELINE_CREATE: &str = "/api/cicd/pipeline/create";

/// OpenAPI route path for the cicd pipeline get endpoint.
pub(crate) const OPENAPI_PATH_CICD_PIPELINE_GET: &str = "/api/cicd/pipeline/get";

/// OpenAPI route path for the cicd pipeline list endpoint.
pub(crate) const OPENAPI_PATH_CICD_PIPELINE_LIST: &str = "/api/cicd/pipeline/list";

/// OpenAPI route path for the cicd run detail endpoint.
pub(crate) const OPENAPI_PATH_CICD_RUN_DETAIL: &str = "/api/cicd/run/detail";

/// OpenAPI route path for the cicd run get endpoint.
pub(crate) const OPENAPI_PATH_CICD_RUN_GET: &str = "/api/cicd/run/get";

/// OpenAPI route path for the cicd run list endpoint.
pub(crate) const OPENAPI_PATH_CICD_RUN_LIST: &str = "/api/cicd/run/list";

/// OpenAPI route path for the cicd run trigger endpoint.
pub(crate) const OPENAPI_PATH_CICD_RUN_TRIGGER: &str = "/api/cicd/run/trigger";

/// OpenAPI route path for the cicd step update endpoint.
pub(crate) const OPENAPI_PATH_CICD_STEP_UPDATE: &str = "/api/cicd/step/update";

/// Query parameter key for last id.
pub(crate) const QUERY_KEY_LAST_ID: &str = "last_id";

/// Query parameter key for offsets.
pub(crate) const QUERY_KEY_OFFSETS: &str = "offsets";

/// Query parameter key for page size.
pub(crate) const QUERY_KEY_PAGE_SIZE: &str = "page_size";

/// Query parameter key for pipeline id.
pub(crate) const QUERY_KEY_PIPELINE_ID: &str = "pipeline_id";

/// Query parameter key for run id.
pub(crate) const QUERY_KEY_RUN_ID: &str = "run_id";

/// The name of the package of the page.
pub(crate) const CICD_VIEW_PAGE_NAME: &str = "cicd";
