use super::*;

/// Creates the reactive state the tracking page renders from.
///
/// The first page of the records is loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `TrackingState` - The state the page renders from.
pub fn use_tracking_state() -> TrackingState {
    let start_time: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let end_time: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let header_key: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let header_value: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let body_content: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let page: Signal<i64> = App::use_signal(|| PAGE_FIRST);
    let page_size: Signal<i64> = App::use_signal(|| PAGE_SIZE);
    let cache_id: Signal<i64> = App::use_signal(|| 0);
    let records: Signal<Vec<TrackingRecord>> = App::use_signal(Vec::new);
    let total: Signal<i64> = App::use_signal(|| 0);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let detail: Signal<Option<TrackingRecord>> = App::use_signal(|| None);
    let state: TrackingState = TrackingState {
        start_time,
        end_time,
        header_key,
        header_value,
        body_content,
        page,
        page_size,
        cache_id,
        records,
        total,
        loading,
        error,
        detail,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(run_query(state));
        true
    });
    state
}

/// Runs the query the filters of the page describe.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The records have been written into the state.
pub async fn run_query(state: TrackingState) {
    state.get_loading().set(true);
    let body: TrackingQuery = build_query(state);
    match post_json::<TrackingQueryResult, TrackingQuery>(PATH_TRACKING_QUERY, &body).await {
        Ok(result) => {
            let first_id: Option<i64> = result.records.first().map(|record: &TrackingRecord| record.id);
            let page: i64 = result.page;
            state.get_records().set(result.records);
            state.get_total().set(result.total);
            state.get_page().set(page);
            if page == PAGE_FIRST && state.get_cache_id().get() == 0 {
                if let Some(id) = first_id {
                    state.get_cache_id().set(id);
                }
            }
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Builds the body of the query the filters of the page describe.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `TrackingQuery` - The body of the query.
fn build_query(state: TrackingState) -> TrackingQuery {
    let cache_id: i64 = state.get_cache_id().get();
    TrackingQuery {
        start_time: parse_local_time(&state.get_start_time().get()),
        end_time: parse_local_time(&state.get_end_time().get()),
        header_key: non_empty(state.get_header_key().get()),
        header_value: non_empty(state.get_header_value().get()),
        body_content: non_empty(state.get_body_content().get()),
        page: Some(state.get_page().get()),
        page_size: Some(state.get_page_size().get()),
        cache_id: if cache_id == 0 { None } else { Some(cache_id) },
    }
}

/// Clears the filters of the page and runs the query again.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The filters are cleared and the records have been reloaded.
pub async fn reset_query(state: TrackingState) {
    state.get_start_time().set(TEXT_EMPTY.to_string());
    state.get_end_time().set(TEXT_EMPTY.to_string());
    state.get_header_key().set(TEXT_EMPTY.to_string());
    state.get_header_value().set(TEXT_EMPTY.to_string());
    state.get_body_content().set(TEXT_EMPTY.to_string());
    state.get_page().set(PAGE_FIRST);
    state.get_cache_id().set(0);
    run_query(state).await;
}

/// Reads a text the visitor typed, dropping the blank ones.
///
/// # Arguments
///
/// - `String` - The typed text.
///
/// # Returns
///
/// - `Option<String>` - The text, absent when it is blank.
fn non_empty(text: String) -> Option<String> {
    let trimmed: String = text.trim().to_string();
    if trimmed.is_empty() {
        return None;
    }
    Some(trimmed)
}

/// Reads the local date and time the visitor typed.
///
/// # Arguments
///
/// - `&str` - The typed date and time, as the browser reports it.
///
/// # Returns
///
/// - `Option<i64>` - The moment in milliseconds since the Unix epoch, absent when the field is blank.
pub fn parse_local_time(value: &str) -> Option<i64> {
    let trimmed: &str = value.trim();
    if trimmed.is_empty() {
        return None;
    }
    let date: js_sys::Date = js_sys::Date::new(&JsValue::from_str(trimmed));
    let time: f64 = date.get_time();
    if time.is_nan() {
        return None;
    }
    Some(time as i64)
}

/// Formats a moment as the local date and time of the browser.
///
/// # Arguments
///
/// - `i64` - The moment in milliseconds since the Unix epoch.
///
/// # Returns
///
/// - `String` - The local date and time of the moment.
pub fn format_local_time(timestamp: i64) -> String {
    let offset: i64 = (js_sys::Date::new_0().get_timezone_offset() as i64) * MILLIS_PER_MINUTE;
    let local: i64 = timestamp - offset;
    let days: i64 = local.div_euclid(MILLIS_PER_DAY);
    let time: i64 = local.rem_euclid(MILLIS_PER_DAY);
    let (year, month, day) = civil_from_days(days);
    format!(
        "{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}",
        time / MILLIS_PER_HOUR,
        (time % MILLIS_PER_HOUR) / MILLIS_PER_MINUTE,
        (time % MILLIS_PER_MINUTE) / MILLIS_PER_SECOND
    )
}

/// Converts the days since the Unix epoch into a year, a month and a day.
///
/// # Arguments
///
/// - `i64` - The days since the Unix epoch.
///
/// # Returns
///
/// - `(i64, u32, u32)` - The year, the month and the day of the date.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let shifted: i64 = days + DAYS_SHIFT;
    let era: i64 = if shifted >= 0 {
        shifted / DAYS_PER_ERA
    } else {
        (shifted - DAYS_PER_ERA) / DAYS_PER_ERA
    };
    let day_of_era: i64 = shifted - era * DAYS_PER_ERA;
    let year_of_era: i64 = (day_of_era - day_of_era / 1460 + day_of_era / 36524
        - day_of_era / 146096)
        / 365;
    let year: i64 = year_of_era + era * YEARS_PER_ERA;
    let day_of_year: i64 =
        day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_of_year: i64 = (5 * day_of_year + 2) / 153;
    let day: u32 = (day_of_year - (153 * month_of_year + 2) / 5 + 1) as u32;
    let month: u32 = if month_of_year < 10 {
        (month_of_year + 3) as u32
    } else {
        (month_of_year - 9) as u32
    };
    let year: i64 = if month <= 2 { year + 1 } else { year };
    (year, month, day)
}
