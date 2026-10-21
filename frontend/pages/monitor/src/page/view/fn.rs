use super::*;

/// Renders the monitor page.
///
/// The page shows the metrics of the live status stream of the server, the
/// static information of the machine and the samples of the performance
/// history.
///
/// # Arguments
///
/// - `VirtualNode<MonitorPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn monitor_page(node: VirtualNode<MonitorPageProps>) -> VirtualNode {
    let MonitorPageProps { state }: MonitorPageProps = node.try_get_props().unwrap_or_default();
    let status: Option<ServerStatus> = state.get_status().get();
    let history: Signal<Vec<PerformancePoint>> = state.get_history();
    let info: SystemInfo = state.get_info().get();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_MONITOR
                title: TITLE_MONITOR
                subtitle: SUBTITLE_MONITOR
            }
            div {
                class: c_monitor_toolbar()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: refresh_handler(state)
                    LABEL_REFRESH
                }
                span {
                    class: c_monitor_connection()
                    if { state.get_connected().get() } {
                        TEXT_CONNECTED
                    } else {
                        TEXT_DISCONNECTED
                    }
                }
            }
            if { state.get_loading().get() } {
                euv_loading {
                    title: TITLE_LOADING
                }
            } else if { !state.get_error().get().is_empty() } {
                euv_alert {
                    variant: AlertVariant::Error
                    state.get_error().get().as_str()
                }
            }
            div {
                class: c_monitor_stats()
                stat_tile {
                    label: LABEL_CPU
                    value: metric(&status, |status: &ServerStatus| format_percent(status.cpu_usage))
                    icon: ICON_CPU
                }
                stat_tile {
                    label: LABEL_MEMORY_USAGE
                    value: metric(&status, |status: &ServerStatus| format_percent(status.memory_usage))
                    icon: ICON_MEMORY
                }
                stat_tile {
                    label: LABEL_DISK_USAGE
                    value: metric(&status, |status: &ServerStatus| format_percent(status.disk_usage))
                    icon: ICON_DISK
                }
                stat_tile {
                    label: LABEL_LOAD
                    value: metric(&status, |status: &ServerStatus| format!("{:.2}", status.load_average))
                    icon: ICON_LOAD
                }
            }
            euv_card {
                title: TITLE_OVERVIEW
                div {
                    class: c_monitor_overview()
                    info_entry {
                        label: LABEL_HOSTNAME
                        value: text_or_unknown(&info.hostname)
                    }
                    info_entry {
                        label: LABEL_OS
                        value: text_or_unknown(&info.os_name)
                    }
                    info_entry {
                        label: LABEL_KERNEL
                        value: text_or_unknown(&info.kernel_version)
                    }
                    info_entry {
                        label: LABEL_CPU_MODEL
                        value: text_or_unknown(&info.cpu_model)
                    }
                    info_entry {
                        label: LABEL_CPU_CORES
                        value: info.cpu_cores.to_string()
                    }
                    info_entry {
                        label: LABEL_MEMORY
                        value: format_bytes(info.total_memory)
                    }
                    info_entry {
                        label: LABEL_DISK
                        value: format_bytes(info.total_disk)
                    }
                }
            }
            euv_card {
                title: TITLE_LIVE
                div {
                    class: c_monitor_overview()
                    info_entry {
                        label: LABEL_UPTIME
                        value: metric(&status, |status: &ServerStatus| format_uptime(status.uptime))
                    }
                    info_entry {
                        label: LABEL_NETWORK_RX
                        value: metric(&status, |status: &ServerStatus| format_bytes(status.network_rx))
                    }
                    info_entry {
                        label: LABEL_NETWORK_TX
                        value: metric(&status, |status: &ServerStatus| format_bytes(status.network_tx))
                    }
                    info_entry {
                        label: LABEL_CONNECTIONS
                        value: metric(&status, |status: &ServerStatus| status.active_connections.to_string())
                    }
                    info_entry {
                        label: LABEL_PROCESSES
                        value: metric(&status, |status: &ServerStatus| status.process_count.to_string())
                    }
                }
            }
            euv_card {
                title: TITLE_HISTORY
                if { history.get().is_empty() } {
                    euv_result {
                        code: CODE_EMPTY
                        title: TITLE_EMPTY
                        description: TEXT_NO_SAMPLE
                    }
                } else {
                    div {
                        class: c_monitor_table_wrap()
                        table {
                            class: c_monitor_table()
                            thead {
                                tr {
                                    th {
                                        class: c_monitor_table_head()
                                        HEAD_TIME
                                    }
                                    th {
                                        class: c_monitor_table_head()
                                        HEAD_CPU
                                    }
                                    th {
                                        class: c_monitor_table_head()
                                        HEAD_MEMORY
                                    }
                                    th {
                                        class: c_monitor_table_head()
                                        HEAD_DISK
                                    }
                                    th {
                                        class: c_monitor_table_head()
                                        HEAD_NETWORK
                                    }
                                    th {
                                        class: c_monitor_table_head()
                                        LABEL_CONNECTIONS
                                    }
                                }
                            }
                            tbody {
                                for point in { history } {
                                    tr {
                                        class: c_monitor_table_row()
                                        td {
                                            class: c_monitor_table_cell()
                                            format_time(point.timestamp as i64)
                                        }
                                        td {
                                            class: c_monitor_table_cell()
                                            format_percent(point.cpu_usage)
                                        }
                                        td {
                                            class: c_monitor_table_cell()
                                            format_percent(point.memory_usage)
                                        }
                                        td {
                                            class: c_monitor_table_cell()
                                            format_percent(point.disk_usage)
                                        }
                                        td {
                                            class: c_monitor_table_cell()
                                            format!("{}/{}", format_bytes(point.network_rx), format_bytes(point.network_tx))
                                        }
                                        td {
                                            class: c_monitor_table_cell()
                                            point.active_connections.to_string()
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

/// Renders one entry of the system overview.
///
/// # Arguments
///
/// - `VirtualNode<InfoEntryProps>` - The props node carrying the entry.
///
/// # Returns
///
/// - `VirtualNode` - The entry tree.
#[component]
pub fn info_entry(node: VirtualNode<InfoEntryProps>) -> VirtualNode {
    let InfoEntryProps { label, value }: InfoEntryProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_monitor_info()
            span {
                class: c_monitor_info_label()
                label
            }
            span {
                class: c_monitor_info_value()
                value.as_str()
            }
        }
    }
}

/// Reads one metric of the live status of the server.
///
/// # Arguments
///
/// - `&Option<ServerStatus>` - The live status, absent before the first frame.
/// - `fn(&ServerStatus) -> String` - The metric the caller is interested in.
///
/// # Returns
///
/// - `String` - The value of the metric, or the placeholder before the first frame.
fn metric(status: &Option<ServerStatus>, pick: fn(&ServerStatus) -> String) -> String {
    match status {
        Some(status) => pick(status),
        None => TEXT_UNKNOWN.to_string(),
    }
}

/// Reads a text the server reported.
///
/// # Arguments
///
/// - `&str` - The reported text.
///
/// # Returns
///
/// - `String` - The reported text, or the placeholder when it is empty.
fn text_or_unknown(text: &str) -> String {
    if text.is_empty() {
        return TEXT_UNKNOWN.to_string();
    }
    text.to_string()
}

/// Builds the handler that reloads the information and the history.
///
/// # Arguments
///
/// - `MonitorState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn refresh_handler(state: MonitorState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(load_overview(state));
    }))
}
