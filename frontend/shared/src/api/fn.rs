use super::*;

/// Sends a request and returns the raw response body.
///
/// The request always carries the JSON content type and the session cookie,
/// so the endpoints behind an authenticated page answer the same way they do
/// for the hand written pages this crate replaces.
///
/// # Arguments
///
/// - `HttpMethod` - The method the request is issued with.
/// - `&str` - The path the request is sent to.
/// - `Option<String>` - The JSON body, when the request carries one.
///
/// # Returns
///
/// - `Result<String, ApiError>` - The response body, or the failure that stopped it.
pub async fn request_text(
    method: HttpMethod,
    path: &str,
    body: Option<String>,
) -> Result<String, ApiError> {
    let Some(window): Option<Window> = window() else {
        return Err(ApiError {
            code: 0,
            message: MESSAGE_NO_WINDOW.to_string(),
        });
    };
    let headers: Headers =
        Headers::new().map_err(|error: JsValue| transport_error(MESSAGE_REQUEST_FAILED, error))?;
    headers
        .set(HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
        .map_err(|error: JsValue| transport_error(MESSAGE_REQUEST_FAILED, error))?;
    let init: RequestInit = RequestInit::new();
    init.set_method(method.as_str());
    init.set_mode(RequestMode::SameOrigin);
    init.set_credentials(RequestCredentials::Include);
    init.set_headers(&JsValue::from(headers));
    if let Some(body) = body {
        init.set_body(&JsValue::from_str(&body));
    }
    let promise: Promise = window.fetch_with_str_and_init(path, &init);
    let response: Response = JsFuture::from(promise)
        .await
        .map_err(|error: JsValue| transport_error(MESSAGE_REQUEST_FAILED, error))?
        .dyn_into()
        .map_err(|error: JsValue| transport_error(MESSAGE_REQUEST_FAILED, error))?;
    let text_promise: Promise = response
        .text()
        .map_err(|error: JsValue| transport_error(MESSAGE_RESPONSE_FAILED, error))?;
    let value: JsValue = JsFuture::from(text_promise)
        .await
        .map_err(|error: JsValue| transport_error(MESSAGE_RESPONSE_FAILED, error))?;
    Ok(value.as_string().unwrap_or_default())
}

/// Sends a request and returns the envelope the server answered with.
///
/// # Arguments
///
/// - `HttpMethod` - The method the request is issued with.
/// - `&str` - The path the request is sent to.
/// - `Option<String>` - The JSON body, when the request carries one.
///
/// # Returns
///
/// - `Result<ApiResponse<T>, ApiError>` - The envelope, or the failure that stopped the call.
pub async fn request_envelope<T: DeserializeOwned>(
    method: HttpMethod,
    path: &str,
    body: Option<String>,
) -> Result<ApiResponse<T>, ApiError> {
    let text: String = request_text(method, path, body).await?;
    serde_json::from_str::<ApiResponse<T>>(&text).map_err(|error: serde_json::Error| ApiError {
        code: 0,
        message: format!("{MESSAGE_INVALID_PAYLOAD}{MESSAGE_SEPARATOR}{error}"),
    })
}

/// Sends a request and returns the payload of a successful envelope.
///
/// A call whose envelope reports a business failure is turned into an
/// [`ApiError`] carrying the status code and the server message, so a page
/// only has to handle the happy path and the failure.
///
/// # Arguments
///
/// - `HttpMethod` - The method the request is issued with.
/// - `&str` - The path the request is sent to.
/// - `Option<String>` - The JSON body, when the request carries one.
///
/// # Returns
///
/// - `Result<T, ApiError>` - The payload, or the failure that stopped the call.
pub async fn request_json<T: DeserializeOwned>(
    method: HttpMethod,
    path: &str,
    body: Option<String>,
) -> Result<T, ApiError> {
    let envelope: ApiResponse<T> = request_envelope(method, path, body).await?;
    if envelope.code != STATUS_SUCCESS {
        return Err(ApiError {
            code: envelope.code,
            message: envelope.message,
        });
    }
    envelope.data.ok_or(ApiError {
        code: envelope.code,
        message: envelope.message,
    })
}

/// Sends a read request and returns the payload of a successful envelope.
///
/// # Arguments
///
/// - `&str` - The path the request is sent to.
///
/// # Returns
///
/// - `Result<T, ApiError>` - The payload, or the failure that stopped the call.
pub async fn get_json<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    request_json(HttpMethod::Get, path, None).await
}

/// Sends a create request and returns the payload of a successful envelope.
///
/// # Arguments
///
/// - `&str` - The path the request is sent to.
/// - `&B` - The body the request carries.
///
/// # Returns
///
/// - `Result<T, ApiError>` - The payload, or the failure that stopped the call.
pub async fn post_json<T: DeserializeOwned, B: Serialize + ?Sized>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let encoded: String = serde_json::to_string(body).map_err(|error: serde_json::Error| ApiError {
        code: 0,
        message: format!("{MESSAGE_INVALID_PAYLOAD}{MESSAGE_SEPARATOR}{error}"),
    })?;
    request_json(HttpMethod::Post, path, Some(encoded)).await
}

/// Sends an update request and returns the payload of a successful envelope.
///
/// # Arguments
///
/// - `&str` - The path the request is sent to.
/// - `&B` - The body the request carries.
///
/// # Returns
///
/// - `Result<T, ApiError>` - The payload, or the failure that stopped the call.
pub async fn put_json<T: DeserializeOwned, B: Serialize + ?Sized>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let encoded: String = serde_json::to_string(body).map_err(|error: serde_json::Error| ApiError {
        code: 0,
        message: format!("{MESSAGE_INVALID_PAYLOAD}{MESSAGE_SEPARATOR}{error}"),
    })?;
    request_json(HttpMethod::Put, path, Some(encoded)).await
}

/// Sends a delete request and returns the payload of a successful envelope.
///
/// # Arguments
///
/// - `&str` - The path the request is sent to.
///
/// # Returns
///
/// - `Result<T, ApiError>` - The payload, or the failure that stopped the call.
pub async fn delete_json<T: DeserializeOwned>(path: &str) -> Result<T, ApiError> {
    request_json(HttpMethod::Delete, path, None).await
}

/// Builds the failure reported when the browser could not complete a request.
///
/// # Arguments
///
/// - `&str` - The description of the failed step.
/// - `JsValue` - The value the browser reported.
///
/// # Returns
///
/// - `ApiError` - The failure describing the transport level problem.
fn transport_error(prefix: &str, error: JsValue) -> ApiError {
    let cause: String = error
        .as_string()
        .unwrap_or_else(|| format!("{error:?}"));
    ApiError {
        code: 0,
        message: format!("{prefix}{MESSAGE_SEPARATOR}{cause}"),
    }
}
