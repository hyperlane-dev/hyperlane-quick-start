use super::*;

/// Renders the notification page.
///
/// The page holds the metrics of the notifications, the two filters, the list
/// of notifications and the actions of the page. The detail of a notification
/// and the form that writes one are opened above the page.
///
/// # Arguments
///
/// - `VirtualNode<NotificationPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn notification_page(node: VirtualNode<NotificationPageProps>) -> VirtualNode {
    let NotificationPageProps { state }: NotificationPageProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_BELL
                title: TITLE_HEADER
                subtitle: TEXT_HEADER
            }
            div {
                class: c_notification_stats()
                stat_tile {
                    label: LABEL_TOTAL
                    value: state.get_total().get().to_string()
                    icon: ICON_TOTAL
                }
                stat_tile {
                    label: LABEL_UNREAD
                    value: state.get_unread().get().to_string()
                    icon: ICON_UNREAD
                }
                stat_tile {
                    label: LABEL_PAGE
                    value: state.get_page().get().to_string()
                    icon: ICON_PAGE
                }
            }
            notification_list_panel {
                state: state
            }
            div {
                class: c_notification_actions()
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: open_create_handler(state)
                        LABEL_NEW
                    }
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: mark_all_handler(state)
                        LABEL_MARK_ALL
                    }
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: refresh_handler(state)
                        LABEL_REFRESH
                    }
                }
            }
            if { state.get_detail().get().is_some() } {
                euv_modal {
                    title: TITLE_DETAIL
                    onclick: close_detail_handler(state)
                    notification_detail {
                        state: state
                    }
                }
            }
            if { state.get_create_open().get() } {
                euv_modal {
                    title: TITLE_CREATE
                    onclick: close_create_handler(state)
                    notification_form {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the list of notifications with its filters and its pager.
///
/// # Arguments
///
/// - `VirtualNode<NotificationListProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The list panel tree.
#[component]
pub fn notification_list_panel(node: VirtualNode<NotificationListProps>) -> VirtualNode {
    let NotificationListProps { state }: NotificationListProps =
        node.try_get_props().unwrap_or_default();
    let notifications: Signal<Vec<NotificationItem>> = state.get_notifications();
    html! {
        euv_card {
            title: TITLE_LIST
            div {
                class: c_notification_filters()
                div {
                    class: c_notification_filter()
                    span {
                        class: c_notification_filter_label()
                        LABEL_FILTER_TYPE
                    }
                    div {
                        class: c_notification_filter_buttons()
                        filter_button {
                            state: state
                            kind: FILTER_TYPE
                            value: FILTER_ALL
                            label: LABEL_ALL_TYPES
                        }
                        filter_button {
                            state: state
                            kind: FILTER_TYPE
                            value: TYPE_SYSTEM
                            label: LABEL_SYSTEM
                        }
                        filter_button {
                            state: state
                            kind: FILTER_TYPE
                            value: TYPE_MESSAGE
                            label: LABEL_MESSAGE
                        }
                        filter_button {
                            state: state
                            kind: FILTER_TYPE
                            value: TYPE_ALERT
                            label: LABEL_ALERT
                        }
                    }
                }
                div {
                    class: c_notification_filter()
                    span {
                        class: c_notification_filter_label()
                        LABEL_FILTER_READ
                    }
                    div {
                        class: c_notification_filter_buttons()
                        filter_button {
                            state: state
                            kind: FILTER_READ_KIND
                            value: FILTER_ALL
                            label: LABEL_ALL_STATUS
                        }
                        filter_button {
                            state: state
                            kind: FILTER_READ_KIND
                            value: FILTER_UNREAD
                            label: LABEL_UNREAD_FILTER
                        }
                        filter_button {
                            state: state
                            kind: FILTER_READ_KIND
                            value: FILTER_READ
                            label: LABEL_READ_FILTER
                        }
                    }
                }
            }
            match { !state.get_notice().get().is_empty() } {
                true => {
                    euv_alert {
                        variant: AlertVariant::Success
                        state.get_notice().get().as_str()
                    }
                }
                false => {
                    match { !state.get_error().get().is_empty() } {
                        true => {
                            euv_alert {
                                variant: AlertVariant::Error
                                state.get_error().get().as_str()
                            }
                        }
                        false => {
                            ""
                        }
                    }
                }
            }
            match { state.get_loading().get() } {
                true => {
                    euv_loading {
                        title: "Loading the notifications"
                    }
                }
                false => {
                    match { state.get_notifications().get().is_empty() } {
                        true => {
                            euv_result {
                                code: "0"
                                title: TITLE_EMPTY
                                description: TEXT_EMPTY
                            }
                        }
                        false => {
                            div {
                                class: c_notification_list()
                                for notification in { notifications } {
                                    notification_row {
                                        state: state
                                        notification: notification
                                    }
                                }
                            }
                            pagination_bar {
                                state: state
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Renders one button of a filter.
///
/// The button that holds the value of the filter is marked, so the visitor
/// always knows which filter the list follows.
///
/// # Arguments
///
/// - `VirtualNode<FilterButtonProps>` - The props node carrying the filter.
///
/// # Returns
///
/// - `VirtualNode` - The filter button tree.
#[component]
pub fn filter_button(node: VirtualNode<FilterButtonProps>) -> VirtualNode {
    let FilterButtonProps {
        state,
        kind,
        value,
        label,
    }: FilterButtonProps = node.try_get_props().unwrap_or_default();
    let active: bool = filter_of(&state, kind) == value;
    let handler: Option<Rc<dyn Fn(Event)>> = filter_handler(state, kind.to_string(), value.to_string());
    html! {
        if active == true {
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: handler.clone()
                    label
                }
            }
        } else {
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: handler.clone()
                    label
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

/// Renders one notification of the list.
///
/// # Arguments
///
/// - `VirtualNode<NotificationRowProps>` - The props node carrying the notification.
///
/// # Returns
///
/// - `VirtualNode` - The notification row tree.
#[component]
pub fn notification_row(node: VirtualNode<NotificationRowProps>) -> VirtualNode {
    let NotificationRowProps { state, notification }: NotificationRowProps =
        node.try_get_props().unwrap_or_default();
    let notification_id: String = notification.id.clone();
    let read_id: String = notification.id.clone();
    let delete_id: String = notification.id.clone();
    let kind: String = notification.notification_type.clone();
    let glyph: &str = glyph_of(&kind);
    let time: String = notification.created_at.to_string();
    let unread: bool = !notification.is_read;
    html! {
        div {
            key: notification.id.to_string()
            class: c_notification_item()
            div {
                class: c_notification_icon()
                glyph
            }
            div {
                class: c_notification_content()
                div {
                    class: c_notification_title()
                    onclick: open_detail_handler(state, notification_id.clone())
                    notification.title.as_str()
                }
                div {
                    class: c_notification_text()
                    notification.content.as_str()
                }
                div {
                    class: c_notification_meta()
                    span {
                        class: c_notification_badge()
                        kind.as_str()
                    }
                    span {
                        class: c_notification_time()
                        time.as_str()
                    }
                }
            }
            div {
                class: c_notification_item_actions()
                if unread == true {
                    div {
                        class: c_hls_button_row()
                        euv_button {
                            variant: EuvButtonVariant::Outline
                            onclick: mark_read_handler(state, read_id.clone())
                            LABEL_MARK_READ
                        }
                    }
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: delete_handler(state, delete_id.clone())
                        LABEL_DELETE
                    }
                }
            }
        }
    }
}

/// Renders the pager of the list of notifications.
///
/// # Arguments
///
/// - `VirtualNode<NotificationListProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn pagination_bar(node: VirtualNode<NotificationListProps>) -> VirtualNode {
    let NotificationListProps { state }: NotificationListProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_notification_pagination()
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: previous_page_handler(state)
                    LABEL_PREVIOUS
                }
            }
            span {
                class: c_notification_pagination_info()
                state.get_page().get().to_string()
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: next_page_handler(state)
                    LABEL_NEXT
                }
            }
        }
    }
}

/// Renders the detail of the notification that is open.
///
/// # Arguments
///
/// - `VirtualNode<NotificationDetailProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The detail tree.
#[component]
pub fn notification_detail(node: VirtualNode<NotificationDetailProps>) -> VirtualNode {
    let NotificationDetailProps { state }: NotificationDetailProps =
        node.try_get_props().unwrap_or_default();
    let Some(notification) = state.get_detail().get() else {
        return html! {
            ""
        };
    };
    let kind: String = notification.notification_type.clone();
    let glyph: &str = glyph_of(&kind);
    let time: String = notification.created_at.to_string();
    let read: bool = notification.is_read;
    html! {
        div {
            class: c_notification_detail()
            div {
                class: c_info_row()
                span {
                    class: c_info_label()
                    LABEL_DETAIL_TYPE
                }
                span {
                    class: c_info_value()
                    glyph
                }
            }
            div {
                class: c_info_row()
                span {
                    class: c_info_label()
                    LABEL_DETAIL_CONTENT
                }
                span {
                    class: c_info_value()
                    notification.content.as_str()
                }
            }
            div {
                class: c_info_row()
                span {
                    class: c_info_label()
                    LABEL_DETAIL_STATUS
                }
                span {
                    class: c_info_value()
                    if read == true {
                        TEXT_DETAIL_READ
                    } else {
                        TEXT_DETAIL_UNREAD
                    }
                }
            }
            div {
                class: c_info_row()
                span {
                    class: c_info_label()
                    LABEL_DETAIL_TIME
                }
                span {
                    class: c_info_value()
                    time.as_str()
                }
            }
        }
    }
}

/// Renders the form that writes a notification.
///
/// # Arguments
///
/// - `VirtualNode<NotificationFormProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The form tree.
#[component]
pub fn notification_form(node: VirtualNode<NotificationFormProps>) -> VirtualNode {
    let NotificationFormProps { state }: NotificationFormProps =
        node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_notification_form()
            euv_field {
                id: "notification-title"
                label: LABEL_TITLE
                placeholder: PLACEHOLDER_TITLE
                value: state.get_create_title()
                oninput: create_title_handler(state)
            }
            div {
                class: c_form_label()
                LABEL_CONTENT
            }
            textarea {
                class: c_textarea_input()
                placeholder: PLACEHOLDER_CONTENT
                rows: ROWS_CONTENT
                oninput: create_content_handler(state)
            }
            div {
                class: c_notification_filter_buttons()
                filter_button {
                    state: state
                    kind: FILTER_CREATE_TYPE
                    value: TYPE_SYSTEM
                    label: LABEL_SYSTEM
                }
                filter_button {
                    state: state
                    kind: FILTER_CREATE_TYPE
                    value: TYPE_MESSAGE
                    label: LABEL_MESSAGE
                }
                filter_button {
                    state: state
                    kind: FILTER_CREATE_TYPE
                    value: TYPE_ALERT
                    label: LABEL_ALERT
                }
            }
            div {
                class: c_button_controls()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: close_create_handler(state)
                    LABEL_CANCEL
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: create_handler(state)
                    LABEL_CREATE
                }
            }
        }
    }
}

/// Returns the value of one filter of the state.
///
/// # Arguments
///
/// - `&NotificationState` - The state the page renders from.
/// - `&str` - The kind of the filter.
///
/// # Returns
///
/// - `String` - The value of the filter.
fn filter_of(state: &NotificationState, kind: &str) -> String {
    if kind == FILTER_TYPE {
        return state.get_filter_type().get();
    }
    if kind == FILTER_READ_KIND {
        return state.get_filter_read().get();
    }
    state.get_create_type().get()
}

/// Returns the glyph of a kind of notification.
///
/// # Arguments
///
/// - `&str` - The kind of the notification.
///
/// # Returns
///
/// - `&str` - The glyph of the kind.
fn glyph_of(kind: &str) -> &str {
    match kind {
        TYPE_SYSTEM => ICON_SYSTEM,
        TYPE_MESSAGE => ICON_MESSAGE,
        TYPE_ALERT => ICON_ALERT,
        _ => ICON_DEFAULT,
    }
}

/// Builds the handler that filters the list.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The kind of the filter.
/// - `String` - The value of the filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn filter_handler(
    state: NotificationState,
    kind: String,
    value: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if kind == FILTER_TYPE {
            state.get_filter_type().set(value.clone());
        } else if kind == FILTER_READ_KIND {
            state.get_filter_read().set(value.clone());
        } else {
            state.get_create_type().set(value.clone());
        }
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_notifications(state));
    }))
}

/// Builds the handler that opens the detail of a notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The identifier of the notification.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_detail_handler(state: NotificationState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let found: Option<NotificationItem> = state
            .get_notifications()
            .get()
            .into_iter()
            .find(|item: &NotificationItem| item.id == id);
        let Some(notification) = found else {
            return;
        };
        open_detail(state, notification);
    }))
}

/// Builds the handler that marks one notification as read.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The identifier of the notification.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn mark_read_handler(state: NotificationState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(mark_as_read(state, id.clone()));
    }))
}

/// Builds the handler that removes a notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
/// - `String` - The identifier of the notification.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn delete_handler(state: NotificationState, id: String) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(delete_notification(state, id.clone()));
    }))
}

/// Builds the handler that marks every notification as read.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn mark_all_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(mark_all_as_read(state));
    }))
}

/// Builds the handler that reloads the list.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn refresh_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_unread_count(state));
        spawn_local(load_notifications(state));
    }))
}

/// Builds the handler that opens the form that writes a notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_create_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(true);
    }))
}

/// Builds the handler that closes the form that writes a notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_create_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(false);
    }))
}

/// Builds the handler that creates the notification the form holds.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_notification(state));
    }))
}

/// Builds the handler that keeps the title of the notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn create_title_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_create_title().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the body of the notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn create_content_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_create_content().set(textarea_value(&event));
    }))
}

/// Builds the handler that closes the detail of a notification.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_detail_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_detail().set(None);
    }))
}

/// Builds the handler that shows the previous page of the list.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(load_notifications(state));
        }
    }))
}

/// Builds the handler that shows the next page of the list.
///
/// # Arguments
///
/// - `NotificationState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: NotificationState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        let total: i64 = state.get_total().get();
        let last: i32 = ((total + PAGE_SIZE as i64 - 1) / PAGE_SIZE as i64) as i32;
        if page < last {
            state.get_page().set(page + 1);
            spawn_local(load_notifications(state));
        }
    }))
}
