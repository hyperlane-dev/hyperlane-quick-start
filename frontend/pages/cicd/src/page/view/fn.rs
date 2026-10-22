use super::*;

/// Renders the CI/CD page.
///
/// The page holds the tab bar over the pipelines and the runs, the dialog
/// that creates a pipeline and the drawer that shows the detail of a run.
///
/// # Arguments
///
/// - `VirtualNode<CicdPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn cicd_page(node: VirtualNode<CicdPageProps>) -> VirtualNode {
    let CicdPageProps { state }: CicdPageProps = node.try_get_props().unwrap_or_default();
    let pipelines: Signal<Vec<Pipeline>> = state.get_pipelines();
    let runs: Signal<Vec<Run>> = state.get_runs();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_PAGE
                title: TITLE_PAGE
                subtitle: SUBTITLE_PAGE
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: VARIANT_ERROR
                    state.get_error().get().as_str()
                }
            }
            if { !state.get_notice().get().is_empty() } {
                euv_alert {
                    variant: VARIANT_SUCCESS
                    state.get_notice().get().as_str()
                }
            }
            euv_tabs {
                items: TAB_ITEMS.to_vec()
                active: state.get_tab()
            }
            match { state.get_tab().get() } {
                tab if tab == TAB_PIPELINES => {
                    euv_card {
                        title: TITLE_PIPELINES
                        div {
                            class: c_cicd_toolbar()
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_PRIMARY
                                    onclick: open_create_handler(state)
                                    LABEL_CREATE
                                }
                            }
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_OUTLINE
                                    onclick: reload_handler(state)
                                    LABEL_RELOAD
                                }
                            }
                        }
                        match { state.get_loading().get() } {
                            true => {
                                euv_loading {
                                    title: TITLE_PIPELINES
                                }
                            }
                            false => {
                                match { pipelines.get().is_empty() } {
                                    true => {
                                        euv_result {
                                            code: "0"
                                            title: TITLE_EMPTY_PIPELINES
                                            description: TEXT_EMPTY_PIPELINES
                                        }
                                    }
                                    false => {
                                        div {
                                            class: c_cicd_list()
                                            for pipeline in { pipelines } {
                                                pipeline_row {
                                                    state: state
                                                    pipeline: pipeline
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    euv_card {
                        title: TITLE_RUNS
                        div {
                            class: c_cicd_toolbar()
                            div {
                                class: c_hls_button_row()
                                euv_button {
                                    variant: VARIANT_OUTLINE
                                    onclick: reload_handler(state)
                                    LABEL_RELOAD
                                }
                            }
                        }
                        match { state.get_loading().get() } {
                            true => {
                                euv_loading {
                                    title: TITLE_RUNS
                                }
                            }
                            false => {
                                match { runs.get().is_empty() } {
                                    true => {
                                        euv_result {
                                            code: "0"
                                            title: TITLE_EMPTY_RUNS
                                            description: TEXT_EMPTY_RUNS
                                        }
                                    }
                                    false => {
                                        div {
                                            class: c_cicd_list()
                                            for run in { runs } {
                                                run_row {
                                                    state: state
                                                    run: run
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if { state.get_create_open().get() } {
                euv_modal {
                    title: TITLE_CREATE
                    onclick: close_create_handler(state)
                    div {
                        class: c_cicd_form()
                        euv_field {
                            id: ID_NAME
                            label: LABEL_NAME
                            placeholder: PLACEHOLDER_NAME
                            value: state.get_draft_name()
                            oninput: name_handler(state)
                        }
                        euv_field {
                            id: ID_DESCRIPTION
                            label: LABEL_DESCRIPTION
                            placeholder: PLACEHOLDER_DESCRIPTION
                            value: state.get_draft_description()
                            oninput: description_handler(state)
                        }
                        div {
                            class: c_form_label()
                            LABEL_CONFIG
                        }
                        textarea {
                            id: ID_CONFIG
                            class: c_textarea_input()
                            placeholder: PLACEHOLDER_CONFIG
                            rows: ROWS_CONFIG
                            oninput: config_handler(state)
                        }
                        div {
                            class: c_button_controls()
                            euv_button {
                                variant: VARIANT_PRIMARY
                                onclick: create_handler(state)
                                LABEL_SAVE
                            }
                        }
                    }
                }
            }
            euv_drawer {
                open: state.get_detail_open()
                run_detail {
                    state: state
                }
            }
        }
    }
}

/// Renders one pipeline of the pipeline list.
///
/// # Arguments
///
/// - `VirtualNode<PipelineRowProps>` - The props node carrying the pipeline.
///
/// # Returns
///
/// - `VirtualNode` - The row tree.
#[component]
pub fn pipeline_row(node: VirtualNode<PipelineRowProps>) -> VirtualNode {
    let PipelineRowProps { state, pipeline }: PipelineRowProps =
        node.try_get_props().unwrap_or_default();
    let description: String = pipeline.description.clone().unwrap_or_default();
    let config: String = pipeline.config_content.clone().unwrap_or_default();
    let created: String = format_moment(pipeline.created_at);
    html! {
        div {
            key: pipeline.id.to_string()
            class: c_cicd_item()
            div {
                class: c_cicd_item_header()
                span {
                    class: c_cicd_item_title()
                    pipeline.name.as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    created.as_str()
                }
            }
            if !description.is_empty() {
                p {
                    class: c_cicd_item_text()
                    description.as_str()
                }
            }
            if !config.is_empty() {
                pre {
                    class: c_cicd_item_config()
                    config.as_str()
                }
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: VARIANT_PRIMARY
                    onclick: trigger_handler(state, pipeline.id)
                    LABEL_TRIGGER
                }
            }
        }
    }
}

/// Renders one run of the run list.
///
/// # Arguments
///
/// - `VirtualNode<RunRowProps>` - The props node carrying the run.
///
/// # Returns
///
/// - `VirtualNode` - The row tree.
#[component]
pub fn run_row(node: VirtualNode<RunRowProps>) -> VirtualNode {
    let RunRowProps { state, run }: RunRowProps = node.try_get_props().unwrap_or_default();
    let pipeline: String = pipeline_name(state, run.pipeline_id);
    let triggered: String = run.triggered_by.clone().unwrap_or_default();
    let duration: String = format_duration(run.duration_ms);
    let started: String = format_moment(run.started_at);
    let status: String = run.status.to_string();
    html! {
        div {
            key: run.id.to_string()
            class: c_cicd_item()
            div {
                class: c_cicd_item_header()
                span {
                    class: c_cicd_item_title()
                    format!("#{} {}", run.run_number, pipeline).as_str()
                }
                span {
                    class: c_cicd_item_status()
                    status.as_str()
                }
            }
            div {
                class: c_cicd_item_meta_row()
                span {
                    class: c_cicd_item_meta()
                    triggered.as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    duration.as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    started.as_str()
                }
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: VARIANT_OUTLINE
                    onclick: detail_handler(state, run.id)
                    LABEL_DETAIL
                }
            }
        }
    }
}

/// Renders the detail of the open run.
///
/// # Arguments
///
/// - `VirtualNode<RunDetailProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The detail tree.
#[component]
pub fn run_detail(node: VirtualNode<RunDetailProps>) -> VirtualNode {
    let RunDetailProps { state }: RunDetailProps = node.try_get_props().unwrap_or_default();
    let Some(detail): Option<RunDetail> = state.get_detail().get() else {
        return html! {
            div {
                class: c_cicd_detail()
                span {
                    class: c_cicd_item_text()
                    TEXT_EMPTY_RUNS
                }
            }
        };
    };
    let pipeline: String = pipeline_name(state, detail.run.pipeline_id);
    let triggered: String = detail.run.triggered_by.clone().unwrap_or_default();
    let duration: String = format_duration(detail.run.duration_ms);
    let started: String = format_moment(detail.run.started_at);
    let status: String = detail.run.status.to_string();
    html! {
        div {
            class: c_cicd_detail()
            span {
                class: c_cicd_detail_title()
                TITLE_DETAIL
            }
            div {
                class: c_cicd_detail_header()
                span {
                    class: c_cicd_item_title()
                    format!("#{} {}", detail.run.run_number, pipeline).as_str()
                }
                span {
                    class: c_cicd_item_status()
                    status.as_str()
                }
            }
            div {
                class: c_cicd_item_meta_row()
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_NUMBER}: {}", detail.run.run_number).as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_STATUS}: {status}").as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_PIPELINE}: {pipeline}").as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_TRIGGERED_BY}: {triggered}").as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_DURATION}: {duration}").as_str()
                }
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_STARTED}: {started}").as_str()
                }
            }
            span {
                class: c_cicd_detail_title()
                TITLE_JOBS
            }
            for job in detail.jobs.clone() {
                div {
                    class: c_cicd_job()
                    div {
                        class: c_cicd_item_header()
                        span {
                            class: c_cicd_item_title()
                            job.job.name.as_str()
                        }
                        span {
                            class: c_cicd_item_status()
                            job.job.status.to_string().as_str()
                        }
                    }
                    span {
                        class: c_cicd_detail_title()
                        TITLE_STEPS
                    }
                    for step in job.steps.clone() {
                        step_block {
                            state: state
                            run_id: detail.run.id
                            step: step
                        }
                    }
                }
            }
            run_log {
                state: state
            }
        }
    }
}

/// Renders one step of a job of a run.
///
/// The block shows the command the step carries out and the output the step
/// wrote, so a finished run keeps the evidence of what it did.
///
/// # Arguments
///
/// - `VirtualNode<StepBlockProps>` - The props node carrying the step.
///
/// # Returns
///
/// - `VirtualNode` - The step tree.
#[component]
pub fn step_block(node: VirtualNode<StepBlockProps>) -> VirtualNode {
    let StepBlockProps { state, run_id, step }: StepBlockProps =
        node.try_get_props().unwrap_or_default();
    let command: String = step.command.clone().unwrap_or_default();
    let output: String = step.output.clone().unwrap_or_default();
    html! {
        div {
            class: c_cicd_step()
            div {
                class: c_cicd_item_header()
                span {
                    class: c_cicd_item_meta()
                    step.name.as_str()
                }
                span {
                    class: c_cicd_item_status()
                    step.status.to_string().as_str()
                }
            }
            if !command.is_empty() {
                span {
                    class: c_cicd_item_meta()
                    format!("{LABEL_COMMAND}: {command}").as_str()
                }
            }
            if !output.is_empty() {
                pre {
                    class: c_cicd_log_body()
                    output.as_str()
                }
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: VARIANT_OUTLINE
                    onclick: tail_handler(state, run_id)
                    LABEL_TAIL
                }
            }
        }
    }
}

/// Returns the name of the pipeline of a run.
///
/// The server leaves the name of the pipeline out of the run, so the name is
/// looked up in the pipelines the page has loaded.
///
/// # Arguments
///
/// - `CicdState` - The state of the page.
/// - `i32` - The identifier of the pipeline.
///
/// # Returns
///
/// - `String` - The name of the pipeline, empty when the page does not know
///   the pipeline.
pub fn pipeline_name(state: CicdState, pipeline_id: i32) -> String {
    let pipelines: Signal<Vec<Pipeline>> = state.get_pipelines();
    let found: Option<Pipeline> = pipelines
        .get()
        .into_iter()
        .find(|pipeline: &Pipeline| pipeline.id == pipeline_id);
    match found {
        Some(pipeline) => pipeline.name,
        None => String::new(),
    }
}

/// Renders the log of the open run.
///
/// # Arguments
///
/// - `VirtualNode<RunLogProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The log tree.
#[component]
pub fn run_log(node: VirtualNode<RunLogProps>) -> VirtualNode {
    let RunLogProps { state }: RunLogProps = node.try_get_props().unwrap_or_default();
    let logs: Signal<Vec<LogEntry>> = state.get_logs();
    html! {
        div {
            class: c_cicd_log()
            div {
                class: c_cicd_item_header()
                span {
                    class: c_cicd_detail_title()
                    TITLE_LOG
                }
                span {
                    class: c_cicd_item_meta()
                    match { state.get_streaming().get() } {
                        true => {
                            TEXT_STREAMING
                        }
                        false => {
                            TEXT_NOT_STREAMING
                        }
                    }
                }
            }
            match { logs.get().is_empty() } {
                true => {
                    span {
                        class: c_cicd_item_text()
                        TEXT_NO_OUTPUT
                    }
                }
                false => {
                    pre {
                        class: c_cicd_log_body()
                        for entry in { logs } {
                            span {
                                class: if entry.is_stderr {
                                    c_cicd_log_line_error()
                                } else {
                                    c_cicd_log_line()
                                }
                                entry.content.as_str()
                            }
                        }
                    }
                }
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: VARIANT_OUTLINE
                    onclick: stop_handler(state)
                    LABEL_STOP
                }
            }
        }
    }
}

/// Builds the handler that opens the dialog that creates a pipeline.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_create_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(true);
        state.get_error().set(TEXT_EMPTY.to_string());
    }))
}

/// Builds the handler that closes the dialog that creates a pipeline.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_create_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(false);
    }))
}

/// Builds the handler that creates the pipeline of the dialog.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_pipeline(state));
    }))
}

/// Builds the handler that reloads the lists of the page.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn reload_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(load_pipelines(state));
        spawn_local(load_runs(state, true));
    }))
}

/// Builds the handler that triggers a run of one pipeline.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the pipeline.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn trigger_handler(state: CicdState, pipeline_id: i32) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(trigger_run(state, pipeline_id));
    }))
}

/// Builds the handler that opens the detail of one run.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the run.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn detail_handler(state: CicdState, run_id: i32) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(open_run(state, run_id));
    }))
}

/// Builds the handler that tails the log of one run.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
/// - `i32` - The identifier of the run.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn tail_handler(state: CicdState, run_id: i32) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        tail_logs(state, run_id);
    }))
}

/// Builds the handler that closes the log stream of the page.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn stop_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        close_stream();
        state.get_streaming().set(false);
    }))
}

/// Builds the handler that keeps the name of the pipeline being created.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn name_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_name().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the description of the pipeline being created.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn description_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_description().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the configuration of the pipeline being created.
///
/// # Arguments
///
/// - `CicdState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn config_handler(state: CicdState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_draft_config().set(textarea_value(&event));
    }))
}
