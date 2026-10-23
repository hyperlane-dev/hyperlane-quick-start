use super::*;

/// Creates the reactive state the euv playground page renders from.
///
/// The state holds the projects of the visitor, the project the editor opens
/// and the state of the build that runs in the background. The list of the
/// projects is loaded as soon as the page is mounted.
///
/// # Returns
///
/// - `PlaygroundState` - The state the page renders from.
pub fn use_playground_state() -> PlaygroundState {
    let projects: Signal<Vec<PlaygroundProject>> = App::use_signal(Vec::new);
    let current: Signal<Option<PlaygroundProjectDetail>> = App::use_signal(|| None);
    let name: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let code: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let dirty: Signal<bool> = App::use_signal(|| false);
    let loading: Signal<bool> = App::use_signal(|| true);
    let error: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let status: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let stderr: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let preview_url: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let running: Signal<bool> = App::use_signal(|| false);
    let creating: Signal<bool> = App::use_signal(|| false);
    let new_name: Signal<String> = App::use_signal(|| TEXT_EMPTY.to_string());
    let state: PlaygroundState = PlaygroundState {
        projects,
        current,
        name,
        code,
        dirty,
        loading,
        error,
        status,
        stderr,
        preview_url,
        running,
        creating,
        new_name,
    };
    let _started: Signal<bool> = App::use_signal(|| {
        spawn_local(load_projects(state));
        true
    });
    state
}

/// Loads the projects of the visitor.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
pub async fn load_projects(state: PlaygroundState) {
    state.get_loading().set(true);
    match get_json::<Vec<PlaygroundProject>>(PATH_PROJECTS).await {
        Ok(projects) => {
            state.get_projects().set(projects.clone());
            state.get_error().set(TEXT_EMPTY.to_string());
            if state.get_current().get().is_none()
                && let Some(first) = projects.first()
            {
                open_project(state, first.id.clone()).await;
            }
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
    state.get_loading().set(false);
}

/// Opens a project in the editor.
///
/// The code of the project is written into the editor of the page, so the
/// visitor sees the code of the project it picked.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
/// - `String` - The identifier of the project.
///
/// # Returns
///
/// - `()` - The project has been written into the state.
pub async fn open_project(state: PlaygroundState, id: String) {
    let path: String = format!("{PATH_PROJECTS_GET}/{id}");
    state.get_loading().set(true);
    match get_json::<PlaygroundProjectDetail>(&path).await {
        Ok(detail) => {
            state.get_current().set(Some(detail.clone()));
            state.get_name().set(detail.name);
            state.get_code().set(detail.code.clone());
            state.get_dirty().set(false);
            state.get_error().set(TEXT_EMPTY.to_string());
            set_editor_value(&detail.code);
        }
        Err(error) => {
            state.get_error().set(error.message);
            load_default_code(state).await;
        }
    }
    state.get_loading().set(false);
}

/// Loads the template of a project the server does not know.
///
/// The template of the server is the code a new project starts with, so the
/// editor is never left empty.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The template has been written into the state.
pub async fn load_default_code(state: PlaygroundState) {
    match get_json::<PlaygroundDefaultCode>(PATH_DEFAULT_CODE).await {
        Ok(default) => {
            state.get_code().set(default.code.clone());
            state.get_dirty().set(true);
            set_editor_value(&default.code);
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Creates a project and opens it in the editor.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The project has been created and written into the state.
pub async fn create_project(state: PlaygroundState) {
    let name: String = state.get_new_name().get();
    let name: String = name.trim().to_string();
    if name.is_empty() {
        state
            .get_error()
            .set(MESSAGE_NAME_REQUIRED.to_string());
        return;
    }
    let body: PlaygroundCreateRequest = PlaygroundCreateRequest { name };
    match post_json::<PlaygroundProjectMutation, PlaygroundCreateRequest>(
        PATH_PROJECTS_CREATE,
        &body,
    )
    .await
    {
        Ok(created) => {
            state.get_creating().set(false);
            state.get_new_name().set(TEXT_EMPTY.to_string());
            state.get_error().set(TEXT_EMPTY.to_string());
            load_projects(state).await;
            open_project(state, created.id).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Removes the project the editor opens.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The project has been removed.
pub async fn delete_project(state: PlaygroundState) {
    let Some(current) = state.get_current().get() else {
        return;
    };
    let path: String = format!("{PATH_PROJECTS_DELETE}/{}", current.id);
    match delete_json::<PlaygroundProjectMutation>(&path).await {
        Ok(deleted) => {
            if deleted.deleted {
                state.get_current().set(None);
                state.get_name().set(TEXT_EMPTY.to_string());
                state.get_code().set(TEXT_EMPTY.to_string());
                state.get_dirty().set(false);
                set_editor_value(TEXT_EMPTY);
            }
            state.get_error().set(TEXT_EMPTY.to_string());
            load_projects(state).await;
        }
        Err(error) => {
            state.get_error().set(error.message);
        }
    }
}

/// Keeps the code the visitor wrote.
///
/// The code is written into the project a moment later, so the visitor does
/// not wait for the server after every keystroke.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
/// - `String` - The code the visitor wrote.
///
/// # Returns
///
/// - `()` - The code has been written into the state.
pub fn keep_code(state: PlaygroundState, code: String) {
    state.get_code().set(code);
    state.get_dirty().set(true);
    schedule_save(state);
}

/// Writes the code of the project into the server.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The code has been written into the state.
pub async fn save_project(state: PlaygroundState) {
    let Some(current) = state.get_current().get() else {
        return;
    };
    if !state.get_dirty().get() {
        return;
    }
    let path: String = format!("{PATH_PROJECTS_SAVE}/{}", current.id);
    let name: String = state.get_name().get();
    let code: String = state.get_code().get();
    let body: PlaygroundSaveRequest = PlaygroundSaveRequest {
        name: Some(name),
        code: Some(code),
    };
    match put_json::<PlaygroundProjectMutation, PlaygroundSaveRequest>(&path, &body).await {
        Ok(saved) => {
            state.get_dirty().set(false);
            state.get_error().set(TEXT_EMPTY.to_string());
            state.get_status().set(MESSAGE_SAVED.to_string());
            update_project(state, &saved);
        }
        Err(error) => {
            state.get_error().set(error.message);
            state.get_status().set(MESSAGE_SAVE_FAILED.to_string());
        }
    }
}

/// Starts a build of the project the editor opens.
///
/// The page follows the build until it is over, so the preview shows the page
/// the build produced.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The build is over, or it failed.
pub async fn run_project(state: PlaygroundState) {
    if state.get_running().get() {
        return;
    }
    let Some(current) = state.get_current().get() else {
        state
            .get_error()
            .set(MESSAGE_NO_PROJECT.to_string());
        return;
    };
    if state.get_dirty().get() {
        save_project(state).await;
    }
    let code: String = state.get_code().get();
    let body: PlaygroundRunRequest = PlaygroundRunRequest {
        project_id: current.id.clone(),
        code: Some(code),
    };
    state.get_running().set(true);
    state.get_stderr().set(TEXT_EMPTY.to_string());
    state.get_status().set(MESSAGE_SUBMITTING.to_string());
    match post_json::<PlaygroundRun, PlaygroundRunRequest>(PATH_RUN, &body).await {
        Ok(run) => {
            if !run.ok {
                state.get_running().set(false);
                state.get_status().set(MESSAGE_SUBMIT_FAILED.to_string());
                state.get_stderr().set(failure_text(&run.message));
                return;
            }
            follow_build(state, run.job_id, now_millis()).await;
        }
        Err(error) => {
            state.get_running().set(false);
            state.get_status().set(MESSAGE_SUBMIT_FAILED.to_string());
            state.get_stderr().set(failure_text(&error.message));
        }
    }
}

/// Follows a build until it is over.
///
/// The page asks the server for the status of the build every couple of
/// seconds, and gives up when the build takes longer than the timeout of the
/// page.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
/// - `String` - The identifier of the build.
/// - `i64` - The moment the build was started.
///
/// # Returns
///
/// - `()` - The build is over, or the page gave up.
async fn follow_build(state: PlaygroundState, job_id: String, started_at: i64) {
    let path: String = format!("{PATH_RUN_STATUS}/{job_id}");
    loop {
        if now_millis() - started_at > POLL_TIMEOUT_MS {
            state.get_running().set(false);
            state.get_status().set(MESSAGE_TIMEOUT.to_string());
            state.get_stderr().set(MESSAGE_TIMEOUT_DETAIL.to_string());
            return;
        }
        let elapsed: i64 = (now_millis() - started_at) / MILLIS_SECOND;
        match get_json::<PlaygroundBuildStatus>(&path).await {
            Ok(build) => {
                match build.status.as_str() {
                    STATUS_PENDING => {
                        state
                            .get_status()
                            .set(format!("{MESSAGE_QUEUED} ({elapsed}s)"));
                    }
                    STATUS_RUNNING => {
                        state
                            .get_status()
                            .set(format!("{MESSAGE_BUILDING} ({elapsed}s)"));
                    }
                    STATUS_SUCCESS => {
                        state.get_running().set(false);
                        state.get_status().set(MESSAGE_RUNNING.to_string());
                        state.get_preview_url().set(build.build_url);
                        return;
                    }
                    STATUS_FAILED => {
                        state.get_running().set(false);
                        state.get_status().set(MESSAGE_BUILD_FAILED.to_string());
                        state.get_stderr().set(failure_text(&build.stderr));
                        return;
                    }
                    _ => {
                        state.get_running().set(false);
                        state
                            .get_status()
                            .set(format!("{MESSAGE_UNKNOWN_STATUS} {}", build.status));
                        return;
                    }
                }
            }
            Err(error) => {
                state.get_error().set(error.message);
            }
        }
        sleep(POLL_INTERVAL_MS).await;
    }
}

/// Writes the saved project into the list of the projects.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
/// - `&PlaygroundProjectMutation` - The project the server reported.
///
/// # Returns
///
/// - `()` - The list has been written into the state.
fn update_project(state: PlaygroundState, saved: &PlaygroundProjectMutation) {
    let mut projects: Vec<PlaygroundProject> = state.get_projects().get();
    let code: String = state.get_code().get();
    let size: u64 = code.len() as u64;
    for project in projects.iter_mut() {
        if project.id != saved.id {
            continue;
        }
        project.name = saved.name.clone();
        project.updated_at_ms = saved.updated_at_ms;
        project.code_size = size;
    }
    state.get_projects().set(projects);
}

/// Writes the code of the project into the server a moment later.
///
/// The timer of the last call is cancelled, so the code is written once the
/// visitor stops typing.
///
/// # Arguments
///
/// - `PlaygroundState` - The state the page renders from.
///
/// # Returns
///
/// - `()` - The code is written into the server later.
fn schedule_save(state: PlaygroundState) {
    cancel_save();
    let Some(window): Option<Window> = window() else {
        return;
    };
    let closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
        spawn_local(save_project(state));
    }));
    let Ok(handle): Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        SAVE_DELAY_MS,
    ) else {
        return;
    };
    closure.forget();
    SAVE_TIMER.with(|timer: &Cell<i32>| timer.set(handle));
}

/// Cancels the timer that writes the code of the project.
///
/// # Returns
///
/// - `()` - The timer of the last call is cancelled.
fn cancel_save() {
    let handle: i32 = SAVE_TIMER.with(|timer: &Cell<i32>| {
        let current: i32 = timer.get();
        timer.set(0);
        current
    });
    if handle == 0 {
        return;
    }
    let Some(window): Option<Window> = window() else {
        return;
    };
    window.clear_timeout_with_handle(handle);
}

/// Returns the text of a failure of the server.
///
/// # Arguments
///
/// - `&str` - The message of the server.
///
/// # Returns
///
/// - `String` - The text of the failure.
fn failure_text(message: &str) -> String {
    if message.trim().is_empty() {
        return MESSAGE_NO_DETAIL.to_string();
    }
    message.to_string()
}

/// Returns the moment the browser counts in milliseconds.
///
/// # Returns
///
/// - `i64` - The moment in milliseconds since the Unix epoch.
fn now_millis() -> i64 {
    js_sys::Date::now() as i64
}

/// Waits for a moment.
///
/// # Arguments
///
/// - `i32` - The number of milliseconds to wait.
///
/// # Returns
///
/// - `()` - The moment is over.
async fn sleep(millis: i32) {
    let promise: js_sys::Promise = js_sys::Promise::new(
        &mut |resolve: js_sys::Function, _reject: js_sys::Function| {
        let Some(window): Option<Window> = window() else {
            return;
        };
        let closure: JsValue = Closure::once_into_js(move || {
            let _: Result<JsValue, JsValue> = resolve.call0(&JsValue::NULL);
        });
        let _: Result<i32, JsValue> = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref(),
            millis,
        );
    });
    let _: Result<JsValue, JsValue> = JsFuture::from(promise).await;
}
