use super::*;

/// Creates the reactive state the redis page renders from.
///
/// The records of the cache are loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `RedisState` - The state the page renders from.
pub fn use_redis_state() -> RedisState {
    let records: Signal<Vec<RedisRecord>> = App::use_signal(Vec::new);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let create_key: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let create_value: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let edit_open: Signal<bool> = App::use_signal(|| false);
    let edit_key: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let edit_value: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: RedisState = RedisState {
        records,
        loading,
        error,
        notice,
        create_key,
        create_value,
        edit_open,
        edit_key,
        edit_value,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_records(state));
        true
    });
    state
}

/// Loads the records of the cache.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The records have been written into the state.
pub async fn load_records(state: RedisState) {
    state.get_loading().set(true);
    match get_json::<Vec<RedisRecord>>(PATH_REDIS_LIST).await {
        Ok(records) => state.get_records().set(records),
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Creates the record the form of the page holds.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The record has been created and the list has been reloaded.
pub async fn create_record(state: RedisState) {
    let key: String = state.get_create_key().get();
    let value: String = state.get_create_value().get();
    if key.trim().is_empty() || value.trim().is_empty() {
        state.get_error().set(MESSAGE_FIELDS_REQUIRED.to_string());
        return;
    }
    let body: RedisDraft = RedisDraft { key, value };
    match post_json::<serde_json::Value, RedisDraft>(PATH_REDIS_CREATE, &body).await {
        Ok(_) => {
            state.get_create_key().set(TEXT_EMPTY.to_string());
            state.get_create_value().set(TEXT_EMPTY.to_string());
            state.get_error().set(TEXT_EMPTY.to_string());
            state.get_notice().set(MESSAGE_CREATED.to_string());
            load_records(state).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Updates the record the editor holds.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The record has been updated and the list has been reloaded.
pub async fn update_record(state: RedisState) {
    let key: String = state.get_edit_key().get();
    let value: String = state.get_edit_value().get();
    if key.trim().is_empty() || value.trim().is_empty() {
        state.get_error().set(MESSAGE_FIELDS_REQUIRED.to_string());
        return;
    }
    let body: RedisDraft = RedisDraft { key, value };
    match post_json::<serde_json::Value, RedisDraft>(PATH_REDIS_UPDATE, &body).await {
        Ok(_) => {
            state.get_edit_open().set(false);
            state.get_error().set(TEXT_EMPTY.to_string());
            state.get_notice().set(MESSAGE_UPDATED.to_string());
            load_records(state).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Removes the record the caller names.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
/// - `String` - The key of the record to remove.
///
/// # Returns
///
/// - `()` - The record has been removed and the list has been reloaded.
pub async fn delete_record(state: RedisState, key: String) {
    let path: String = format!("{PATH_REDIS_DELETE}?{QUERY_KEY}={}", encode_uri_component(&key));
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => {
            state.get_error().set(TEXT_EMPTY.to_string());
            state.get_notice().set(MESSAGE_DELETED.to_string());
            load_records(state).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Opens the editor of one record.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
/// - `String` - The key of the record to edit.
/// - `String` - The value of the record to edit.
///
/// # Returns
///
/// - `()` - The editor is open.
pub fn open_editor(state: RedisState, key: String, value: String) {
    state.get_edit_key().set(key);
    state.get_edit_value().set(value);
    state.get_edit_open().set(true);
}

/// Closes the editor of a record.
///
/// # Arguments
///
/// - `RedisState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The editor is closed.
pub fn close_editor(state: RedisState) {
    state.get_edit_open().set(false);
}
