use super::*;

/// Serves the page of a package at the path of the route.
///
/// Every page of the application is loaded at its own path, so the address of
/// the browser names the page and not the file that carries it. The build
/// writes the page and its artifacts into the static resource directory, and
/// the browser resolves the artifacts against the address of the document. The
/// base element points the browser at the directory of the artifacts, so the
/// page keeps working at its own path.
///
/// # Arguments
///
/// - `&mut Context` - The request context.
///
/// - `&str` - The name of the package of the page.
///
/// # Returns
///
/// - `Status` - The handle result.
pub fn serve_page(ctx: &mut Context, name: &str) -> Status {
    let path: PathBuf = PathBuf::from(STATIC_RESOURCES_DIR).join(name).join(PAGE_INDEX_NAME);
    let Ok(content) = fs::read(&path) else {
        ctx.get_mut_response().set_status_code(404);
        return Status::Continue;
    };
    let document: String = String::from_utf8_lossy(&content).to_string();
    let base: String = format!(
        "{BASE_ELEMENT}{STATIC_URL_PREFIX}/{name}/{BASE_ELEMENT_END}"
    );
    let document: String = document.replacen(HEAD_ELEMENT, &format!("{HEAD_ELEMENT}{base}"), 1);
    ctx.get_mut_response()
        .set_body(document.as_bytes())
        .set_status_code(200)
        .set_header(CONTENT_TYPE, CONTENT_TYPE_HTML);
    Status::Continue
}
