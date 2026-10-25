use super::*;

/// Builds a multipart body for one upload request.
///
/// The body carries the metadata of the upload session as text fields and
/// the uploaded file as a file part, so the server reads the file name, the
/// media type and the raw content of the upload from the body itself.
///
/// # Arguments
///
/// - `&str` - The identifier of the upload session.
/// - `&str` - The number of chunks of the upload session.
///
/// # Returns
///
/// - `Result<FormData, ApiError>` - The multipart body, or the failure that stopped it.
pub fn try_register_form(file_id: &str, total_chunks: &str) -> Result<FormData, ApiError> {
    let form: FormData = FormData::new().map_err(|error: JsValue| form_error(error))?;
    try_append_text(&form, FIELD_FILE_ID, file_id)?;
    try_append_text(&form, FIELD_TOTAL_CHUNKS, total_chunks)?;
    Ok(form)
}

/// Builds a multipart body for one chunk of an upload session.
///
/// # Arguments
///
/// - `&str` - The identifier of the upload session.
/// - `&str` - The index of the chunk the body carries.
/// - `&Blob` - The bytes of the chunk.
/// - `&str` - The name of the uploaded file.
///
/// # Returns
///
/// - `Result<FormData, ApiError>` - The multipart body, or the failure that stopped it.
pub fn try_save_form(
    file_id: &str,
    chunk_index: &str,
    chunk: &Blob,
    file_name: &str,
) -> Result<FormData, ApiError> {
    let form: FormData = FormData::new().map_err(|error: JsValue| form_error(error))?;
    try_append_text(&form, FIELD_FILE_ID, file_id)?;
    try_append_text(&form, FIELD_CHUNK_INDEX, chunk_index)?;
    try_append_file(&form, chunk, file_name)?;
    Ok(form)
}

/// Builds a multipart body for the merge of an upload session.
///
/// # Arguments
///
/// - `&str` - The identifier of the upload session.
///
/// # Returns
///
/// - `Result<FormData, ApiError>` - The multipart body, or the failure that stopped it.
pub fn try_merge_form(file_id: &str) -> Result<FormData, ApiError> {
    let form: FormData = FormData::new().map_err(|error: JsValue| form_error(error))?;
    try_append_text(&form, FIELD_FILE_ID, file_id)?;
    Ok(form)
}

/// Adds one text field to a multipart body.
///
/// # Arguments
///
/// - `&FormData` - The multipart body.
/// - `&str` - The name of the field.
/// - `&str` - The value of the field.
///
/// # Returns
///
/// - `Result<(), ApiError>` - `Ok` when the field was added, or the failure that stopped it.
fn try_append_text(form: &FormData, name: &str, value: &str) -> Result<(), ApiError> {
    form.append_with_str(name, value)
        .map_err(|error: JsValue| form_error(error))
}

/// Adds the uploaded file to a multipart body.
///
/// The name of the file travels with the part, so the server reads it from
/// the body of the request instead of from a header of its own.
///
/// # Arguments
///
/// - `&FormData` - The multipart body.
/// - `&Blob` - The bytes of the file.
/// - `&str` - The name of the uploaded file.
///
/// # Returns
///
/// - `Result<(), ApiError>` - `Ok` when the file was added, or the failure that stopped it.
pub fn try_append_file(form: &FormData, file: &Blob, file_name: &str) -> Result<(), ApiError> {
    form.append_with_blob_and_filename(PART_FILE, file, file_name)
        .map_err(|error: JsValue| form_error(error))
}

/// Reads the bytes of a slice of a file.
///
/// The slice keeps the media type of the file, so the part the browser sends
/// carries the type of the uploaded file.
///
/// # Arguments
///
/// - `&File` - The file the slice is taken from.
/// - `i32` - The first byte of the slice.
/// - `i32` - The byte after the last byte of the slice.
///
/// # Returns
///
/// - `Result<Blob, ApiError>` - The bytes of the slice, or the failure that stopped it.
pub fn try_file_slice(file: &File, start: i32, end: i32) -> Result<Blob, ApiError> {
    let content_type: String = file.type_();
    file.slice_with_i32_and_i32_and_content_type(start, end, &content_type)
        .map_err(|error: JsValue| ApiError {
            code: 0,
            message: format!("{MESSAGE_FILE_UNREADABLE}{MESSAGE_SEPARATOR}{:?}", error),
        })
}

/// Builds an empty blob that carries the name and the type of a file.
///
/// The blob names the file without carrying its bytes, so the server learns
/// the name of the file from the body of the registration request.
///
/// # Arguments
///
/// - `&File` - The file the blob names.
///
/// # Returns
///
/// - `Result<Blob, ApiError>` - The blob, or the failure that stopped it.
pub fn try_file_descriptor(file: &File) -> Result<Blob, ApiError> {
    try_file_slice(file, 0, 0)
}

/// Builds the failure reported when the multipart body could not be built.
///
/// # Arguments
///
/// - `JsValue` - The value the browser reported.
///
/// # Returns
///
/// - `ApiError` - The failure describing the problem.
fn form_error(error: JsValue) -> ApiError {
    ApiError {
        code: 0,
        message: format!("{MESSAGE_FORM_FAILED}{MESSAGE_SEPARATOR}{:?}", error),
    }
}
