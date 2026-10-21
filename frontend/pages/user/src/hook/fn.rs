use super::*;

/// Creates the reactive state the user management page renders from.
///
/// The state holds the list of users, the cursor of the pages, the user
/// whose detail is open and the fields of the two forms of the detail.
/// The list is loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `UserState` - The state the page renders from.
pub fn use_user_state() -> UserState {
    let users: Signal<Vec<UserProfile>> = App::use_signal(Vec::new);
    let total: Signal<i64> = App::use_signal(|| 0);
    let has_more: Signal<bool> = App::use_signal(|| false);
    let cursors: Signal<Vec<String>> = App::use_signal(Vec::new);
    let page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let keyword: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let pending: Signal<bool> = App::use_signal(|| false);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let detail: Signal<Option<UserProfile>> = App::use_signal(|| None);
    let detail_mode: Signal<String> = App::use_signal(|| MODE_VIEW.to_string());
    let edit_email: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let edit_phone: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let old_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let new_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let confirm: Signal<Option<PendingAction>> = App::use_signal(|| None);
    let state: UserState = UserState {
        users,
        total,
        has_more,
        cursors,
        page,
        keyword,
        pending,
        loading,
        error,
        notice,
        detail,
        detail_mode,
        edit_email,
        edit_phone,
        old_password,
        new_password,
        confirm,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_users(state));
        true
    });
    let _debounce: IntervalHandle = App::use_interval(SEARCH_DELAY_MS, move || {
        if state.get_pending().get() {
            state.get_pending().set(false);
            spawn_local(load_users(state));
        }
    });
    state
}

/// Loads the page of the users being displayed.
///
/// The list is filtered by the keyword of the search field and walks through
/// the pages with the cursor of the server.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn load_users(state: UserState) {
    let page: i32 = state.get_page().get();
    let keyword: String = state.get_keyword().get();
    let cursor: String = cursor_of(&state, page);
    state.get_loading().set(true);
    let mut path: String = format!("{PATH_LIST}?limit={PAGE_SIZE}");
    if !keyword.is_empty() {
        path.push_str(&format!("&keyword={}", encode_uri_component(&keyword)));
    }
    if !cursor.is_empty() {
        path.push_str(&format!("&last_id={}", encode_uri_component(&cursor)));
    }
    match get_json::<UserList>(&path).await {
        Ok(list) => {
            let cursors: Vec<String> = cursors_of(&state, page, &list);
            state.get_users().set(list.users);
            state.get_total().set(list.total_count);
            state.get_has_more().set(list.has_more);
            state.get_cursors().set(cursors);
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
    state.get_loading().set(false);
}

/// Opens the detail of a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `()` - The detail has been written into the state.
pub async fn open_user(state: UserState, id: String) {
    let path: String = format!("{PATH_GET}/{id}");
    match get_json::<UserProfile>(&path).await {
        Ok(user) => {
            state.get_edit_email().set(user.email.clone().unwrap_or_default());
            state.get_edit_phone().set(user.phone.clone().unwrap_or_default());
            state.get_detail_mode().set(MODE_VIEW.to_string());
            state.get_detail().set(Some(user));
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Saves the contact fields of the user whose detail is open.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The user has been updated, or the failure has been reported.
pub async fn save_contact(state: UserState) {
    let Some(user) = state.get_detail().get() else {
        return;
    };
    let email: String = state.get_edit_email().get().trim().to_string();
    let phone: String = state.get_edit_phone().get().trim().to_string();
    let body: UpdateUserRequest = UpdateUserRequest {
        email: if email.is_empty() { None } else { Some(email) },
        phone: if phone.is_empty() { None } else { Some(phone) },
    };
    let path: String = format!("{PATH_UPDATE}/{}", user.id);
    match post_json::<UserProfile, UpdateUserRequest>(&path, &body).await {
        Ok(updated) => {
            state.get_detail().set(Some(updated));
            state.get_detail_mode().set(MODE_VIEW.to_string());
            state.get_notice().set("User updated successfully".to_string());
            load_users(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Saves the password of the user whose detail is open.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The password has been changed, or the failure has been reported.
pub async fn save_password(state: UserState) {
    let Some(user) = state.get_detail().get() else {
        return;
    };
    let old_password: String = state.get_old_password().get();
    let new_password: String = state.get_new_password().get();
    if old_password.is_empty() || new_password.is_empty() {
        state.get_error().set(ERROR_EMPTY_PASSWORD.to_string());
        return;
    }
    let body: ChangePasswordRequest = ChangePasswordRequest {
        old_password,
        new_password,
    };
    let path: String = format!("{PATH_CHANGE_PASSWORD}/{}", user.id);
    match post_json::<UserProfile, ChangePasswordRequest>(&path, &body).await {
        Ok(_) => {
            state.get_old_password().set(EMPTY.to_string());
            state.get_new_password().set(EMPTY.to_string());
            state.get_detail_mode().set(MODE_VIEW.to_string());
            state.get_notice().set("Password changed successfully".to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Approves or rejects a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
/// - `bool` - Whether the user is approved.
///
/// # Returns
///
/// - `()` - The status has been changed, or the failure has been reported.
pub async fn set_status(state: UserState, id: String, approved: bool) {
    let body: UpdateStatusRequest = UpdateStatusRequest { approved };
    let path: String = format!("{PATH_UPDATE_STATUS}/{id}");
    match post_json::<UserProfile, UpdateStatusRequest>(&path, &body).await {
        Ok(_) => {
            state
                .get_notice()
                .set(if approved { "User approved" } else { "User rejected" }.to_string());
            load_users(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Removes a user.
///
/// # Arguments
///
/// - `UserState` - The state the page renders from.
/// - `String` - The identifier of the user.
///
/// # Returns
///
/// - `()` - The user has been removed, or the failure has been reported.
pub async fn delete_user(state: UserState, id: String) {
    let path: String = format!("{PATH_DELETE}/{id}");
    match request_json::<serde_json::Value>(HttpMethod::Post, &path, None).await {
        Ok(_) => {
            state.get_notice().set("User deleted successfully".to_string());
            load_users(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Returns the cursor the page starts after.
///
/// # Arguments
///
/// - `&UserState` - The state the page renders from.
/// - `i32` - The page that is loaded.
///
/// # Returns
///
/// - `String` - The identifier the page starts after, empty for the first page.
fn cursor_of(state: &UserState, page: i32) -> String {
    if page <= PAGE_FIRST {
        return EMPTY.to_string();
    }
    state
        .get_cursors()
        .get()
        .get((page - 2) as usize)
        .cloned()
        .unwrap_or_default()
}

/// Returns the cursors of the pages after the one that was loaded.
///
/// # Arguments
///
/// - `&UserState` - The state the page renders from.
/// - `i32` - The page that was loaded.
/// - `&UserList` - The list the server answered with.
///
/// # Returns
///
/// - `Vec<String>` - The cursors of the pages.
fn cursors_of(state: &UserState, page: i32, list: &UserList) -> Vec<String> {
    let mut cursors: Vec<String> = state.get_cursors().get();
    cursors.truncate(page as usize);
    while cursors.len() < page as usize {
        cursors.push(EMPTY.to_string());
    }
    if let Some(last) = list.users.last() {
        cursors[page as usize - 1] = last.id.clone();
    }
    cursors
}
