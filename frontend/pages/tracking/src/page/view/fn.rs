use super::*;

/// Renders the tracking page.
///
/// The page holds the filters of the query, the metrics of the answer and
/// the table of the records, with the detail of a record opened in a modal.
///
/// # Arguments
///
/// - `VirtualNode<TrackingPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn tracking_page(node: VirtualNode<TrackingPageProps>) -> VirtualNode {
    let TrackingPageProps { state }: TrackingPageProps = node.try_get_props().unwrap_or_default();
    let records: Signal<Vec<TrackingRecord>> = state.get_records();
    let total: i64 = state.get_total().get();
    let page: i64 = state.get_page().get();
    let page_size: i64 = state.get_page_size().get();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_TRACKING
                title: TITLE_TRACKING
                subtitle: SUBTITLE_TRACKING
            }
            if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            euv_card {
                title: TITLE_SEARCH
                div {
                    class: c_tracking_form()
                    euv_input {
                        id: ID_START_TIME
                        label: LABEL_START_TIME
                        input_type: INPUT_TYPE_DATETIME
                        placeholder: PLACEHOLDER_START_TIME
                        value: state.get_start_time()
                        oninput: start_time_handler(state)
                    }
                    euv_input {
                        id: ID_END_TIME
                        label: LABEL_END_TIME
                        input_type: INPUT_TYPE_DATETIME
                        placeholder: PLACEHOLDER_END_TIME
                        value: state.get_end_time()
                        oninput: end_time_handler(state)
                    }
                    euv_input {
                        id: ID_HEADER_KEY
                        label: LABEL_HEADER_KEY
                        placeholder: PLACEHOLDER_HEADER_KEY
                        value: state.get_header_key()
                        oninput: header_key_handler(state)
                    }
                    euv_input {
                        id: ID_HEADER_VALUE
                        label: LABEL_HEADER_VALUE
                        placeholder: PLACEHOLDER_HEADER_VALUE
                        value: state.get_header_value()
                        oninput: header_value_handler(state)
                    }
                    euv_input {
                        id: ID_BODY_CONTENT
                        label: LABEL_BODY_CONTENT
                        placeholder: PLACEHOLDER_BODY_CONTENT
                        value: state.get_body_content()
                        oninput: body_content_handler(state)
                    }
                    div {
                        class: c_button_controls()
                        euv_button {
                            variant: EuvButtonVariant::Primary
                            onclick: search_handler(state)
                            LABEL_SEARCH
                        }
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: reset_handler(state)
                            LABEL_RESET
                        }
                    }
                }
            }
            div {
                class: c_tracking_stats()
                stat_tile {
                    label: LABEL_TOTAL
                    value: total.to_string()
                    icon: ICON_TOTAL
                }
                stat_tile {
                    label: LABEL_PAGE
                    value: page.to_string()
                    icon: ICON_PAGE
                }
                stat_tile {
                    label: LABEL_PAGES
                    value: total_pages(total, page_size).to_string()
                    icon: ICON_PAGES
                }
            }
            euv_card {
                title: TITLE_RESULTS
                if { state.get_loading().get() } {
                    euv_loading {
                        title: TITLE_LOADING
                    }
                } else if { records.get().is_empty() } {
                    euv_result {
                        code: CODE_EMPTY
                        title: TITLE_EMPTY
                        description: TEXT_NO_RECORD
                    }
                } else {
                    div {
                        class: c_tracking_table_wrap()
                        table {
                            class: c_tracking_table()
                            thead {
                                tr {
                                    th {
                                        class: c_tracking_table_head()
                                        HEAD_ID
                                    }
                                    th {
                                        class: c_tracking_table_head()
                                        HEAD_TIMESTAMP
                                    }
                                    th {
                                        class: c_tracking_table_head()
                                        HEAD_CREATED_AT
                                    }
                                    th {
                                        class: c_tracking_table_head()
                                        HEAD_ACTION
                                    }
                                }
                            }
                            tbody {
                                for record in { records } {
                                    tr {
                                        class: c_tracking_table_row()
                                        td {
                                            class: c_tracking_table_cell()
                                            record.id.to_string()
                                        }
                                        td {
                                            class: c_tracking_table_cell()
                                            format_local_time(record.timestamp)
                                        }
                                        td {
                                            class: c_tracking_table_cell()
                                            format_local_time(record.created_at)
                                        }
                                        td {
                                            class: c_tracking_table_cell()
                                            euv_button {
                                                variant: EuvButtonVariant::Outline
                                                onclick: detail_handler(state, record.id)
                                                LABEL_DETAIL
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    pagination_bar {
                        state: state
                    }
                }
            }
            if { state.get_detail().get().is_some() } {
                euv_modal {
                    title: TITLE_DETAIL
                    onclick: close_detail_handler(state)
                    record_detail {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders one metric tile of the page.
///
/// # Arguments
///
/// - `VirtualNode<StatTileProps>` - The props node carrying the metric.
///
/// # Returns
///
/// - `VirtualNode` - The metric tile tree.
#[component]
pub fn stat_tile(node: VirtualNode<StatTileProps>) -> VirtualNode {
    let StatTileProps {
        label,
        value,
        icon,
    }: StatTileProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_hls_stat_card()
            div {
                class: c_hls_stat_icon()
                icon
            }
            div {
                class: c_hls_stat_value()
                value
            }
            div {
                class: c_hls_stat_label()
                label
            }
        }
    }
}

/// Renders the pager of the records.
///
/// # Arguments
///
/// - `VirtualNode<TrackingPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn pagination_bar(node: VirtualNode<TrackingPageProps>) -> VirtualNode {
    let TrackingPageProps { state }: TrackingPageProps = node.try_get_props().unwrap_or_default();
    let total: i64 = state.get_total().get();
    let page: i64 = state.get_page().get();
    let page_size: i64 = state.get_page_size().get();
    html! {
        div {
            class: c_tracking_pagination()
            euv_button {
                variant: EuvButtonVariant::Outline
                onclick: previous_page_handler(state)
                LABEL_PREVIOUS
            }
            span {
                class: c_tracking_pagination_info()
                format!("{page} / {}", total_pages(total, page_size))
            }
            euv_button {
                variant: EuvButtonVariant::Outline
                onclick: next_page_handler(state)
                LABEL_NEXT
            }
        }
    }
}

/// Renders the detail of the open record.
///
/// # Arguments
///
/// - `VirtualNode<TrackingPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The detail tree.
#[component]
pub fn record_detail(node: VirtualNode<TrackingPageProps>) -> VirtualNode {
    let TrackingPageProps { state }: TrackingPageProps = node.try_get_props().unwrap_or_default();
    let detail: Option<TrackingRecord> = state.get_detail().get();
    let Some(record) = detail else {
        return html! {
            ""
        };
    };
    let headers: String = serde_json::to_string_pretty(&record.headers).unwrap_or_default();
    html! {
        div {
            class: c_tracking_detail()
            div {
                class: c_tracking_detail_label()
                LABEL_HEADERS
            }
            pre {
                class: c_tracking_detail_body()
                headers.as_str()
            }
            div {
                class: c_tracking_detail_label()
                LABEL_BODY
            }
            pre {
                class: c_tracking_detail_body()
                record.body.as_str()
            }
        }
    }
}

/// Counts the pages the filter holds.
///
/// # Arguments
///
/// - `i64` - The number of the records the filter holds.
/// - `i64` - The number of the records one page holds.
///
/// # Returns
///
/// - `i64` - The number of the pages.
fn total_pages(total: i64, page_size: i64) -> i64 {
    if page_size <= 0 {
        return PAGE_FIRST;
    }
    let pages: i64 = (total + page_size - 1) / page_size;
    if pages < PAGE_FIRST {
        return PAGE_FIRST;
    }
    pages
}

/// Builds the handler that keeps the start of the window.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn start_time_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_start_time().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the end of the window.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn end_time_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_end_time().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the header the query filters on.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn header_key_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_header_key().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the value of the header the query filters on.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn header_value_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_header_value().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the content the query filters on.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn body_content_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_body_content().set(input_value(&event));
    }))
}

/// Builds the handler that runs the query of the filters.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn search_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_page().set(PAGE_FIRST);
        spawn_local(run_query(state));
    }))
}

/// Builds the handler that clears the filters and runs the query again.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn reset_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(reset_query(state));
    }))
}

/// Builds the handler that shows the previous page of the records.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i64 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(run_query(state));
        }
    }))
}

/// Builds the handler that shows the next page of the records.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i64 = state.get_page().get();
        let total: i64 = state.get_total().get();
        let page_size: i64 = state.get_page_size().get();
        if page < total_pages(total, page_size) {
            state.get_page().set(page + 1);
            spawn_local(run_query(state));
        }
    }))
}

/// Builds the handler that opens the detail of a record.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
/// - `i64` - The identifier of the record to open.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn detail_handler(state: TrackingState, id: i64) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let records: Vec<TrackingRecord> = state.get_records().get();
        let record: Option<TrackingRecord> = records.into_iter().find(|record: &TrackingRecord| record.id == id);
        state.get_detail().set(record);
    }))
}

/// Builds the handler that closes the detail of a record.
///
/// # Arguments
///
/// - `TrackingState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_detail_handler(state: TrackingState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail().set(None);
    }))
}
