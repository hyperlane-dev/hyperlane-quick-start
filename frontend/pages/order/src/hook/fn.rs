use super::*;

/// Creates the reactive state the order page renders from.
///
/// The state holds the records, the statistics, the users and the profile of
/// the visitor. The profile and the statistics are loaded as soon as the page
/// is mounted, the records follow when their tab is opened.
///
/// # Returns
///
/// - `OrderState` - The state the page renders from.
pub fn use_order_state() -> OrderState {
    let tab: Signal<String> = App::use_signal(|| TAB_DASHBOARD.to_string());
    let records: Signal<Vec<RecordItem>> = App::use_signal(Vec::new);
    let total_income: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let total_expense: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let balance: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let total_count: Signal<i64> = App::use_signal(|| 0);
    let page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let start_date: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let end_date: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let filter_category: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let filter_type: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let overview: Signal<Option<OverviewStatistics>> = App::use_signal(|| None);
    let current_user: Signal<Option<UserProfile>> = App::use_signal(|| None);
    let users: Signal<Vec<UserProfile>> = App::use_signal(Vec::new);
    let users_total: Signal<i64> = App::use_signal(|| 0);
    let users_has_more: Signal<bool> = App::use_signal(|| false);
    let users_cursors: Signal<Vec<String>> = App::use_signal(Vec::new);
    let users_page: Signal<i32> = App::use_signal(|| PAGE_FIRST);
    let users_keyword: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let users_pending: Signal<bool> = App::use_signal(|| false);
    let profile_email: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let profile_phone: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let old_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let new_password: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let create_open: Signal<bool> = App::use_signal(|| false);
    let record_amount: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let record_category: Signal<String> = App::use_signal(|| CATEGORY_FOOD.to_string());
    let record_type: Signal<String> = App::use_signal(|| TYPE_EXPENSE.to_string());
    let record_description: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let record_bill_date: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let record_target_user: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| EMPTY.to_string());
    let state: OrderState = OrderState {
        tab,
        records,
        total_income,
        total_expense,
        balance,
        total_count,
        page,
        start_date,
        end_date,
        filter_category,
        filter_type,
        overview,
        current_user,
        users,
        users_total,
        users_has_more,
        users_cursors,
        users_page,
        users_keyword,
        users_pending,
        profile_email,
        profile_phone,
        old_password,
        new_password,
        create_open,
        record_amount,
        record_category,
        record_type,
        record_description,
        record_bill_date,
        record_target_user,
        loading,
        error,
        notice,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_current_user(state));
        spawn_local(load_overview(state));
        true
    });
    let _debounce: IntervalHandle = App::use_interval(SEARCH_DELAY_MS, move || {
        if state.get_users_pending().get() {
            state.get_users_pending().set(false);
            spawn_local(load_users(state));
        }
    });
    watch!(tab, |tab_value: String| {
        if tab_value == TAB_RECORDS {
            spawn_local(load_records(state));
            return;
        }
        if tab_value == TAB_USERS {
            spawn_local(load_users(state));
            return;
        }
        if tab_value == TAB_DASHBOARD {
            spawn_local(load_overview(state));
        }
    });
    state
}

/// Loads the user that is signed in.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The user has been written into the state.
pub async fn load_current_user(state: OrderState) {
    match get_json::<UserProfile>(PATH_USER_INFO).await {
        Ok(user) => {
            state.get_profile_email().set(user.email.clone().unwrap_or_default());
            state.get_profile_phone().set(user.phone.clone().unwrap_or_default());
            state.get_current_user().set(Some(user));
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Loads the statistics of the ledger.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The statistics have been written into the state.
pub async fn load_overview(state: OrderState) {
    match get_json::<OverviewStatistics>(PATH_OVERVIEW).await {
        Ok(statistics) => {
            state.get_overview().set(Some(statistics));
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Loads the page of the records being displayed.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The records have been written into the state.
pub async fn load_records(state: OrderState) {
    let page: i32 = state.get_page().get();
    let start: String = state.get_start_date().get();
    let end: String = state.get_end_date().get();
    let category: String = state.get_filter_category().get();
    let kind: String = state.get_filter_type().get();
    state.get_loading().set(true);
    let mut path: String = format!("{PATH_RECORD_LIST}?page={page}&limit={PAGE_SIZE}");
    if !start.is_empty() {
        path.push_str(&format!("&start_date={}", encode_uri_component(&start)));
    }
    if !end.is_empty() {
        path.push_str(&format!("&end_date={}", encode_uri_component(&end)));
    }
    if !category.is_empty() {
        path.push_str(&format!("&category={}", encode_uri_component(&category)));
    }
    if !kind.is_empty() {
        path.push_str(&format!("&transaction_type={}", encode_uri_component(&kind)));
    }
    match get_json::<RecordList>(&path).await {
        Ok(list) => {
            state.get_records().set(list.records);
            state.get_total_income().set(list.total_income);
            state.get_total_expense().set(list.total_expense);
            state.get_balance().set(list.balance);
            state.get_total_count().set(list.total_count);
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
    state.get_loading().set(false);
}

/// Loads the page of the users being displayed.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The users have been written into the state.
pub async fn load_users(state: OrderState) {
    let page: i32 = state.get_users_page().get();
    let keyword: String = state.get_users_keyword().get();
    let cursor: String = users_cursor_of(&state, page);
    let mut path: String = format!("{PATH_USER_LIST}?limit={USER_PAGE_SIZE}");
    if !keyword.is_empty() {
        path.push_str(&format!("&keyword={}", encode_uri_component(&keyword)));
    }
    if !cursor.is_empty() {
        path.push_str(&format!("&last_id={}", encode_uri_component(&cursor)));
    }
    match get_json::<UserList>(&path).await {
        Ok(list) => {
            let cursors: Vec<String> = users_cursors_of(&state, page, &list);
            state.get_users().set(list.users);
            state.get_users_total().set(list.total_count);
            state.get_users_has_more().set(list.has_more);
            state.get_users_cursors().set(cursors);
            state.get_error().set(EMPTY.to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Creates the record the form holds.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The record has been created, or the failure has been reported.
pub async fn create_record(state: OrderState) {
    let amount: String = state.get_record_amount().get().trim().to_string();
    if amount.is_empty() {
        state.get_error().set(ERROR_EMPTY_AMOUNT.to_string());
        return;
    }
    if amount.parse::<f64>().is_err() {
        state.get_error().set(ERROR_INVALID_AMOUNT.to_string());
        return;
    }
    let description: String = state.get_record_description().get().trim().to_string();
    let bill_date: String = state.get_record_bill_date().get().trim().to_string();
    let target: String = state.get_record_target_user().get().trim().to_string();
    let body: CreateRecordRequest = CreateRecordRequest {
        amount,
        category: state.get_record_category().get(),
        transaction_type: state.get_record_type().get(),
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
        bill_date: if bill_date.is_empty() {
            None
        } else {
            Some(bill_date)
        },
        target_user_id: if target.is_empty() {
            None
        } else {
            Some(target)
        },
        image_ids: Vec::new(),
    };
    match post_json::<RecordItem, CreateRecordRequest>(PATH_RECORD_CREATE, &body).await {
        Ok(_) => {
            state.get_record_amount().set(EMPTY.to_string());
            state.get_record_description().set(EMPTY.to_string());
            state.get_record_bill_date().set(EMPTY.to_string());
            state.get_record_target_user().set(EMPTY.to_string());
            state.get_create_open().set(false);
            state.get_notice().set("Record created!".to_string());
            state.get_page().set(PAGE_FIRST);
            load_records(state).await;
            load_overview(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Saves the contact fields of the profile.
///
/// The fields are encrypted with the public key of the server, the same way
/// the credentials of the authentication page are.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The profile has been saved, or the failure has been reported.
pub async fn save_profile(state: OrderState) {
    let Some(user) = state.get_current_user().get() else {
        return;
    };
    let email: String = state.get_profile_email().get().trim().to_string();
    let phone: String = state.get_profile_phone().get().trim().to_string();
    let key: PublicKey = match public_key().await {
        Ok(key) => key,
        Err(message) => {
            state.get_error().set(message);
            return;
        }
    };
    let mut body: UpdateUserRequest = UpdateUserRequest {
        email: None,
        phone: None,
    };
    if !email.is_empty() {
        match encrypt_field(&email, &key) {
            Ok(value) => body.email = Some(value),
            Err(message) => {
                state.get_error().set(message);
                return;
            }
        }
    }
    if !phone.is_empty() {
        match encrypt_field(&phone, &key) {
            Ok(value) => body.phone = Some(value),
            Err(message) => {
                state.get_error().set(message);
                return;
            }
        }
    }
    let path: String = format!("{PATH_USER_UPDATE}/{}", user.id);
    match post_json::<UserProfile, UpdateUserRequest>(&path, &body).await {
        Ok(updated) => {
            state.get_current_user().set(Some(updated));
            state.get_notice().set("Profile updated!".to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Saves the password of the profile.
///
/// The passwords are encrypted with the public key of the server, the same
/// way the credentials of the authentication page are.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The password has been saved, or the failure has been reported.
pub async fn save_password(state: OrderState) {
    let Some(user) = state.get_current_user().get() else {
        return;
    };
    let old_password: String = state.get_old_password().get();
    let new_password: String = state.get_new_password().get();
    if old_password.is_empty() || new_password.is_empty() {
        state.get_error().set(ERROR_EMPTY_PASSWORD.to_string());
        return;
    }
    let key: PublicKey = match public_key().await {
        Ok(key) => key,
        Err(message) => {
            state.get_error().set(message);
            return;
        }
    };
    let encrypted_old: String = match encrypt_field(&old_password, &key) {
        Ok(value) => value,
        Err(message) => {
            state.get_error().set(message);
            return;
        }
    };
    let encrypted_new: String = match encrypt_field(&new_password, &key) {
        Ok(value) => value,
        Err(message) => {
            state.get_error().set(message);
            return;
        }
    };
    let body: ChangePasswordRequest = ChangePasswordRequest {
        old_password: encrypted_old,
        new_password: encrypted_new,
    };
    let path: String = format!("{PATH_USER_CHANGE_PASSWORD}/{}", user.id);
    match post_json::<UserProfile, ChangePasswordRequest>(&path, &body).await {
        Ok(_) => {
            state.get_old_password().set(EMPTY.to_string());
            state.get_new_password().set(EMPTY.to_string());
            state.get_notice().set("Password changed!".to_string());
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Fetches the public key the fields are encrypted with.
///
/// # Returns
///
/// - `Result<PublicKey, String>` - The key of the server, or the failure that stopped it.
async fn public_key() -> Result<PublicKey, String> {
    let key: RsaPublicKey = match get_json::<RsaPublicKey>(PATH_PUBLIC_KEY).await {
        Ok(key) => key,
        Err(_) => return Err(ERROR_KEY_FAILED.to_string()),
    };
    PublicKey::from_base64(&key.modulus, &key.exponent)
}

/// Returns the cursor the page of the users starts after.
///
/// # Arguments
///
/// - `&OrderState` - The state the page renders from.
/// - `i32` - The page that is loaded.
///
/// # Returns
///
/// - `String` - The identifier the page starts after, empty for the first page.
fn users_cursor_of(state: &OrderState, page: i32) -> String {
    if page <= PAGE_FIRST {
        return EMPTY.to_string();
    }
    state
        .get_users_cursors()
        .get()
        .get((page - 2) as usize)
        .cloned()
        .unwrap_or_default()
}

/// Returns the cursors of the pages of the users after the one that was loaded.
///
/// # Arguments
///
/// - `&OrderState` - The state the page renders from.
/// - `i32` - The page that was loaded.
/// - `&UserList` - The list the server answered with.
///
/// # Returns
///
/// - `Vec<String>` - The cursors of the pages.
fn users_cursors_of(state: &OrderState, page: i32, list: &UserList) -> Vec<String> {
    let mut cursors: Vec<String> = state.get_users_cursors().get();
    cursors.truncate(page as usize);
    while cursors.len() < page as usize {
        cursors.push(EMPTY.to_string());
    }
    if let Some(last) = list.users.last() {
        cursors[page as usize - 1] = last.id.clone();
    }
    cursors
}
