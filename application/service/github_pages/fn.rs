use super::*;

/// Checks whether the given path segment is safe from path traversal attacks.
///
/// A path is considered unsafe if it contains `..`, backslash characters,
/// or starts with a forward slash (absolute path).
///
/// # Arguments
///
/// - `&str` - The path segment to validate.
///
/// # Returns
///
/// - `bool` - `true` if the path is safe, `false` if it contains traversal patterns.
pub fn is_safe_path(value: &str) -> bool {
    !value.contains("..") && !value.contains('\\') && !value.starts_with('/')
}

/// Extracts relative resource paths from HTML, JS, or CSS content.
///
/// Parses the content for HTML tag attributes (`<script src>`, `<link href>`, `<img src>`,
/// `<video src>`, `<audio src>`, `<source src>`, `<track src>`, `<embed src>`, `<object data>`),
/// ES module `import ... from '...'` statements, `new URL('...', import.meta.url)` patterns,
/// and CSS `url()` / `@import` references,
/// returning only relative paths (excluding `http://`, `https://`, `//`, and `data:` URIs).
/// Duplicates are removed and the result is sorted for deterministic output.
///
/// # Arguments
///
/// - `&str` - The content to parse (HTML, JS, or CSS).
///
/// # Returns
///
/// - `Vec<String>` - A sorted, deduplicated list of relative resource paths.
pub fn extract_resource_paths(content: &str) -> Vec<String> {
    let mut paths: HashSet<String> = HashSet::new();
    extract_tag_src_paths(content, &mut paths);
    extract_es_module_import_paths(content, &mut paths);
    extract_new_url_paths(content, &mut paths);
    extract_css_url_paths(content, &mut paths);
    let mut result: Vec<String> = paths
        .into_iter()
        .filter(|path: &String| !path.contains("${"))
        .collect();
    result.sort();
    result
}

/// Extracts relative resource paths from content based on the file extension.
///
/// Selectively calls only the relevant extraction functions for the given file type:
/// - **HTML** (`html`, `htm`, `svg`, `xml`): All extractors (tags, imports, new URL, CSS url).
/// - **JS** (`js`, `mjs`, `cjs`): Only ES module imports and `new URL()` patterns.
/// - **CSS** (`css`, `scss`, `less`, `sass`): Only `url()` and `@import` patterns.
/// - **Other text** (`json`, `map`, `wasm`): No extraction (returns empty).
///
/// # Arguments
///
/// - `&str` - The content to parse.
///
/// # Returns
///
/// - `Vec<String>` - A sorted, deduplicated list of relative resource paths.
pub fn extract_resource_paths_by_extension(content: &str, extension: &str) -> Vec<String> {
    let mut paths: HashSet<String> = HashSet::new();
    match extension {
        RESOURCE_EXTENSION_HTML | "htm" | "svg" | "xml" => {
            extract_tag_src_paths(content, &mut paths);
            extract_es_module_import_paths(content, &mut paths);
            extract_new_url_paths(content, &mut paths);
            extract_css_url_paths(content, &mut paths);
        }
        "js" | "mjs" | "cjs" => {
            extract_es_module_import_paths(content, &mut paths);
            extract_new_url_paths(content, &mut paths);
        }
        _ if RESOURCE_STYLESHEET_EXTENSIONS.contains(&extension) => {
            extract_css_url_paths(content, &mut paths);
        }
        _ => {}
    }
    let mut result: Vec<String> = paths
        .into_iter()
        .filter(|path: &String| !path.contains("${"))
        .collect();
    result.sort();
    result
}

/// Extracts resource paths from CSS `url()` and `@import` statements.
///
/// # Arguments
///
/// - `&str` - The content to scan (CSS or any text containing CSS-like constructs).
/// - `&mut HashSet<String>` - The set into which discovered paths are inserted.
fn extract_css_url_paths(content: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = content.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        let remaining: &[u8] = &bytes[position..];
        let url_idx: Option<usize> = find_substring(remaining, CSS_URL_FUNCTION_OPEN, 0);
        let import_idx: Option<usize> = find_substring(remaining, CSS_AT_IMPORT, 0);
        let next_url: Option<usize> = url_idx.map(|i: usize| position + i);
        let next_import: Option<usize> = import_idx.map(|i: usize| position + i);
        let target: Option<usize> = match (next_url, next_import) {
            (Some(u), Some(i)) => Some(min(u, i)),
            (Some(u), None) => Some(u),
            (None, Some(i)) => Some(i),
            (None, None) => None,
        };
        let target_pos: usize = match target {
            Some(pos) => pos,
            None => break,
        };
        if next_url == Some(target_pos) {
            let url_start: usize = target_pos + 4;
            let after_url: usize = skip_whitespace(bytes, url_start);
            if after_url >= len {
                position = url_start;
                continue;
            }
            if bytes[after_url] == b'\\' {
                position = after_url + 1;
                continue;
            }
            let quote: u8 = bytes[after_url];
            if (quote == BYTE_QUOTE_DOUBLE || quote == BYTE_QUOTE_SINGLE)
                && let Some(value) = read_string_literal(bytes, after_url)
            {
                if !is_absolute_url(&value) {
                    paths.insert(value);
                }
                let mut scan: usize = after_url + 1;
                while scan < len && bytes[scan] != quote {
                    scan += 1;
                }
                scan += 1;
                while scan < len && bytes[scan] != BYTE_PAREN_CLOSE {
                    scan += 1;
                }
                position = if scan < len { scan + 1 } else { len };
            } else {
                let mut end: usize = after_url;
                while end < len
                    && bytes[end] != BYTE_PAREN_CLOSE
                    && !bytes[end].is_ascii_whitespace()
                {
                    end += 1;
                }
                if end > after_url {
                    let value: String = String::from_utf8_lossy(&bytes[after_url..end]).to_string();
                    if !is_absolute_url(&value) {
                        paths.insert(value);
                    }
                }
                position = end + 1;
            }
        } else {
            let import_start: usize = target_pos + 7;
            let after_import: usize = skip_whitespace(bytes, import_start);
            if after_import >= len {
                position = import_start;
                continue;
            }
            if bytes[after_import..].starts_with(CSS_URL_FUNCTION_OPEN) {
                position = after_import;
                continue;
            }
            if bytes[after_import] == b'\\' {
                position = after_import + 1;
                continue;
            }
            let quote: u8 = bytes[after_import];
            if (quote == BYTE_QUOTE_DOUBLE || quote == BYTE_QUOTE_SINGLE)
                && let Some(value) = read_string_literal(bytes, after_import)
            {
                if !is_absolute_url(&value) {
                    paths.insert(value);
                }
                let mut scan: usize = after_import + 1;
                while scan < len && bytes[scan] != quote {
                    scan += 1;
                }
                position = if scan < len { scan + 1 } else { len };
            } else {
                position = import_start + 7;
            }
        }
    }
}

/// Extracts resource paths from `new URL('...', import.meta.url)` patterns in JS modules.
///
/// # Arguments
///
/// - `&str` - The content to scan (typically JS module source).
/// - `&mut HashSet<String>` - The set into which discovered paths are inserted.
fn extract_new_url_paths(content: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = content.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        position = match find_substring(bytes, ES_NEW_URL_OPEN, position) {
            Some(pos) => pos,
            None => break,
        };
        let url_start: usize = position + 8;
        let after_url: usize = skip_whitespace(bytes, url_start);
        if after_url < len && bytes[after_url] == b'\\' {
            position = after_url + 1;
            continue;
        }
        if after_url < len
            && (bytes[after_url] == BYTE_QUOTE_DOUBLE || bytes[after_url] == BYTE_QUOTE_SINGLE)
            && let Some(value) = read_string_literal(bytes, after_url)
            && !is_absolute_url(&value)
        {
            paths.insert(value);
        }
        position = url_start;
    }
}

/// Extracts `src` and `href` attribute values from HTML tags.
///
/// Supported tags and their target attributes:
/// - `<script src>`, `<img src>`, `<link href>` (classic resource references)
/// - `<video src>`, `<audio src>` (media elements)
/// - `<source src>` (media source elements inside `<video>`/`<audio>`)
/// - `<track src>` (text track elements inside `<video>`/`<audio>`)
/// - `<embed src>` (embedded content)
/// - `<object data>` (embedded objects)
///
/// # Arguments
///
/// - `&str` - The HTML content to scan.
/// - `&mut HashSet<String>` - The set into which discovered paths are inserted.
fn extract_tag_src_paths(html: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = html.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        if bytes[position] != BYTE_ANGLE_OPEN {
            position += 1;
            continue;
        }
        position += 1;
        position = skip_whitespace(bytes, position);
        let tag_start: usize = position;
        let tag_name: &[u8] = match read_tag_name(bytes, &mut position) {
            Some(name) => name,
            None => continue,
        };
        let is_script: bool = eq_ignore_case(tag_name, HTML_TAG_SCRIPT);
        let is_link: bool = eq_ignore_case(tag_name, HTML_TAG_LINK);
        let is_img: bool = eq_ignore_case(tag_name, b"img");
        let is_video: bool = eq_ignore_case(tag_name, HTML_TAG_VIDEO);
        let is_audio: bool = eq_ignore_case(tag_name, HTML_TAG_AUDIO);
        let is_source: bool = eq_ignore_case(tag_name, HTML_TAG_SOURCE);
        let is_track: bool = eq_ignore_case(tag_name, HTML_TAG_TRACK);
        let is_embed: bool = eq_ignore_case(tag_name, HTML_TAG_EMBED);
        let is_object: bool = eq_ignore_case(tag_name, HTML_TAG_OBJECT);
        if !is_script
            && !is_link
            && !is_img
            && !is_video
            && !is_audio
            && !is_source
            && !is_track
            && !is_embed
            && !is_object
        {
            position = find_tag_end(bytes, tag_start);
            continue;
        }
        let target_attr: &[u8] = if is_link {
            HTML_ATTR_HREF
        } else if is_object {
            HTML_SCHEME_DATA
        } else {
            HTML_ATTR_SRC
        };
        let mut found_attr: bool = false;
        loop {
            position = skip_whitespace(bytes, position);
            if position >= len || bytes[position] == BYTE_ANGLE_CLOSE {
                break;
            }
            if bytes[position] == BYTE_SLASH
                && position + 1 < len
                && bytes[position + 1] == BYTE_ANGLE_CLOSE
            {
                break;
            }
            let attr_name: &[u8] = match read_attr_name(bytes, &mut position) {
                Some(name) => name,
                None => break,
            };
            position = skip_whitespace(bytes, position);
            if position < len && bytes[position] == BYTE_EQUALS {
                position += 1;
                position = skip_whitespace(bytes, position);
                if position < len && bytes[position] == b'\\' {
                    continue;
                }
                if eq_ignore_case(attr_name, target_attr)
                    && let Some(value) = read_attr_value(bytes, &mut position)
                {
                    if !is_absolute_url(&value) {
                        paths.insert(value);
                    }
                    found_attr = true;
                } else if !eq_ignore_case(attr_name, target_attr) {
                    let _: Option<String> = read_attr_value(bytes, &mut position);
                }
            }
        }
        if found_attr && is_script {
            position = skip_until_close_script(bytes, position);
        }
    }
}

/// Extracts resource paths from ES module `import ... from '...'` statements.
///
/// # Arguments
///
/// - `&str` - The HTML content to scan.
/// - `&mut HashSet<String>` - The set into which discovered paths are inserted.
fn extract_es_module_import_paths(html: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = html.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        position = match find_substring(bytes, ES_IMPORT_KEYWORD, position) {
            Some(pos) => pos,
            None => break,
        };
        position += 6;
        if position < len && is_alpha(bytes[position]) {
            continue;
        }
        if position >= 7 && is_alpha(bytes[position - 7]) {
            continue;
        }
        let from_pos: Option<usize> = find_substring(bytes, ES_FROM_KEYWORD, position);
        match from_pos {
            Some(fp) => {
                let mut scan: usize = position;
                while scan < fp {
                    if bytes[scan] == BYTE_SEMICOLON || bytes[scan] == BYTE_ANGLE_CLOSE {
                        break;
                    }
                    scan += 1;
                }
                if scan == fp
                    && let Some(path) = read_string_literal(bytes, skip_whitespace(bytes, fp + 4))
                    && path.starts_with('.')
                {
                    paths.insert(path);
                    position = skip_whitespace(bytes, fp + 4);
                } else {
                    position = fp + 4;
                }
            }
            None => break,
        }
    }
}

/// Advances `position` past any whitespace characters.
///
/// # Arguments
///
/// - `&[u8]` - The byte slice to scan.
/// - `usize` - The starting position.
///
/// # Returns
///
/// - `usize` - The first non-whitespace position at or after the input.
fn skip_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && bytes[position].is_ascii_whitespace() {
        position += 1;
    }
    position
}

/// Reads a tag name starting at `position` (e.g. `script`, `link`, `img`).
///
/// # Arguments
///
/// - `&'a [u8]` - The raw byte slice.
/// - `&mut usize` - The current position, updated to point after the tag name.
///
/// # Returns
///
/// - `Option<&'a [u8]>` - The tag name, when present.
fn read_tag_name<'a>(bytes: &'a [u8], position: &mut usize) -> Option<&'a [u8]> {
    let start: usize = *position;
    while *position < bytes.len() && (is_alpha(bytes[*position]) || bytes[*position] == BYTE_HYPHEN)
    {
        *position += 1;
    }
    if *position > start {
        Some(&bytes[start..*position])
    } else {
        None
    }
}

/// Reads an attribute name starting at `position`.
///
/// # Arguments
///
/// - `&'a [u8]` - The raw byte slice.
/// - `&mut usize` - The current position, updated to point after the attribute name.
///
/// # Returns
///
/// - `Option<&'a [u8]>` - The attr name, when present.
fn read_attr_name<'a>(bytes: &'a [u8], position: &mut usize) -> Option<&'a [u8]> {
    let start: usize = *position;
    while *position < bytes.len()
        && bytes[*position] != BYTE_EQUALS
        && bytes[*position] != BYTE_ANGLE_CLOSE
        && bytes[*position] != b' '
        && bytes[*position] != b'\t'
        && bytes[*position] != b'\n'
        && bytes[*position] != b'\r'
        && bytes[*position] != BYTE_SLASH
    {
        *position += 1;
    }
    if *position > start {
        Some(&bytes[start..*position])
    } else {
        None
    }
}

/// Reads an attribute value (quoted or unquoted) starting at `position`.
///
/// # Arguments
///
/// - `&[u8]` - The byte slice to scan.
/// - `&mut usize` - The current position, updated to point after the value.
///
/// # Returns
///
/// - `Option<String>` - The decoded attribute value, or `None` on parse failure.
fn read_attr_value(bytes: &[u8], position: &mut usize) -> Option<String> {
    if *position >= bytes.len() {
        return None;
    }
    let quote: u8 = bytes[*position];
    if quote == BYTE_QUOTE_DOUBLE || quote == BYTE_QUOTE_SINGLE {
        *position += 1;
        let start: usize = *position;
        while *position < bytes.len() && bytes[*position] != quote {
            *position += 1;
        }
        let value: String = String::from_utf8_lossy(&bytes[start..*position]).to_string();
        if *position < bytes.len() {
            *position += 1;
        }
        Some(value)
    } else {
        let start: usize = *position;
        while *position < bytes.len()
            && bytes[*position] != b' '
            && bytes[*position] != b'\t'
            && bytes[*position] != b'\n'
            && bytes[*position] != b'\r'
            && bytes[*position] != BYTE_ANGLE_CLOSE
        {
            *position += 1;
        }
        if *position > start {
            Some(String::from_utf8_lossy(&bytes[start..*position]).to_string())
        } else {
            None
        }
    }
}

/// Advances `position` past the closing `</script>` tag.
///
/// # Arguments
///
/// - `&[u8]` - The byte slice to scan.
/// - `usize` - The starting position (after the opening `<script>` tag).
///
/// # Returns
///
/// - `usize` - The position after the closing `</script>` tag.
fn skip_until_close_script(bytes: &[u8], mut position: usize) -> usize {
    let len: usize = bytes.len();
    while position + 8 < len {
        if bytes[position] == BYTE_ANGLE_OPEN
            && bytes[position + 1] == BYTE_SLASH
            && (bytes[position + 2] == BYTE_SCRIPT_S_LOWER
                || bytes[position + 2] == BYTE_SCRIPT_S_UPPER)
            && (bytes[position + 3] == BYTE_SCRIPT_C_LOWER
                || bytes[position + 3] == BYTE_SCRIPT_C_UPPER)
            && (bytes[position + 4] == BYTE_SCRIPT_R_LOWER
                || bytes[position + 4] == BYTE_SCRIPT_R_UPPER)
            && (bytes[position + 5] == BYTE_SCRIPT_I_LOWER
                || bytes[position + 5] == BYTE_SCRIPT_I_UPPER)
            && (bytes[position + 6] == BYTE_SCRIPT_P_LOWER
                || bytes[position + 6] == BYTE_SCRIPT_P_UPPER)
            && (bytes[position + 7] == BYTE_SCRIPT_T_LOWER
                || bytes[position + 7] == BYTE_SCRIPT_T_UPPER)
        {
            position += 8;
            while position < len && bytes[position] != BYTE_ANGLE_CLOSE {
                position += 1;
            }
            if position < len {
                position += 1;
            }
            return position;
        }
        position += 1;
    }
    len
}

/// Finds the end of the current HTML tag (the `>` character).
///
/// # Arguments
///
/// - `&[u8]` - The byte slice to scan.
/// - `usize` - The starting position (at or before the tag start).
///
/// # Returns
///
/// - `usize` - The position after the `>` character.
fn find_tag_end(bytes: &[u8], mut position: usize) -> usize {
    let len: usize = bytes.len();
    let mut in_quote: Option<u8> = None;
    while position < len {
        let byte: u8 = bytes[position];
        if let Some(quote) = in_quote {
            if byte == quote {
                in_quote = None;
            }
        } else if byte == BYTE_QUOTE_DOUBLE || byte == BYTE_QUOTE_SINGLE {
            in_quote = Some(byte);
        } else if byte == BYTE_ANGLE_CLOSE {
            return position + 1;
        }
        position += 1;
    }
    len
}

/// Checks whether the given URL string is an absolute URL or data URI.
///
/// # Arguments
///
/// - `&str` - The URL string to check.
///
/// # Returns
///
/// - `bool` - `true` if the URL is absolute or a data URI.
fn is_absolute_url(value: &str) -> bool {
    value.starts_with(URL_SCHEME_HTTP)
        || value.starts_with(URL_SCHEME_HTTPS)
        || value.starts_with("//")
        || value.starts_with(URL_SCHEME_DATA)
}

/// Compares two byte slices case-insensitively.
///
/// # Arguments
///
/// - `&[u8]` - The first byte slice.
///
/// # Returns
///
/// - `bool` - `true` if the slices are equal ignoring ASCII case.
fn eq_ignore_case(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(x, y): (&u8, &u8)| x.eq_ignore_ascii_case(y))
}

/// Checks whether the given byte is an ASCII alphabetic character.
///
/// # Arguments
///
/// - `u8` - The byte to check.
///
/// # Returns
///
/// - `bool` - `true` if the byte is in `A-Z` or `a-z`.
fn is_alpha(byte: u8) -> bool {
    byte.is_ascii_alphabetic()
}

/// Finds the first occurrence of `needle` in `haystack` at or after `start`.
///
/// # Arguments
///
/// - `&[u8]` - The haystack to search.
/// - `usize` - The starting position.
///
/// # Returns
///
/// - `Option<usize>` - The position of the first match, or `None`.
fn find_substring(haystack: &[u8], needle: &[u8], start: usize) -> Option<usize> {
    if needle.is_empty() || start >= haystack.len() {
        return None;
    }
    let needle_len: usize = needle.len();
    let end: usize = haystack.len().saturating_sub(needle_len - 1);
    let mut position: usize = start;
    while position < end {
        if &haystack[position..position + needle_len] == needle {
            return Some(position);
        }
        position += 1;
    }
    None
}

/// Reads a single- or double-quoted string literal starting at `position`.
///
/// # Arguments
///
/// - `&[u8]` - The byte slice to scan.
/// - `usize` - The starting position (at the opening quote).
///
/// # Returns
///
/// - `Option<String>` - The string content without quotes, or `None` on parse failure.
fn read_string_literal(bytes: &[u8], position: usize) -> Option<String> {
    if position >= bytes.len() {
        return None;
    }
    let quote: u8 = bytes[position];
    if quote != BYTE_QUOTE_DOUBLE && quote != BYTE_QUOTE_SINGLE {
        return None;
    }
    let mut end: usize = position + 1;
    while end < bytes.len() && bytes[end] != quote {
        end += 1;
    }
    if end >= bytes.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&bytes[position + 1..end]).to_string())
}

/// Parses an HTTP Range header value into a (start, end) byte range.
///
/// Supports standard `bytes=start-end`, `bytes=start-`, and `bytes=-suffix` formats.
/// Returns the resolved (start, end) byte offsets where `end` is inclusive.
///
/// # Arguments
///
/// - `&str` - The Range header value (e.g. `"bytes=0-1023"`).
/// - `u64` - The total file size in bytes.
///
/// # Returns
///
/// - `Result<(u64, u64), String>` - The resolved (start, end) byte range, or an error message.
pub fn parse_range_header(range_header: &str, file_size: u64) -> Result<(u64, u64), String> {
    if !range_header.starts_with(RANGE_HEADER_PREFIX) {
        return Err(ERROR_INVALID_RANGE_HEADER_FORMAT.to_string());
    }
    let range_spec: &str = &range_header[RANGE_HEADER_PREFIX.len()..];
    let parts: Vec<&str> = range_spec.split('-').collect();
    if parts.len() != 2 {
        return Err(ERROR_INVALID_RANGE_SPECIFICATION.to_string());
    }
    let start_str: &str = parts[0];
    let end_str: &str = parts[1];
    if start_str.is_empty() && end_str.is_empty() {
        return Err(ERROR_INVALID_RANGE_SPECIFICATION.to_string());
    }
    let start: u64 = if start_str.is_empty() {
        let suffix_length: u64 = end_str
            .parse()
            .map_err(|_| ERROR_INVALID_RANGE_SPECIFICATION.to_string())?;
        file_size.saturating_sub(suffix_length)
    } else {
        start_str
            .parse()
            .map_err(|_| ERROR_INVALID_RANGE_SPECIFICATION.to_string())?
    };
    let end: u64 = if end_str.is_empty() {
        file_size - 1
    } else {
        let parsed_end: u64 = end_str
            .parse()
            .map_err(|_| ERROR_INVALID_RANGE_SPECIFICATION.to_string())?;
        parsed_end.min(file_size - 1)
    };
    if start >= file_size {
        return Err(ERROR_RANGE_START_EXCEEDS_FILE_SIZE.to_string());
    }
    if start > end {
        return Err(ERROR_INVALID_RANGE_START_GREATER_THAN_END.to_string());
    }
    Ok((start, end))
}

/// Reads a specific byte range from a local file.
///
/// Opens the file, seeks to the start offset, and reads `length` bytes.
///
/// # Arguments
///
/// - `&str` - The file path on disk.
/// - `u64` - The starting byte offset.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The read byte buffer, or an error if the file cannot be opened or
///   read.
pub async fn read_file_range(path: &str, start: u64, length: u64) -> Result<Vec<u8>, String> {
    let mut file: File =
        File::open(path).map_err(|error: Error| format!("Failed to open file {error}"))?;
    file.seek(SeekFrom::Start(start))
        .map_err(|error: Error| format!("Failed to seek file {error}"))?;
    let mut buffer: Vec<u8> = vec![0u8; length as usize];
    file.read_exact(&mut buffer)
        .map_err(|error: Error| format!("Failed to read file {error}"))?;
    Ok(buffer)
}

/// Returns a reference to the shared HTTP client, creating it on first call.
///
/// The client is configured with the module-wide timeout and redirect policy
/// and is reused across all fetch calls to amortize connection-pool setup.
///
/// # Returns
///
/// - `&'static Client` - The HTTP client.
pub fn get_http_client() -> &'static Client {
    SHARED_HTTP_CLIENT.get_or_init(|| {
        Client::builder()
            .timeout(Duration::from_secs(FETCH_TIMEOUT_SECS))
            .redirect(Policy::limited(MAX_REDIRECTS))
            .build()
            .expect(ERROR_SHARED_HTTP_CLIENT)
    })
}

/// Awaits the completion of a pending fetch identified by the watch receiver.
///
/// Returns the raw fetched bytes on success, or the error the designated
/// fetcher encountered.
///
/// # Arguments
///
/// - `&mut FetchPendingReceiver` - The receiver.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The for pending fetch result, or an error message.
pub async fn wait_for_pending_fetch(
    receiver: &mut FetchPendingReceiver,
) -> Result<Vec<u8>, String> {
    receiver
        .wait_for(|opt: &FetchPendingValue| opt.is_some())
        .await
        .map_err(|_| ERROR_FETCH_CANCELLED.to_string())?;
    // wait_for guarantees `is_some` when it returns `Ok`
    receiver.borrow_and_update().clone().unwrap()
}
