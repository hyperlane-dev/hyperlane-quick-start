use super::*;

/// Creates the reactive state the upload page renders from.
///
/// The state holds the files that are uploaded, the upload history of the
/// browser and the message of the last call. The history is read from the
/// browser storage as soon as the page is mounted.
///
/// # Returns
///
/// - `UploadState` - The state the page renders from.
pub fn use_upload_state() -> UploadState {
    let entries: Signal<Vec<UploadEntry>> = App::use_signal(Vec::new);
    let history: Signal<Vec<UploadHistoryItem>> = App::use_signal(Vec::new);
    let message: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let failed: Signal<bool> = App::use_signal(|| false);
    let uploading: Signal<bool> = App::use_signal(|| false);
    let state: UploadState = UploadState {
        entries,
        history,
        message,
        failed,
        uploading,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        load_history(state);
        true
    });
    state
}

/// Reads the upload history out of the browser storage.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The history has been written into the state.
pub fn load_history(state: UploadState) {
    let Some(storage): Option<Storage> = storage() else {
        return;
    };
    let Some(raw): Option<String> = storage.get_item(STORAGE_KEY).ok().flatten() else {
        return;
    };
    let Ok(items): Result<Vec<UploadHistoryItem>, serde_json::Error> =
        serde_json::from_str(&raw)
    else {
        return;
    };
    state.get_history().set(items);
}

/// Writes the upload history into the browser storage.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The history has been stored.
pub fn store_history(state: UploadState) {
    let Some(storage): Option<Storage> = storage() else {
        return;
    };
    let Ok(encoded): Result<String, serde_json::Error> =
        serde_json::to_string(&state.get_history().get())
    else {
        return;
    };
    let _stored: Result<(), JsValue> = storage.set_item(STORAGE_KEY, &encoded);
}

/// Uploads the files the visitor picked.
///
/// Every file gets an entry of its own in the upload list, and the entries
/// are uploaded one after the other so the progress of the page stays
/// readable.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `Vec<File>` - The files the visitor picked.
///
/// # Returns
///
/// - `()` - The files have been uploaded.
pub async fn upload_files(state: UploadState, files: Vec<File>) {
    if files.is_empty() {
        return;
    }
    state.get_uploading().set(true);
    for file in files {
        let id: String = new_upload_id();
        let entry: UploadEntry = UploadEntry {
            id: id.clone(),
            name: file.name(),
            size: file.size(),
            progress: 0.0,
            status: UploadStatus::Pending,
            url: TEXT_EMPTY.to_string(),
            error: TEXT_EMPTY.to_string(),
        };
        push_entry(state, entry);
        upload_file(state, id, file).await;
    }
    state.get_uploading().set(false);
}

/// Uploads one file of the upload list.
///
/// The file is registered first, then its chunks are stored one by one, and
/// the merged file is fetched at the end. The address of the uploaded file
/// is written into the entry and into the upload history.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `String` - The identifier of the upload session.
/// - `File` - The file to upload.
///
/// # Returns
///
/// - `()` - The file has been uploaded.
pub async fn upload_file(state: UploadState, id: String, file: File) {
    set_entry_status(state, &id, UploadStatus::Uploading, 0.0);
    let total_chunks: i32 = (file.size() / CHUNK_SIZE as f64).ceil() as i32;
    let total_chunks: i32 = total_chunks.max(1);
    let encoded_name: String = encode_uri_component(&file.name())
        .as_string()
        .unwrap_or_default();
    let total_chunks_text: String = total_chunks.to_string();
    let register_headers: Vec<(&str, &str)> = vec![
        (HEADER_FILE_ID, id.as_str()),
        (HEADER_TOTAL_CHUNKS, total_chunks_text.as_str()),
        (HEADER_FILE_NAME, encoded_name.as_str()),
    ];
    if let Err(error) =
        upload_request(METHOD_POST, PATH_REGISTER, &register_headers, None).await
    {
        fail_entry(state, &id, error.message);
        return;
    }
    for index in 0..total_chunks {
        let start: i32 = index * CHUNK_SIZE;
        let end: i32 = start + CHUNK_SIZE;
        let chunk: Blob = match file.slice_with_i32_and_i32(start, end) {
            Ok(chunk) => chunk,
            Err(_) => {
                fail_entry(state, &id, "The file could not be read".to_string());
                return;
            }
        };
        let chunk_index_text: String = index.to_string();
        let chunk_headers: Vec<(&str, &str)> = vec![
            (HEADER_FILE_ID, id.as_str()),
            (HEADER_CHUNK_INDEX, chunk_index_text.as_str()),
        ];
        if let Err(error) =
            upload_request(METHOD_POST, PATH_SAVE, &chunk_headers, Some(chunk)).await
        {
            fail_entry(state, &id, error.message);
            return;
        }
        let progress: f64 = (index + 1) as f64 / total_chunks as f64 * PERCENT_FULL;
        set_entry_status(state, &id, UploadStatus::Uploading, progress);
    }
    let merge_headers: Vec<(&str, &str)> = vec![(HEADER_FILE_ID, id.as_str())];
    match upload_request(METHOD_POST, PATH_MERGE, &merge_headers, None).await {
        Ok(response) => {
            let name: String = file.name();
            set_entry_url(state, &id, response.url.clone());
            add_history(state, id.clone(), name, file.size(), response.url.clone());
            state
                .get_message()
                .set("The file has been uploaded".to_string());
            state.get_failed().set(false);
        }
        Err(error) => fail_entry(state, &id, error.message),
    }
}

/// Sends one request to one of the upload endpoints.
///
/// The upload endpoints answer with a flat object instead of the envelope of
/// the other endpoints, so the body is read as text and parsed here.
///
/// # Arguments
///
/// - `&str` - The HTTP method the request is issued with.
/// - `&str` - The path the request is sent to.
/// - `&[(&str, &str)]` - The headers the request carries.
/// - `Option<Blob>` - The body the request carries.
///
/// # Returns
///
/// - `Result<UploadResponse, ApiError>` - The answer of the endpoint, or the failure.
pub async fn upload_request(
    method: &str,
    path: &str,
    headers: &[(&str, &str)],
    body: Option<Blob>,
) -> Result<UploadResponse, ApiError> {
    let Some(window): Option<Window> = window() else {
        return Err(ApiError {
            code: 0,
            message: MESSAGE_NO_WINDOW.to_string(),
        });
    };
    let header_map: Headers =
        Headers::new().map_err(|error: JsValue| transport_failure(error))?;
    for (name, value) in headers {
        header_map
            .set(name, value)
            .map_err(|error: JsValue| transport_failure(error))?;
    }
    let init: RequestInit = RequestInit::new();
    init.set_method(method);
    init.set_mode(RequestMode::SameOrigin);
    init.set_credentials(RequestCredentials::Include);
    init.set_headers(&JsValue::from(header_map));
    if let Some(body) = body {
        init.set_body(&JsValue::from(body));
    }
    let promise: Promise = window.fetch_with_str_and_init(path, &init);
    let value: JsValue = JsFuture::from(promise)
        .await
        .map_err(|error: JsValue| transport_failure(error))?;
    let response: Response = value
        .dyn_into()
        .map_err(|error: JsValue| transport_failure(error))?;
    let text_promise: Promise = response
        .text()
        .map_err(|error: JsValue| transport_failure(error))?;
    let body: JsValue = JsFuture::from(text_promise)
        .await
        .map_err(|error: JsValue| transport_failure(error))?;
    let raw: String = body.as_string().unwrap_or_default();
    let parsed: UploadResponse =
        serde_json::from_str(&raw).map_err(|error: serde_json::Error| ApiError {
            code: 0,
            message: format!("{MESSAGE_INVALID_PAYLOAD}{MESSAGE_SEPARATOR}{error}"),
        })?;
    if parsed.code != STATUS_SUCCESS {
        return Err(ApiError {
            code: parsed.code,
            message: parsed.msg,
        });
    }
    Ok(parsed)
}

/// Writes the upload history of the page into a file the browser downloads.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The file has been downloaded.
pub fn export_history(state: UploadState) {
    let Ok(encoded): Result<String, serde_json::Error> =
        serde_json::to_string(&state.get_history().get())
    else {
        return;
    };
    download_file(EXPORT_FILE_NAME, &encoded, CONTENT_TYPE_JSON);
}

/// Merges the upload history of an imported file into the history of the page.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `&str` - The content of the imported file.
///
/// # Returns
///
/// - `()` - The history has been merged.
pub fn import_history(state: UploadState, content: &str) {
    let Ok(items): Result<Vec<UploadHistoryItem>, serde_json::Error> =
        serde_json::from_str(content)
    else {
        state
            .get_message()
            .set("The imported file is not an upload history".to_string());
        state.get_failed().set(true);
        return;
    };
    let mut merged: Vec<UploadHistoryItem> = state.get_history().get();
    for item in items {
        if !merged.iter().any(|known: &UploadHistoryItem| known.id == item.id) {
            merged.push(item);
        }
    }
    state.get_history().set(merged);
    store_history(state);
    state
        .get_message()
        .set("The upload history has been imported".to_string());
    state.get_failed().set(false);
}

/// Adds one record to the upload history of the page.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `String` - The identifier of the upload session.
/// - `String` - The name of the uploaded file.
/// - `f64` - The size of the uploaded file.
/// - `String` - The address the uploaded file is served under.
///
/// # Returns
///
/// - `()` - The record has been added.
fn add_history(
    state: UploadState,
    id: String,
    name: String,
    size: f64,
    url: String,
) {
    let mut history: Vec<UploadHistoryItem> = state.get_history().get();
    let item: UploadHistoryItem = UploadHistoryItem {
        id,
        name,
        size,
        progress: PERCENT_FULL,
        url,
        upload_time: current_time(),
    };
    history.retain(|known: &UploadHistoryItem| known.id != item.id);
    history.insert(0, item);
    state.get_history().set(history);
    store_history(state);
}

/// Adds one file to the upload list of the page.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `UploadEntry` - The entry of the file.
///
/// # Returns
///
/// - `()` - The entry has been added.
fn push_entry(state: UploadState, entry: UploadEntry) {
    let mut entries: Vec<UploadEntry> = state.get_entries().get();
    entries.push(entry);
    state.get_entries().set(entries);
}

/// Writes the state and the progress of one file of the upload list.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `&str` - The identifier of the upload session.
/// - `UploadStatus` - The state of the upload.
/// - `f64` - The share of the file that has been uploaded.
///
/// # Returns
///
/// - `()` - The entry has been written.
fn set_entry_status(state: UploadState, id: &str, status: UploadStatus, progress: f64) {
    let mut entries: Vec<UploadEntry> = state.get_entries().get();
    for entry in entries.iter_mut() {
        if entry.id == id {
            entry.status = status;
            entry.progress = progress;
        }
    }
    state.get_entries().set(entries);
}

/// Writes the address of one uploaded file into the upload list.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `&str` - The identifier of the upload session.
/// - `String` - The address the uploaded file is served under.
///
/// # Returns
///
/// - `()` - The entry has been written.
fn set_entry_url(state: UploadState, id: &str, url: String) {
    let mut entries: Vec<UploadEntry> = state.get_entries().get();
    for entry in entries.iter_mut() {
        if entry.id == id {
            entry.status = UploadStatus::Done;
            entry.progress = PERCENT_FULL;
            entry.url = url.clone();
        }
    }
    state.get_entries().set(entries);
}

/// Marks one file of the upload list as failed.
///
/// # Arguments
///
/// - `UploadState` - The state the page renders from.
/// - `&str` - The identifier of the upload session.
/// - `String` - The message of the failure.
///
/// # Returns
///
/// - `()` - The entry has been written.
fn fail_entry(state: UploadState, id: &str, message: String) {
    let mut entries: Vec<UploadEntry> = state.get_entries().get();
    for entry in entries.iter_mut() {
        if entry.id == id {
            entry.status = UploadStatus::Failed;
            entry.error = message.clone();
        }
    }
    state.get_entries().set(entries);
    state.get_message().set(message);
    state.get_failed().set(true);
}

/// Reads the browser storage of the page.
///
/// # Returns
///
/// - `Option<Storage>` - The storage, absent when the browser denies it.
fn storage() -> Option<Storage> {
    window()
        .and_then(|window: Window| window.local_storage().ok())
        .flatten()
}

/// Builds the identifier of one upload session.
///
/// # Returns
///
/// - `String` - The identifier of the upload session.
fn new_upload_id() -> String {
    let stamp: i64 = Date::now() as i64;
    let noise: f64 = Math::random();
    format!("{stamp:x}{:x}", (noise * 1_000_000.0) as u64)
}

/// Reads the current moment as the text of the upload history.
///
/// # Returns
///
/// - `String` - The current moment.
fn current_time() -> String {
    Date::new_0().to_iso_string().as_string().unwrap_or_default()
}

/// Builds the failure reported when the browser could not complete a request.
///
/// # Arguments
///
/// - `JsValue` - The value the browser reported.
///
/// # Returns
///
/// - `ApiError` - The failure describing the transport level problem.
fn transport_failure(error: JsValue) -> ApiError {
    let cause: String = error.as_string().unwrap_or_else(|| format!("{error:?}"));
    ApiError {
        code: 0,
        message: format!("{MESSAGE_REQUEST_FAILED}{MESSAGE_SEPARATOR}{cause}"),
    }
}

/// Writes a text into a file the browser downloads.
///
/// # Arguments
///
/// - `&str` - The name of the file.
/// - `&str` - The content of the file.
/// - `&str` - The content type of the file.
///
/// # Returns
///
/// - `bool` - Whether the file has been downloaded.
fn download_file(name: &str, content: &str, content_type: &str) -> bool {
    let parts: JsValue = JsValue::from_str(content);
    let Ok(blob): Result<Blob, JsValue> = Blob::new_with_str_sequence(&parts) else {
        return false;
    };
    let _content_type: &str = content_type;
    let Some(href): Option<String> = object_url(&blob) else {
        return false;
    };
    let Some(window): Option<Window> = window() else {
        return false;
    };
    let Some(document): Option<Document> = window.document() else {
        return false;
    };
    let Ok(element): Result<Element, JsValue> = document.create_element("a") else {
        return false;
    };
    let _href: Result<(), JsValue> = element.set_attribute("href", &href);
    let _download: Result<(), JsValue> = element.set_attribute("download", name);
    let Ok(html): Result<HtmlElement, Element> = element.dyn_into() else {
        return false;
    };
    html.click();
    true
}

/// Builds the address of the object behind a blob.
///
/// # Arguments
///
/// - `&Blob` - The blob the address is built for.
///
/// # Returns
///
/// - `Option<String>` - The address of the blob.
fn object_url(blob: &Blob) -> Option<String> {
    let global: JsValue = js_sys::global().into();
    let constructor: JsValue = Reflect::get(&global, &JsValue::from_str("URL")).ok()?;
    let factory: JsValue =
        Reflect::get(&constructor, &JsValue::from_str("createObjectURL")).ok()?;
    let function: js_sys::Function = factory.dyn_into().ok()?;
    let address: JsValue = function.call1(&constructor, &blob.clone().into()).ok()?;
    address.as_string()
}
