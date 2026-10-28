/// Error message when only admin can create records.
pub(crate) const ERROR_ONLY_ADMIN_CAN_CREATE_RECORDS: &str = "Only admin can create records";

/// Error message when only admin can create for others.
pub(crate) const ERROR_ONLY_ADMIN_CAN_CREATE_FOR_OTHERS: &str =
    "Only admin can create records for other users";

/// Error message when invalid record id.
pub(crate) const ERROR_INVALID_RECORD_ID: &str = "Invalid record ID";

/// Error message when record id required.
pub(crate) const ERROR_RECORD_ID_REQUIRED: &str = "Record ID is required";

/// Error message when only admin overview.
pub(crate) const ERROR_ONLY_ADMIN_OVERVIEW: &str = "Only admin can access overview statistics";

/// Error message when invalid image id.
pub(crate) const ERROR_INVALID_IMAGE_ID: &str = "Invalid image ID";

/// Error message when image id required.
pub(crate) const ERROR_IMAGE_ID_REQUIRED: &str = "Image ID is required";

/// Error message when image not found.
pub(crate) const ERROR_IMAGE_NOT_FOUND: &str = "Image not found";

/// Error message when missing x file name.
pub(crate) const ERROR_MISSING_X_FILE_NAME: &str = "Missing X-File-Name header";

/// Error message when missing x mime type.
pub(crate) const ERROR_MISSING_X_MIME_TYPE: &str = "Missing X-Mime-Type header";

/// Chrono format string for a plain calendar date.
pub(crate) const DATE_FORMAT_YYYY_MM_DD: &str = "%Y-%m-%d";

/// OpenAPI response description for bad request.
pub(crate) const OPENAPI_DESCRIPTION_BAD_REQUEST: &str = "Bad request";

/// OpenAPI response description for forbidden.
pub(crate) const OPENAPI_DESCRIPTION_FORBIDDEN: &str = "Forbidden";

/// OpenAPI response description for image downloaded.
pub(crate) const OPENAPI_DESCRIPTION_IMAGE_DOWNLOADED: &str = "Image downloaded successfully";

/// OpenAPI response description for image list retrieved.
pub(crate) const OPENAPI_DESCRIPTION_IMAGE_LIST_RETRIEVED: &str =
    "Image list retrieved successfully";

/// OpenAPI response description for image uploaded.
pub(crate) const OPENAPI_DESCRIPTION_IMAGE_UPLOADED: &str = "Image uploaded successfully";

/// OpenAPI response description for internal server error.
pub(crate) const OPENAPI_DESCRIPTION_INTERNAL_SERVER_ERROR: &str = "Internal server error";

/// OpenAPI response description for overview statistics retrieved.
pub(crate) const OPENAPI_DESCRIPTION_OVERVIEW_STATISTICS_RETRIEVED: &str =
    "Statistics retrieved successfully";

/// OpenAPI response description for record created.
pub(crate) const OPENAPI_DESCRIPTION_RECORD_CREATED: &str = "Record created successfully";

/// OpenAPI response description for record details retrieved.
pub(crate) const OPENAPI_DESCRIPTION_RECORD_DETAILS_RETRIEVED: &str =
    "Record details retrieved successfully";

/// OpenAPI response description for record list retrieved.
pub(crate) const OPENAPI_DESCRIPTION_RECORD_LIST_RETRIEVED: &str =
    "List of records retrieved successfully";

/// OpenAPI response description for record not found.
pub(crate) const OPENAPI_DESCRIPTION_RECORD_NOT_FOUND: &str = "Record not found";

/// OpenAPI response description for unauthorized.
pub(crate) const OPENAPI_DESCRIPTION_UNAUTHORIZED: &str = "Unauthorized";

/// OpenAPI description of the image id parameter.
pub(crate) const OPENAPI_PARAM_DESCRIPTION_IMAGE_ID: &str = "Image ID";

/// OpenAPI description of the record id parameter.
pub(crate) const OPENAPI_PARAM_DESCRIPTION_RECORD_ID: &str = "Record ID";

/// OpenAPI route path for the order image download endpoint.
pub(crate) const OPENAPI_PATH_ORDER_IMAGE_DOWNLOAD: &str = "/api/order/image/download/{id}";

/// OpenAPI route path for the order image list endpoint.
pub(crate) const OPENAPI_PATH_ORDER_IMAGE_LIST: &str = "/api/order/image/list/{record_id}";

/// OpenAPI route path for the order image upload endpoint.
pub(crate) const OPENAPI_PATH_ORDER_IMAGE_UPLOAD: &str = "/api/order/image/upload";

/// OpenAPI route path for the order overview statistics endpoint.
pub(crate) const OPENAPI_PATH_ORDER_OVERVIEW_STATISTICS: &str = "/api/order/overview/statistics";

/// OpenAPI route path for the order record create endpoint.
pub(crate) const OPENAPI_PATH_ORDER_RECORD_CREATE: &str = "/api/order/record/create";

/// OpenAPI route path for the order record get endpoint.
pub(crate) const OPENAPI_PATH_ORDER_RECORD_GET: &str = "/api/order/record/get/{id}";

/// OpenAPI route path for the order record list endpoint.
pub(crate) const OPENAPI_PATH_ORDER_RECORD_LIST: &str = "/api/order/record/list";
