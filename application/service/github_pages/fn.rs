use super::*;

/// Extracts relative resource paths from HTML content.
///
/// Parses the HTML for `<script src>`, `<link href>`, `<img src>` attributes
/// and ES module `import ... from '...'` statements, returning only relative
/// paths (excluding `http://`, `https://`, `//`, and `data:` URIs).
/// Duplicates are removed and the result is sorted for deterministic output.
///
/// # Arguments
/// - `&str`: The HTML content to parse.
///
/// # Returns
/// - `Vec<String>`: A sorted, deduplicated list of relative resource paths.
pub fn extract_resource_paths(html: &str) -> Vec<String> {
    let mut paths: HashSet<String> = HashSet::new();
    extract_tag_src_paths(html, &mut paths);
    extract_es_module_import_paths(html, &mut paths);
    let mut result: Vec<String> = paths.into_iter().collect();
    result.sort();
    result
}

/// Extracts `src` and `href` attribute values from HTML tags (`<script>`, `<link>`, `<img>`).
///
/// Scans the raw HTML bytes for opening tags, then extracts the value of the
/// `src` attribute (for `<script>` and `<img>`) or the `href` attribute
/// (for `<link>`). Only relative paths are kept.
///
/// # Arguments
/// - `&str`: The HTML content to scan.
/// - `&mut HashSet<String>`: The set into which discovered paths are inserted.
fn extract_tag_src_paths(html: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = html.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        if bytes[position] != b'<' {
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
        let is_script: bool = eq_ignore_case(tag_name, b"script");
        let is_link: bool = eq_ignore_case(tag_name, b"link");
        let is_img: bool = eq_ignore_case(tag_name, b"img");
        if !is_script && !is_link && !is_img {
            position = find_tag_end(bytes, tag_start);
            continue;
        }
        let target_attr: &[u8] = if is_link { b"href" } else { b"src" };
        let mut found_attr: bool = false;
        loop {
            position = skip_whitespace(bytes, position);
            if position >= len || bytes[position] == b'>' {
                break;
            }
            if bytes[position] == b'/' && position + 1 < len && bytes[position + 1] == b'>' {
                break;
            }
            let attr_name: &[u8] = match read_attr_name(bytes, &mut position) {
                Some(name) => name,
                None => break,
            };
            position = skip_whitespace(bytes, position);
            if position < len && bytes[position] == b'=' {
                position += 1;
                position = skip_whitespace(bytes, position);
                if eq_ignore_case(attr_name, target_attr)
                    && let Some(value) = read_attr_value(bytes, &mut position)
                {
                    if !is_absolute_url(&value) {
                        paths.insert(value);
                    }
                    found_attr = true;
                } else if !eq_ignore_case(attr_name, target_attr) {
                    let _ = read_attr_value(bytes, &mut position);
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
/// Matches patterns such as `import X from './path'` and dynamic `import('./path')`.
/// Only paths starting with `.` are captured (relative imports).
///
/// # Arguments
/// - `&str`: The HTML content to scan.
/// - `&mut HashSet<String>`: The set into which discovered paths are inserted.
fn extract_es_module_import_paths(html: &str, paths: &mut HashSet<String>) {
    let bytes: &[u8] = html.as_bytes();
    let len: usize = bytes.len();
    let mut position: usize = 0;
    while position < len {
        position = match find_substring(bytes, b"import", position) {
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
        let from_pos: Option<usize> = find_substring(bytes, b"from", position);
        match from_pos {
            Some(fp) => {
                let mut scan: usize = position;
                while scan < fp {
                    if bytes[scan] == b';' || bytes[scan] == b'>' {
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
/// - `&[u8]`: The byte slice to scan.
/// - `usize`: The starting position.
///
/// # Returns
/// - `usize`: The first non-whitespace position at or after the input.
fn skip_whitespace(bytes: &[u8], mut position: usize) -> usize {
    while position < bytes.len() && bytes[position].is_ascii_whitespace() {
        position += 1;
    }
    position
}

/// Reads a tag name starting at `position` (e.g. `script`, `link`, `img`).
///
/// # Arguments
/// - `&[u8]`: The byte slice to scan.
/// - `&mut usize`: The current position, updated to point after the tag name.
///
/// # Returns
/// - `Option<&[u8]>`: The tag name bytes, or `None` if no valid tag name found.
fn read_tag_name<'a>(bytes: &'a [u8], position: &mut usize) -> Option<&'a [u8]> {
    let start: usize = *position;
    while *position < bytes.len() && (is_alpha(bytes[*position]) || bytes[*position] == b'-') {
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
/// - `&[u8]`: The byte slice to scan.
/// - `&mut usize`: The current position, updated to point after the attribute name.
///
/// # Returns
/// - `Option<&[u8]>`: The attribute name bytes, or `None` if no valid name found.
fn read_attr_name<'a>(bytes: &'a [u8], position: &mut usize) -> Option<&'a [u8]> {
    let start: usize = *position;
    while *position < bytes.len()
        && bytes[*position] != b'='
        && bytes[*position] != b'>'
        && bytes[*position] != b' '
        && bytes[*position] != b'\t'
        && bytes[*position] != b'\n'
        && bytes[*position] != b'\r'
        && bytes[*position] != b'/'
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
/// Supports single-quoted, double-quoted, and unquoted values.
///
/// # Arguments
/// - `&[u8]`: The byte slice to scan.
/// - `&mut usize`: The current position, updated to point after the value.
///
/// # Returns
/// - `Option<String>`: The decoded attribute value, or `None` on parse failure.
fn read_attr_value(bytes: &[u8], position: &mut usize) -> Option<String> {
    if *position >= bytes.len() {
        return None;
    }
    let quote: u8 = bytes[*position];
    if quote == b'"' || quote == b'\'' {
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
            && bytes[*position] != b'>'
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
/// - `&[u8]`: The byte slice to scan.
/// - `usize`: The starting position (after the opening `<script>` tag).
///
/// # Returns
/// - `usize`: The position after the closing `</script>` tag.
fn skip_until_close_script(bytes: &[u8], mut position: usize) -> usize {
    let len: usize = bytes.len();
    while position + 8 < len {
        if bytes[position] == b'<'
            && bytes[position + 1] == b'/'
            && (bytes[position + 2] == b's' || bytes[position + 2] == b'S')
            && (bytes[position + 3] == b'c' || bytes[position + 3] == b'C')
            && (bytes[position + 4] == b'r' || bytes[position + 4] == b'R')
            && (bytes[position + 5] == b'i' || bytes[position + 5] == b'I')
            && (bytes[position + 6] == b'p' || bytes[position + 6] == b'P')
            && (bytes[position + 7] == b't' || bytes[position + 7] == b'T')
        {
            position += 8;
            while position < len && bytes[position] != b'>' {
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
/// - `&[u8]`: The byte slice to scan.
/// - `usize`: The starting position (at or before the tag start).
///
/// # Returns
/// - `usize`: The position after the `>` character.
fn find_tag_end(bytes: &[u8], mut position: usize) -> usize {
    let len: usize = bytes.len();
    let mut in_quote: Option<u8> = None;
    while position < len {
        let byte: u8 = bytes[position];
        if let Some(quote) = in_quote {
            if byte == quote {
                in_quote = None;
            }
        } else if byte == b'"' || byte == b'\'' {
            in_quote = Some(byte);
        } else if byte == b'>' {
            return position + 1;
        }
        position += 1;
    }
    len
}

/// Checks whether the given URL string is an absolute URL or data URI.
///
/// Returns `true` for strings starting with `http://`, `https://`, `//`, or `data:`.
///
/// # Arguments
/// - `&str`: The URL string to check.
///
/// # Returns
/// - `bool`: `true` if the URL is absolute or a data URI.
fn is_absolute_url(value: &str) -> bool {
    value.starts_with("http://")
        || value.starts_with("https://")
        || value.starts_with("//")
        || value.starts_with("data:")
}

/// Compares two byte slices case-insensitively.
///
/// # Arguments
/// - `&[u8]`: The first byte slice.
/// - `&[u8]`: The second byte slice.
///
/// # Returns
/// - `bool`: `true` if the slices are equal ignoring ASCII case.
fn eq_ignore_case(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter()
        .zip(b.iter())
        .all(|(x, y)| x.eq_ignore_ascii_case(y))
}

/// Checks whether the given byte is an ASCII alphabetic character.
///
/// # Arguments
/// - `u8`: The byte to check.
///
/// # Returns
/// - `bool`: `true` if the byte is in `A-Z` or `a-z`.
fn is_alpha(byte: u8) -> bool {
    byte.is_ascii_alphabetic()
}

/// Finds the first occurrence of `needle` in `haystack` at or after `start`.
///
/// # Arguments
/// - `&[u8]`: The haystack to search.
/// - `&[u8]`: The needle to find.
/// - `usize`: The starting position.
///
/// # Returns
/// - `Option<usize>`: The position of the first match, or `None`.
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
/// - `&[u8]`: The byte slice to scan.
/// - `usize`: The starting position (at the opening quote).
///
/// # Returns
/// - `Option<String>`: The string content without quotes, or `None` on parse failure.
fn read_string_literal(bytes: &[u8], position: usize) -> Option<String> {
    if position >= bytes.len() {
        return None;
    }
    let quote: u8 = bytes[position];
    if quote != b'"' && quote != b'\'' {
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
