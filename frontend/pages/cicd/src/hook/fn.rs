use super::*;

/// Creates the reactive state the CI/CD page renders from.
///
/// The state holds the pipelines, the runs, the dialog that creates a
/// pipeline and the drawer that shows the detail of a run. The pipelines
/// and the runs are loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `CicdState` - The state the page renders from.
pub fn use_cicd_state() -> CicdState {
    let tab: Signal<String> = App::use_signal(|| TAB_PIPELINES.to_string());
    let pipelines: Signal<Vec<Pipeline>> = App::use_signal(Vec::new);
    let runs: Signal<Vec<Run>> = App::use_signal(Vec::new);
    let total: Signal<i32> = App::use_signal(|| 0);
    let has_more: Signal<bool> = App::use_signal(|| false);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let notice: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let create_open: Signal<bool> = App::use_signal(|| false);
    let draft_name: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let draft_description: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let draft_config: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let detail: Signal<Option<RunDetail>> = App::use_signal(|| None);
    let detail_open: Signal<bool> = App::use_signal(|| false);
    let logs: Signal<Vec<LogEntry>> = App::use_signal(Vec::new);
    let log_step: Signal<i32> = App::use_signal(|| 0);
    let streaming: Signal<bool> = App::use_signal(|| false);
    let state: CicdState = CicdState {
        tab,
        pipelines,
        runs,
        total,
        has_more,
        loading,
        error,
        notice,
        create_open,
        draft_name,
        draft_description,
        draft_config,
        detail,
        detail_open,
        logs,
        log_step,
        streaming,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_pipelines(state));
        spawn_local(load_runs(state, true));
        true
    });
    state
}

/// Loads the pipelines of the server.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The pipelines have been written into the state.
pub async fn load_pipelines(state: CicdState) {
    state.get_loading().set(true);
    match get_json::<Vec<Pipeline>>(PATH_PIPELINE_LIST).await {
        Ok(pipelines) => {
            state.get_pipelines().set(pipelines);
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Loads the runs of the server.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `bool` - Whether the list is loaded from the first run again.
///
/// # Returns
///
/// - `()` - The runs have been written into the state.
pub async fn load_runs(state: CicdState, reset: bool) {
    state.get_loading().set(true);
    let path: String = if reset {
        format!("{PATH_RUN_LIST}?page_size={RUNS_PAGE_SIZE}")
    } else {
        let last: i32 = state.get_runs().get().last().map(|run: &Run| run.id).unwrap_or(0);
        format!("{PATH_RUN_LIST}?page_size={RUNS_PAGE_SIZE}&last_id={last}")
    };
    match get_json::<RunPage>(&path).await {
        Ok(page) => {
            if reset {
                state.get_runs().set(page.runs);
            } else {
                let mut runs: Vec<Run> = state.get_runs().get();
                runs.extend(page.runs);
                state.get_runs().set(runs);
            }
            state.get_total().set(page.total);
            state.get_has_more().set(page.has_more);
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Creates the pipeline the dialog holds.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The pipeline has been created.
pub async fn create_pipeline(state: CicdState) {
    let name: String = state.get_draft_name().get().trim().to_string();
    if name.is_empty() {
        state
            .get_error()
            .set("A pipeline needs a name".to_string());
        return;
    }
    let description: String = state.get_draft_description().get().trim().to_string();
    let config: String = state.get_draft_config().get().trim().to_string();
    let body: CreatePipelineBody = CreatePipelineBody {
        name,
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
        config_content: if config.is_empty() { None } else { Some(config) },
    };
    state.get_loading().set(true);
    match post_json::<i32, CreatePipelineBody>(PATH_PIPELINE_CREATE, &body).await {
        Ok(_) => {
            state.get_draft_name().set(TEXT_EMPTY.to_string());
            state.get_draft_description().set(TEXT_EMPTY.to_string());
            state.get_draft_config().set(TEXT_EMPTY.to_string());
            state.get_create_open().set(false);
            state
                .get_notice()
                .set("The pipeline has been created".to_string());
            state.get_error().set(TEXT_EMPTY.to_string());
            load_pipelines(state).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Triggers a run of one pipeline.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the pipeline.
///
/// # Returns
///
/// - `()` - The run has been triggered.
pub async fn trigger_run(state: CicdState, pipeline_id: i32) {
    let body: TriggerRunBody = TriggerRunBody {
        pipeline_id,
        triggered_by: Some(TRIGGERED_BY.to_string()),
    };
    state.get_loading().set(true);
    match post_json::<i32, TriggerRunBody>(PATH_RUN_TRIGGER, &body).await {
        Ok(_) => {
            state
                .get_notice()
                .set("The run has been triggered".to_string());
            state.get_error().set(TEXT_EMPTY.to_string());
            load_runs(state, true).await;
        }
        Err(error) => state.get_error().set(error.message),
    }
    state.get_loading().set(false);
}

/// Opens the detail of one run.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the run.
///
/// # Returns
///
/// - `()` - The detail has been written into the state.
pub async fn open_run(state: CicdState, run_id: i32) {
    let path: String = format!("{PATH_RUN_DETAIL}?id={run_id}");
    match get_json::<RunDetail>(&path).await {
        Ok(detail) => {
            state.get_detail().set(Some(detail));
            state.get_detail_open().set(true);
            state.get_logs().set(Vec::new());
            state.get_error().set(TEXT_EMPTY.to_string());
        }
        Err(error) => state.get_error().set(error.message),
    }
}

/// Tails the log of one run.
///
/// The server streams the log of the run as a sequence of server sent
/// events; the stream is closed when the run finishes.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the run.
///
/// # Returns
///
/// - `()` - The stream has been opened.
pub fn tail_logs(state: CicdState, run_id: i32) {
    close_stream();
    let path: String = format!("{PATH_RUN_LOGS}?run_id={run_id}");
    let Ok(source): Result<EventSource, JsValue> = EventSource::new(&path) else {
        state
            .get_error()
            .set("The log stream could not be opened".to_string());
        return;
    };
    let on_log: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |event: MessageEvent| {
        let data: JsValue = event.data();
        let Some(raw): Option<String> = data.as_string() else {
            return;
        };
        let Ok(entry): Result<LogEntry, serde_json::Error> = serde_json::from_str(&raw) else {
            return;
        };
        let mut logs: Vec<LogEntry> = state.get_logs().get();
        logs.push(entry);
        state.get_logs().set(logs);
    });
    let on_complete: Closure<dyn FnMut(MessageEvent)> = Closure::new(move |_: MessageEvent| {
        state.get_streaming().set(false);
        close_stream();
    });
    let on_error: Closure<dyn FnMut(Event)> = Closure::new(move |_: Event| {
        state.get_streaming().set(false);
    });
    let target: EventTarget = source.clone().unchecked_into();
    let _logged: Result<(), JsValue> = target.add_event_listener_with_callback(
        EVENT_LOG,
        on_log.as_ref().unchecked_ref(),
    );
    let _completed: Result<(), JsValue> = target.add_event_listener_with_callback(
        EVENT_COMPLETE,
        on_complete.as_ref().unchecked_ref(),
    );
    let _failed: Result<(), JsValue> =
        target.add_event_listener_with_callback("error", on_error.as_ref().unchecked_ref());
    let handlers: Vec<JsValue> = vec![
        on_log.into_js_value(),
        on_complete.into_js_value(),
        on_error.into_js_value(),
    ];
    LOG_STREAM.with(|stream: &RefCell<Option<LogStream>>| {
        let previous: Option<LogStream> = stream.replace(Some(LogStream { source, handlers }));
        if let Some(previous) = previous {
            drop(previous.handlers);
        }
    });
    state.get_streaming().set(true);
}

/// Closes the log stream of the page.
///
/// # Returns
///
/// - `()` - The stream has been closed.
pub fn close_stream() {
    LOG_STREAM.with(|stream: &RefCell<Option<LogStream>>| {
        let previous: Option<LogStream> = stream.replace(None);
        if let Some(previous) = previous {
            previous.source.close();
            drop(previous.handlers);
        }
    });
}

/// Formats the duration of a run, a job or a step.
///
/// # Arguments
///
/// - `i32` - The duration in milliseconds.
///
/// # Returns
///
/// - `String` - The duration of the work.
pub fn format_duration(millis: i32) -> String {
    if millis < MILLIS_PER_SECOND as i32 {
        return format!("{millis} ms");
    }
    format!("{:.1} s", millis as f64 / MILLIS_PER_SECOND as f64)
}

/// Formats the moment a run, a job or a step was written as a short time.
///
/// # Arguments
///
/// - `Option<i64>` - The moment in milliseconds since the Unix epoch.
///
/// # Returns
///
/// - `String` - The time of the moment.
pub fn format_moment(moment: Option<i64>) -> String {
    let Some(timestamp): Option<i64> = moment else {
        return TEXT_EMPTY.to_string();
    };
    if timestamp <= 0 {
        return TEXT_EMPTY.to_string();
    }
    let date: js_sys::Date = js_sys::Date::new(&JsValue::from_f64(timestamp as f64));
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes()
    )
}
