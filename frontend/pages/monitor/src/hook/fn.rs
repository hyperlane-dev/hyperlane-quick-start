use super::*;

/// Creates the reactive state the monitor page renders from.
///
/// The information and the history of the server are loaded as soon as the
/// page is mounted, and the live status stream is opened right after.
///
/// # Returns
///
/// - `MonitorState` - The state the page renders from.
pub fn use_monitor_state() -> MonitorState {
    let info: Signal<SystemInfo> = App::use_signal(SystemInfo::default);
    let status: Signal<Option<ServerStatus>> = App::use_signal(|| None);
    let history: Signal<Vec<PerformancePoint>> = App::use_signal(Vec::new);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let connected: Signal<bool> = App::use_signal(|| false);
    let state: MonitorState = MonitorState {
        info,
        status,
        history,
        loading,
        error,
        connected,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_overview(state));
        connect_stream(state);
        true
    });
    state
}

/// Loads the information and the performance history of the server.
///
/// # Arguments
///
/// - `MonitorState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The information and the history have been written into the state.
pub async fn load_overview(state: MonitorState) {
    state.get_loading().set(true);
    match get_json::<SystemInfo>(PATH_SERVER_INFO).await {
        Ok(info) => state.get_info().set(info),
        Err(error) => state.get_error().set(error.message),
    }
    match get_json::<PerformanceHistory>(PATH_SERVER_HISTORY).await {
        Ok(history) => {
            let mut points: Vec<PerformancePoint> = history.data_points;
            if points.len() > HISTORY_SAMPLES {
                points.drain(..points.len() - HISTORY_SAMPLES);
            }
            state.get_history().set(points);
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Opens the live status stream of the server.
///
/// The server answers the stream with one status frame per tick; every
/// frame is written into the state of the page.
///
/// # Arguments
///
/// - `MonitorState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The stream is open.
pub fn connect_stream(state: MonitorState) {
    let Ok(source) = EventSource::new(PATH_SERVER_STATUS) else {
        state.get_error().set(MESSAGE_STREAM_FAILED.to_string());
        return;
    };
    let message: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |event: MessageEvent| {
        let payload: String = event.data().as_string().unwrap_or_default();
        if let Ok(status) = serde_json::from_str::<ServerStatus>(&payload) {
            state.get_status().set(Some(status));
            state.get_connected().set(true);
        }
    });
    let open: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_connected().set(true);
    });
    let error: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_connected().set(false);
    });
    source.set_onmessage(Some(message.as_ref().unchecked_ref::<js_sys::Function>()));
    source.set_onopen(Some(open.as_ref().unchecked_ref::<js_sys::Function>()));
    source.set_onerror(Some(error.as_ref().unchecked_ref::<js_sys::Function>()));
    STREAM.with(|stream: &RefCell<Option<MonitorStream>>| {
        let mut slot: RefMut<'_, Option<MonitorStream>> = stream.borrow_mut();
        if let Some(previous) = slot.take() {
            previous.close();
        }
        *slot = Some(MonitorStream {
            source,
            message,
            open,
            error,
        });
    });
}

/// Formats a number of bytes as a readable size.
///
/// # Arguments
///
/// - `u64` - The number of the bytes.
///
/// # Returns
///
/// - `String` - The size the bytes stand for.
pub fn format_bytes(bytes: u64) -> String {
    let value: f64 = bytes as f64;
    if value >= BYTES_PER_GIBIBYTE {
        return format!("{:.2} GB", value / BYTES_PER_GIBIBYTE);
    }
    if value >= BYTES_PER_MEBIBYTE {
        return format!("{:.2} MB", value / BYTES_PER_MEBIBYTE);
    }
    if value >= BYTES_PER_KIBIBYTE {
        return format!("{:.2} KB", value / BYTES_PER_KIBIBYTE);
    }
    format!("{bytes} B")
}

/// Formats a share as a percentage with one decimal.
///
/// # Arguments
///
/// - `f64` - The share, as a percentage.
///
/// # Returns
///
/// - `String` - The share with its unit.
pub fn format_percent(share: f64) -> String {
    format!("{share:.1}%")
}

/// Formats the uptime of the server as days, hours and minutes.
///
/// # Arguments
///
/// - `u64` - The uptime in seconds.
///
/// # Returns
///
/// - `String` - The uptime of the server.
pub fn format_uptime(seconds: u64) -> String {
    let seconds: i64 = seconds as i64;
    let days: i64 = seconds / SECONDS_PER_DAY;
    let hours: i64 = (seconds % SECONDS_PER_DAY) / SECONDS_PER_HOUR;
    let minutes: i64 = (seconds % SECONDS_PER_HOUR) / SECONDS_PER_MINUTE;
    format!("{days}d {hours}h {minutes}m")
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
pub fn format_time(timestamp: i64) -> String {
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
