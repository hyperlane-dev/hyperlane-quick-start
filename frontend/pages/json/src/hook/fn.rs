use super::*;

/// Creates the reactive state the JSON editor page renders from.
///
/// The state holds the text of the document, the parsed document, the nodes
/// of the tree that are visible and the fields of the editor of a node.
///
/// # Returns
///
/// - `JsonState` - The state the page renders from.
pub fn use_json_state() -> JsonState {
    let input: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let root: Signal<Option<Value>> = App::use_signal(|| None);
    let nodes: Signal<Vec<JsonNode>> = App::use_signal(Vec::new);
    let expanded: Signal<Vec<Vec<String>>> = App::use_signal(Vec::new);
    let loading: Signal<bool> = App::use_signal(|| false);
    let error: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let edit_path: Signal<Vec<String>> = App::use_signal(Vec::new);
    let edit_key: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let edit_type: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let edit_value: Signal<String> = App::use_signal(|| EMPTY.to_string());
    JsonState {
        input,
        root,
        nodes,
        expanded,
        loading,
        error,
        notice,
        edit_path,
        edit_key,
        edit_type,
        edit_value,
    }
}

/// Parses the document of the text field and builds the tree.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The document has been written into the state.
pub fn parse_document(state: JsonState) {
    let text: String = state.get_input().get();
    if text.trim().is_empty() {
        state.get_root().set(None);
        state.get_nodes().set(Vec::new());
        state.get_error().set(ERROR_EMPTY_DOCUMENT.to_string());
        return;
    }
    match serde_json::from_str::<Value>(&text) {
        Ok(value) => {
            state.get_root().set(Some(value));
            state.get_error().set(EMPTY.to_string());
            state.get_notice().set(EMPTY.to_string());
            rebuild(state);
        }
        Err(error) => {
            state.get_error().set(format!("{ERROR_PARSE}: {error}"));
        }
    }
}

/// Opens or closes one node of the tree.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
/// - `Vec<String>` - The path of the node.
///
/// # Returns
///
/// - `()` - The tree has been written into the state.
pub fn toggle_node(state: JsonState, path: Vec<String>) {
    let mut expanded: Vec<Vec<String>> = state.get_expanded().get();
    if expanded.contains(&path) {
        expanded.retain(|open: &Vec<String>| open != &path);
    } else {
        expanded.push(path);
    }
    state.get_expanded().set(expanded);
    rebuild(state);
}

/// Opens the editor of one node of the tree.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
/// - `Vec<String>` - The path of the node.
///
/// # Returns
///
/// - `()` - The editor has been written into the state.
pub fn open_editor(state: JsonState, path: Vec<String>) {
    let Some(value) = value_of(&state.get_root().get(), &path) else {
        return;
    };
    state.get_edit_key().set(path.last().cloned().unwrap_or_default());
    state.get_edit_type().set(kind_of(&value));
    state.get_edit_value().set(edit_text(&value));
    state.get_edit_path().set(path);
}

/// Saves the value of the node that is edited.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The document has been written into the state, or the failure has been reported.
pub fn save_edit(state: JsonState) {
    let path: Vec<String> = state.get_edit_path().get();
    if path.is_empty() {
        return;
    }
    let kind: String = state.get_edit_type().get();
    let raw: String = state.get_edit_value().get();
    let value: Value = match parse_value(&kind, &raw) {
        Ok(value) => value,
        Err(message) => {
            state.get_error().set(message);
            return;
        }
    };
    let Some(mut root) = state.get_root().get() else {
        return;
    };
    if !assign(&mut root, &path, value) {
        state.get_error().set(ERROR_VALUE.to_string());
        return;
    }
    state.get_root().set(Some(root.clone()));
    state.get_input().set(format_document(&root));
    state.get_edit_path().set(Vec::new());
    state.get_error().set(EMPTY.to_string());
    rebuild(state);
}

/// Copies the formatted document to the clipboard.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The document has been copied, or the failure has been reported.
pub async fn copy_document(state: JsonState) {
    let Some(root) = state.get_root().get() else {
        return;
    };
    let text: String = format_document(&root);
    let Some(window): Option<Window> = window() else {
        state.get_error().set(ERROR_COPY.to_string());
        return;
    };
    let promise: Promise = window.navigator().clipboard().write_text(&text);
    if JsFuture::from(promise).await.is_err() {
        state.get_error().set(ERROR_COPY.to_string());
        return;
    }
    state.get_notice().set(NOTICE_COPIED.to_string());
}

/// Builds the nodes of the tree that are visible.
///
/// # Arguments
///
/// - `JsonState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The nodes have been written into the state.
fn rebuild(state: JsonState) {
    let Some(root) = state.get_root().get() else {
        state.get_nodes().set(Vec::new());
        return;
    };
    let expanded: Vec<Vec<String>> = state.get_expanded().get();
    let mut nodes: Vec<JsonNode> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    collect(&root, &mut path, &mut nodes, &expanded, 0);
    state.get_nodes().set(nodes);
}

/// Collects the nodes of the tree that are visible.
///
/// # Arguments
///
/// - `&Value` - The value of the node.
/// - `&mut Vec<String>` - The path of the node.
/// - `&mut Vec<JsonNode>` - The nodes that were collected so far.
/// - `&[Vec<String>]` - The paths of the nodes that are open.
/// - `usize` - The depth of the node.
///
/// # Returns
///
/// - `()` - The nodes have been collected.
fn collect(
    value: &Value,
    path: &mut Vec<String>,
    nodes: &mut Vec<JsonNode>,
    expanded: &[Vec<String>],
    depth: usize,
) {
    let kind: String = kind_of(value);
    let inner: Option<Value> = inner_document(value);
    let json_string: bool = inner.is_some();
    let container: bool = kind == KIND_OBJECT || kind == KIND_ARRAY || json_string;
    let open: bool = expanded.contains(path);
    let key: String = path.last().cloned().unwrap_or_default();
    nodes.push(JsonNode {
        path: path.clone(),
        key,
        depth,
        kind: if json_string {
            KIND_JSON.to_string()
        } else {
            kind
        },
        json_string,
        container,
        expanded: open,
        preview: preview_of(value),
    });
    if !container || !open {
        return;
    }
    let children: Vec<(String, Value)> = match &inner {
        Some(inner) => {
            let entries: Vec<(String, Value)> = entries_of(inner);
            if entries.is_empty() {
                vec![(KEY_VALUE.to_string(), inner.clone())]
            } else {
                entries
            }
        }
        None => entries_of(value),
    };
    for (child_key, child) in children {
        path.push(child_key);
        collect(&child, path, nodes, expanded, depth + 1);
        path.pop();
    }
}

/// Returns the entries of a value.
///
/// The entries of a list hold the position of the item as their key.
///
/// # Arguments
///
/// - `&Value` - The value the entries are read from.
///
/// # Returns
///
/// - `Vec<(String, Value)>` - The entries of the value.
fn entries_of(value: &Value) -> Vec<(String, Value)> {
    match value {
        Value::Object(map) => map
            .iter()
            .map(|(key, child): (&String, &Value)| (key.clone(), child.clone()))
            .collect(),
        Value::Array(items) => items
            .iter()
            .enumerate()
            .map(|(index, child): (usize, &Value)| (index.to_string(), child.clone()))
            .collect(),
        _ => Vec::new(),
    }
}

/// Returns the document a text holds.
///
/// # Arguments
///
/// - `&Value` - The value of the node.
///
/// # Returns
///
/// - `Option<Value>` - The document of the text, absent when the value holds none.
fn inner_document(value: &Value) -> Option<Value> {
    let Value::String(text) = value else {
        return None;
    };
    inner_text(text)
}

/// Returns the document a text holds.
///
/// # Arguments
///
/// - `&str` - The text the document is read from.
///
/// # Returns
///
/// - `Option<Value>` - The document of the text, absent when the text holds none.
fn inner_text(text: &str) -> Option<Value> {
    let trimmed: &str = text.trim();
    if !(trimmed.starts_with('{') && trimmed.ends_with('}'))
        && !(trimmed.starts_with('[') && trimmed.ends_with(']'))
    {
        return None;
    }
    serde_json::from_str::<Value>(trimmed).ok()
}

/// Returns the kind of a value.
///
/// # Arguments
///
/// - `&Value` - The value the kind is read from.
///
/// # Returns
///
/// - `String` - The kind of the value.
fn kind_of(value: &Value) -> String {
    match value {
        Value::Null => KIND_NULL.to_string(),
        Value::Bool(_) => KIND_BOOLEAN.to_string(),
        Value::Number(_) => KIND_NUMBER.to_string(),
        Value::String(_) => KIND_STRING.to_string(),
        Value::Array(_) => KIND_ARRAY.to_string(),
        Value::Object(_) => KIND_OBJECT.to_string(),
    }
}

/// Returns a short preview of a value.
///
/// # Arguments
///
/// - `&Value` - The value the preview is read from.
///
/// # Returns
///
/// - `String` - The preview of the value.
fn preview_of(value: &Value) -> String {
    match value {
        Value::String(text) => cut(text),
        Value::Null => KIND_NULL.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
        Value::Array(_) | Value::Object(_) => {
            let text: String = serde_json::to_string(value).unwrap_or_default();
            cut(&text)
        }
    }
}

/// Cuts a text to the length of a preview.
///
/// # Arguments
///
/// - `&str` - The text to cut.
///
/// # Returns
///
/// - `String` - The cut text.
fn cut(text: &str) -> String {
    if text.chars().count() <= PREVIEW_LENGTH {
        return text.to_string();
    }
    let head: String = text.chars().take(PREVIEW_LENGTH).collect();
    format!("{head}{PREVIEW_SUFFIX}")
}

/// Returns the text the editor of a node holds.
///
/// # Arguments
///
/// - `&Value` - The value of the node.
///
/// # Returns
///
/// - `String` - The text of the editor.
fn edit_text(value: &Value) -> String {
    match value {
        Value::Object(_) | Value::Array(_) => {
            serde_json::to_string_pretty(value).unwrap_or_default()
        }
        Value::String(text) => text.clone(),
        Value::Null => KIND_NULL.to_string(),
        Value::Bool(flag) => flag.to_string(),
        Value::Number(number) => number.to_string(),
    }
}

/// Parses the text of the editor into a value.
///
/// # Arguments
///
/// - `&str` - The kind of the value.
/// - `&str` - The text of the editor.
///
/// # Returns
///
/// - `Result<Value, String>` - The value, or the failure that stopped it.
fn parse_value(kind: &str, raw: &str) -> Result<Value, String> {
    match kind {
        KIND_OBJECT | KIND_ARRAY => serde_json::from_str::<Value>(raw)
            .map_err(|error: serde_json::Error| format!("{ERROR_VALUE}: {error}")),
        KIND_NUMBER => raw
            .trim()
            .parse::<f64>()
            .map(|number: f64| Value::from(number))
            .map_err(|_| format!("{ERROR_VALUE}: {raw}")),
        KIND_BOOLEAN => match raw.trim().to_lowercase().as_str() {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            _ => Err(format!("{ERROR_VALUE}: {raw}")),
        },
        KIND_NULL => Ok(Value::Null),
        _ => Ok(Value::String(raw.to_string())),
    }
}

/// Returns the value a path holds.
///
/// # Arguments
///
/// - `&Option<Value>` - The document.
/// - `&[String]` - The path of the value.
///
/// # Returns
///
/// - `Option<Value>` - The value of the path, absent when the path holds none.
fn value_of(root: &Option<Value>, path: &[String]) -> Option<Value> {
    let mut current: Value = root.clone()?;
    for segment in path {
        current = match &current {
            Value::String(text) => inner_text(text)?,
            Value::Object(map) => map.get(segment)?.clone(),
            Value::Array(items) => {
                let index: usize = segment.parse().ok()?;
                items.get(index)?.clone()
            }
            _ => return None,
        };
    }
    Some(current)
}

/// Writes a value into the document.
///
/// A text that holds a document is opened, the value is written into the
/// document it holds and the text is written back, so a value that lives
/// inside a document that lives inside a text is written the same way.
///
/// # Arguments
///
/// - `&mut Value` - The document.
/// - `&[String]` - The path of the value.
/// - `Value` - The value to write.
///
/// # Returns
///
/// - `bool` - Whether the value was written.
fn assign(current: &mut Value, path: &[String], value: Value) -> bool {
    let Some((segment, rest)) = path.split_first() else {
        *current = value;
        return true;
    };
    match current {
        Value::Object(map) => match map.get_mut(segment) {
            Some(child) => assign(child, rest, value),
            None => false,
        },
        Value::Array(items) => match segment.parse::<usize>().ok() {
            Some(index) => match items.get_mut(index) {
                Some(child) => assign(child, rest, value),
                None => false,
            },
            None => false,
        },
        Value::String(text) => {
            let Some(mut inner) = inner_text(text) else {
                return false;
            };
            if !assign(&mut inner, rest, value) {
                return false;
            }
            *text = serde_json::to_string(&inner).unwrap_or_default();
            true
        }
        _ => false,
    }
}

/// Formats the document the way the editor shows it.
///
/// # Arguments
///
/// - `&Value` - The document.
///
/// # Returns
///
/// - `String` - The formatted document.
fn format_document(root: &Value) -> String {
    serde_json::to_string_pretty(root).unwrap_or_default()
}
