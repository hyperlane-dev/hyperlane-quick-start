use super::*;

/// Renders the order page.
///
/// The page holds a tab bar over the statistics, the records, the users and
/// the profile of the visitor. The form that writes a record is opened above
/// the page.
///
/// # Arguments
///
/// - `VirtualNode<OrderPageProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The page tree.
#[component]
pub fn order_page(node: VirtualNode<OrderPageProps>) -> VirtualNode {
    let OrderPageProps { state }: OrderPageProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_page_container()
            euv_header {
                icon: ICON_ORDER
                title: TITLE_HEADER
                subtitle: TEXT_HEADER
            }
            euv_tabs {
                items: tab_items(&state)
                active: state.get_tab()
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
            match { state.get_tab().get() == TAB_DASHBOARD } {
                true => {
                    dashboard_panel {
                        state: state
                    }
                }
                false => {
                    match { state.get_tab().get() == TAB_RECORDS } {
                        true => {
                            records_panel {
                                state: state
                            }
                        }
                        false => {
                            match { state.get_tab().get() == TAB_USERS } {
                                true => {
                                    users_panel {
                                        state: state
                                    }
                                }
                                false => {
                                    profile_panel {
                                        state: state
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if { state.get_create_open().get() } {
                euv_modal {
                    title: TITLE_RECORD
                    onclick: close_record_handler(state)
                    record_form {
                        state: state
                    }
                }
            }
        }
    }
}

/// Renders the statistics of the ledger.
///
/// The page shows the headline numbers of the statistics, the charts the
/// server offers with them are left out.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The panel tree.
#[component]
pub fn dashboard_panel(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    let Some(statistics) = state.get_overview().get() else {
        return html! {
            euv_result {
                code: "0"
                title: TITLE_EMPTY
                description: TEXT_EMPTY_STATISTICS
            }
        };
    };
    let today: TodayStatistics = statistics.today.clone();
    let changes: ChangesStatistics = statistics.changes.clone();
    let averages: AverageTransactionStats = statistics.avg_transaction_stats.clone();
    html! {
        euv_card {
            title: TITLE_DASHBOARD
            div {
                class: c_order_stats()
                stat_tile {
                    label: LABEL_TRANSACTIONS
                    value: today.transactions.to_string()
                    icon: ICON_TRANSACTIONS
                    hint: change_text(changes.transactions_change)
                }
                stat_tile {
                    label: LABEL_INCOME
                    value: today.income.clone()
                    icon: ICON_INCOME
                    hint: change_text(changes.income_change)
                }
                stat_tile {
                    label: LABEL_EXPENSE
                    value: today.expense.clone()
                    icon: ICON_EXPENSE
                    hint: change_text(changes.expense_change)
                }
                stat_tile {
                    label: LABEL_NEW_USERS
                    value: today.new_users.to_string()
                    icon: ICON_USERS
                    hint: change_text(changes.new_users_change)
                }
            }
            div {
                class: c_order_stats()
                stat_tile {
                    label: LABEL_AVG_INCOME
                    value: averages.avg_income_per_transaction.clone()
                    icon: ICON_AVERAGE
                    hint: change_text(changes.avg_income_change)
                }
                stat_tile {
                    label: LABEL_AVG_EXPENSE
                    value: averages.avg_expense_per_transaction.clone()
                    icon: ICON_AVERAGE
                    hint: change_text(changes.avg_expense_change)
                }
                stat_tile {
                    label: LABEL_OVERALL_AVG
                    value: averages.overall_avg_amount.clone()
                    icon: ICON_AVERAGE
                    hint: EMPTY.to_string()
                }
                stat_tile {
                    label: LABEL_MAX_INCOME
                    value: averages.max_single_income.clone()
                    icon: ICON_MAXIMUM
                    hint: EMPTY.to_string()
                }
                stat_tile {
                    label: LABEL_MAX_EXPENSE
                    value: averages.max_single_expense.clone()
                    icon: ICON_MAXIMUM
                    hint: EMPTY.to_string()
                }
            }
        }
    }
}

/// Renders the records of the ledger with their filters and their pager.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The panel tree.
#[component]
pub fn records_panel(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    let records: Signal<Vec<RecordItem>> = state.get_records();
    html! {
        div {
            class: c_order_stats()
            stat_tile {
                label: LABEL_TOTAL_INCOME
                value: state.get_total_income().get()
                icon: ICON_INCOME
                hint: EMPTY.to_string()
            }
            stat_tile {
                label: LABEL_TOTAL_EXPENSE
                value: state.get_total_expense().get()
                icon: ICON_EXPENSE
                hint: EMPTY.to_string()
            }
            stat_tile {
                label: LABEL_BALANCE
                value: state.get_balance().get()
                icon: ICON_BALANCE
                hint: EMPTY.to_string()
            }
            stat_tile {
                label: LABEL_RECORD_COUNT
                value: state.get_total_count().get().to_string()
                icon: ICON_RECORDS
                hint: EMPTY.to_string()
            }
        }
        euv_card {
            title: TITLE_RECORDS
            div {
                class: c_order_filters()
                euv_field {
                    id: "order-start-date"
                    label: LABEL_START_DATE
                    input_type: INPUT_DATE
                    value: state.get_start_date()
                    oninput: start_date_handler(state)
                }
                euv_field {
                    id: "order-end-date"
                    label: LABEL_END_DATE
                    input_type: INPUT_DATE
                    value: state.get_end_date()
                    oninput: end_date_handler(state)
                }
                div {
                    class: c_order_filter()
                    span {
                        class: c_order_filter_label()
                        LABEL_FILTER_TYPE
                    }
                    div {
                        class: c_order_filter_buttons()
                        filter_button {
                            state: state
                            kind: FILTER_KIND_TYPE
                            value: EMPTY
                            label: LABEL_ALL_TYPES
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_TYPE
                            value: TYPE_INCOME
                            label: LABEL_INCOME_FILTER
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_TYPE
                            value: TYPE_EXPENSE
                            label: LABEL_EXPENSE_FILTER
                        }
                    }
                }
                div {
                    class: c_order_filter()
                    span {
                        class: c_order_filter_label()
                        LABEL_FILTER_CATEGORY
                    }
                    div {
                        class: c_order_filter_buttons()
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: EMPTY
                            label: LABEL_ALL_CATEGORIES
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_FOOD
                            label: LABEL_FOOD
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_TRANSPORT
                            label: LABEL_TRANSPORT
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_SHOPPING
                            label: LABEL_SHOPPING
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_ENTERTAINMENT
                            label: LABEL_ENTERTAINMENT
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_SALARY
                            label: LABEL_SALARY
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_INVESTMENT
                            label: LABEL_INVESTMENT
                        }
                        filter_button {
                            state: state
                            kind: FILTER_KIND_CATEGORY
                            value: CATEGORY_OTHER
                            label: LABEL_OTHER
                        }
                    }
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: open_record_handler(state)
                        LABEL_NEW_RECORD
                    }
                }
            }
            match { state.get_loading().get() } {
                true => {
                    euv_loading {
                        title: "Loading the records"
                    }
                }
                false => {
                    match { state.get_records().get().is_empty() } {
                        true => {
                            euv_result {
                                code: "0"
                                title: TITLE_EMPTY
                                description: TEXT_EMPTY_RECORDS
                            }
                        }
                        false => {
                            div {
                                class: c_order_list()
                                for record in { records } {
                                    record_row {
                                        record: record
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

/// Renders the users of the server.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The panel tree.
#[component]
pub fn users_panel(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    let users: Signal<Vec<UserProfile>> = state.get_users();
    let admin: bool = is_admin(&state);
    html! {
        euv_card {
            title: TITLE_USERS
            div {
                class: c_order_toolbar()
                euv_input {
                    id: ID_SEARCH
                    label: LABEL_SEARCH
                    placeholder: PLACEHOLDER_SEARCH
                    value: state.get_users_keyword()
                    oninput: search_handler(state)
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Outline
                        onclick: search_button_handler(state)
                        LABEL_SEARCH_BUTTON
                    }
                }
            }
            if admin == false {
                euv_result {
                    code: "403"
                    title: TITLE_EMPTY
                    description: TEXT_NO_ADMIN
                }
            } else if { state.get_users().get().is_empty() } {
                euv_result {
                    code: "0"
                    title: TITLE_EMPTY
                    description: TEXT_EMPTY_USERS
                }
            } else {
                div {
                    class: c_order_list()
                    for user in { users } {
                        user_row {
                            user: user
                        }
                    }
                }
                users_pagination {
                    state: state
                }
            }
        }
    }
}

/// Renders the profile of the visitor.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The panel tree.
#[component]
pub fn profile_panel(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    let user: Option<UserProfile> = state.get_current_user().get();
    let username: String = user
        .as_ref()
        .map(|profile: &UserProfile| profile.username.clone())
        .unwrap_or_default();
    let role: String = user
        .as_ref()
        .map(|profile: &UserProfile| profile.role.clone())
        .unwrap_or_default();
    html! {
        euv_card {
            title: TITLE_PROFILE
            div {
                class: c_order_form()
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Username"
                    }
                    span {
                        class: c_info_value()
                        username.as_str()
                    }
                }
                div {
                    class: c_info_row()
                    span {
                        class: c_info_label()
                        "Role"
                    }
                    span {
                        class: c_info_value()
                        role.as_str()
                    }
                }
                euv_field {
                    id: ID_EMAIL
                    label: LABEL_EMAIL
                    input_type: INPUT_EMAIL
                    placeholder: PLACEHOLDER_EMAIL
                    value: state.get_profile_email()
                    oninput: profile_email_handler(state)
                }
                euv_field {
                    id: ID_PHONE
                    label: LABEL_PHONE
                    input_type: INPUT_PHONE
                    placeholder: PLACEHOLDER_PHONE
                    value: state.get_profile_phone()
                    oninput: profile_phone_handler(state)
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: save_profile_handler(state)
                        LABEL_SAVE_PROFILE
                    }
                }
            }
        }
        euv_card {
            title: LABEL_NEW_PASSWORD
            div {
                class: c_order_form()
                euv_field {
                    id: ID_OLD_PASSWORD
                    label: LABEL_OLD_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_OLD_PASSWORD
                    value: state.get_old_password()
                    oninput: old_password_handler(state)
                }
                euv_field {
                    id: ID_NEW_PASSWORD
                    label: LABEL_NEW_PASSWORD
                    input_type: INPUT_PASSWORD
                    placeholder: PLACEHOLDER_NEW_PASSWORD
                    value: state.get_new_password()
                    oninput: new_password_handler(state)
                }
                div {
                    class: c_hls_button_row()
                    euv_button {
                        variant: EuvButtonVariant::Primary
                        onclick: save_password_handler(state)
                        LABEL_SAVE_PASSWORD
                    }
                }
            }
        }
    }
}

/// Renders the form that writes a record.
///
/// # Arguments
///
/// - `VirtualNode<RecordFormProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The form tree.
#[component]
pub fn record_form(node: VirtualNode<RecordFormProps>) -> VirtualNode {
    let RecordFormProps { state }: RecordFormProps = node.try_get_props().unwrap_or_default();
    let admin: bool = is_admin(&state);
    html! {
        div {
            class: c_order_form()
            div {
                class: c_order_filter()
                span {
                    class: c_order_filter_label()
                    LABEL_FILTER_TYPE
                }
                div {
                    class: c_order_filter_buttons()
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_TYPE
                        value: TYPE_EXPENSE
                        label: LABEL_EXPENSE_FILTER
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_TYPE
                        value: TYPE_INCOME
                        label: LABEL_INCOME_FILTER
                    }
                }
            }
            div {
                class: c_order_filter()
                span {
                    class: c_order_filter_label()
                    LABEL_FILTER_CATEGORY
                }
                div {
                    class: c_order_filter_buttons()
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_FOOD
                        label: LABEL_FOOD
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_TRANSPORT
                        label: LABEL_TRANSPORT
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_SHOPPING
                        label: LABEL_SHOPPING
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_ENTERTAINMENT
                        label: LABEL_ENTERTAINMENT
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_SALARY
                        label: LABEL_SALARY
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_INVESTMENT
                        label: LABEL_INVESTMENT
                    }
                    filter_button {
                        state: state
                        kind: FILTER_KIND_RECORD_CATEGORY
                        value: CATEGORY_OTHER
                        label: LABEL_OTHER
                    }
                }
            }
            euv_field {
                id: ID_AMOUNT
                label: LABEL_AMOUNT
                input_type: INPUT_NUMBER
                placeholder: PLACEHOLDER_AMOUNT
                value: state.get_record_amount()
                oninput: record_amount_handler(state)
            }
            euv_field {
                id: ID_DESCRIPTION
                label: LABEL_DESCRIPTION
                placeholder: PLACEHOLDER_DESCRIPTION
                value: state.get_record_description()
                oninput: record_description_handler(state)
            }
            euv_field {
                id: ID_BILL_DATE
                label: LABEL_BILL_DATE
                input_type: INPUT_DATE
                value: state.get_record_bill_date()
                oninput: record_bill_date_handler(state)
            }
            if admin == true {
                euv_field {
                    id: ID_TARGET_USER
                    label: LABEL_TARGET_USER
                    value: state.get_record_target_user()
                    oninput: record_target_handler(state)
                }
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: close_record_handler(state)
                    LABEL_CANCEL
                }
                euv_button {
                    variant: EuvButtonVariant::Primary
                    onclick: create_record_handler(state)
                    LABEL_CREATE
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
        hint,
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
            if !hint.is_empty() {
                div {
                    class: c_order_stat_hint()
                    hint.as_str()
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

/// Renders one record of the ledger.
///
/// # Arguments
///
/// - `VirtualNode<RecordRowProps>` - The props node carrying the record.
///
/// # Returns
///
/// - `VirtualNode` - The record row tree.
#[component]
pub fn record_row(node: VirtualNode<RecordRowProps>) -> VirtualNode {
    let RecordRowProps { record }: RecordRowProps = node.try_get_props().unwrap_or_default();
    let kind: String = record.transaction_type.clone();
    let amount: String = record.amount.clone();
    let category: String = record.category.clone();
    let description: String = record.description.clone().unwrap_or_default();
    let owner: String = record.username.clone().unwrap_or_default();
    let bill: String = record.bill_no.clone();
    let date: String = record.bill_date.to_string();
    html! {
        div {
            key: record.id.to_string()
            class: c_order_record()
            div {
                class: c_order_record_header()
                span {
                    class: c_order_record_bill()
                    bill.as_str()
                }
                span {
                    class: c_order_record_amount()
                    amount.as_str()
                }
            }
            div {
                class: c_order_record_meta()
                span {
                    class: c_order_record_tag()
                    kind.as_str()
                }
                span {
                    class: c_order_record_tag()
                    category.as_str()
                }
                if !owner.is_empty() {
                    span {
                        class: c_order_record_owner()
                        owner.as_str()
                    }
                }
                span {
                    class: c_order_record_date()
                    date.as_str()
                }
            }
            if !description.is_empty() {
                p {
                    class: c_order_record_description()
                    description.as_str()
                }
            }
        }
    }
}

/// Renders one user of the server.
///
/// # Arguments
///
/// - `VirtualNode<UserRowProps>` - The props node carrying the user.
///
/// # Returns
///
/// - `VirtualNode` - The user row tree.
#[component]
pub fn user_row(node: VirtualNode<UserRowProps>) -> VirtualNode {
    let UserRowProps { user }: UserRowProps = node.try_get_props().unwrap_or_default();
    let name: String = user.username.clone();
    let role: String = user.role.clone();
    let status: String = user.status.clone();
    let email: String = user.email.clone().unwrap_or_default();
    html! {
        div {
            key: user.id.to_string()
            class: c_order_user()
            div {
                class: c_order_user_details()
                div {
                    class: c_order_user_name()
                    name.as_str()
                }
                div {
                    class: c_order_user_meta()
                    email.as_str()
                }
            }
            div {
                class: c_order_user_badges()
                span {
                    class: c_order_record_tag()
                    role.as_str()
                }
                span {
                    class: c_order_record_tag()
                    status.as_str()
                }
            }
        }
    }
}

/// Renders the pager of the list of records.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn pagination_bar(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_order_pagination()
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: previous_page_handler(state)
                    LABEL_PREVIOUS
                }
            }
            span {
                class: c_order_pagination_info()
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

/// Renders the pager of the list of users.
///
/// # Arguments
///
/// - `VirtualNode<PanelProps>` - The props node carrying the page state.
///
/// # Returns
///
/// - `VirtualNode` - The pager tree.
#[component]
pub fn users_pagination(node: VirtualNode<PanelProps>) -> VirtualNode {
    let PanelProps { state }: PanelProps = node.try_get_props().unwrap_or_default();
    html! {
        div {
            class: c_order_pagination()
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: previous_users_handler(state)
                    LABEL_PREVIOUS
                }
            }
            span {
                class: c_order_pagination_info()
                state.get_users_page().get().to_string()
            }
            div {
                class: c_hls_button_row()
                euv_button {
                    variant: EuvButtonVariant::Outline
                    onclick: next_users_handler(state)
                    LABEL_NEXT
                }
            }
        }
    }
}

/// Returns the tabs of the page.
///
/// The tab that shows the users is only shown to an administrator.
///
/// # Arguments
///
/// - `&OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Vec<EuvTabItem>` - The tabs of the page.
fn tab_items(state: &OrderState) -> Vec<EuvTabItem> {
    if is_admin(state) {
        return TAB_ITEMS_ADMIN.to_vec();
    }
    TAB_ITEMS.to_vec()
}

/// Reports whether the visitor is an administrator.
///
/// # Arguments
///
/// - `&OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `bool` - Whether the visitor is an administrator.
fn is_admin(state: &OrderState) -> bool {
    match state.get_current_user().get() {
        Some(user) => user.role == ROLE_ADMIN,
        None => false,
    }
}

/// Returns the value of one filter of the state.
///
/// # Arguments
///
/// - `&OrderState` - The state the page renders from.
/// - `&str` - The kind of the filter.
///
/// # Returns
///
/// - `String` - The value of the filter.
fn filter_of(state: &OrderState, kind: &str) -> String {
    match kind {
        FILTER_KIND_TYPE => state.get_filter_type().get(),
        FILTER_KIND_CATEGORY => state.get_filter_category().get(),
        FILTER_KIND_RECORD_TYPE => state.get_record_type().get(),
        _ => state.get_record_category().get(),
    }
}

/// Builds the text of a change of a metric.
///
/// # Arguments
///
/// - `Option<f64>` - The change of the metric.
///
/// # Returns
///
/// - `String` - The text of the change.
fn change_text(change: Option<f64>) -> String {
    match change {
        Some(change) => format!("{change:+.2}%"),
        None => EMPTY.to_string(),
    }
}

/// Builds the handler that filters the list.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
/// - `String` - The kind of the filter.
/// - `String` - The value of the filter.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn filter_handler(
    state: OrderState,
    kind: String,
    value: String,
) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        if kind == FILTER_KIND_TYPE {
            state.get_filter_type().set(value.clone());
            state.get_page().set(PAGE_FIRST);
            spawn_local(load_records(state));
            return;
        }
        if kind == FILTER_KIND_CATEGORY {
            state.get_filter_category().set(value.clone());
            state.get_page().set(PAGE_FIRST);
            spawn_local(load_records(state));
            return;
        }
        if kind == FILTER_KIND_RECORD_TYPE {
            state.get_record_type().set(value.clone());
            return;
        }
        state.get_record_category().set(value.clone());
    }))
}

/// Builds the handler that keeps the first day of the range.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn start_date_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_start_date().set(input_value(&event));
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_records(state));
    }))
}

/// Builds the handler that keeps the last day of the range.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn end_date_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_end_date().set(input_value(&event));
        state.get_page().set(PAGE_FIRST);
        spawn_local(load_records(state));
    }))
}

/// Builds the handler that keeps the keyword of the search field.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn search_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_users_keyword().set(input_value(&event));
        state.get_users_page().set(PAGE_FIRST);
        state.get_users_cursors().set(Vec::new());
        state.get_users_pending().set(true);
    }))
}

/// Builds the handler that sends the search right away.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn search_button_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_users_pending().set(false);
        state.get_users_page().set(PAGE_FIRST);
        state.get_users_cursors().set(Vec::new());
        spawn_local(load_users(state));
    }))
}

/// Builds the handler that keeps the address of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn profile_email_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_profile_email().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the phone number of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn profile_phone_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_profile_phone().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the current password of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn old_password_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_old_password().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the new password of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn new_password_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_new_password().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the amount of the record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn record_amount_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_record_amount().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the description of the record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn record_description_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_record_description().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the day of the bill of the record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn record_bill_date_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_record_bill_date().set(input_value(&event));
    }))
}

/// Builds the handler that keeps the user of the record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The input handler.
fn record_target_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |event: Event| {
        state.get_record_target_user().set(input_value(&event));
    }))
}

/// Builds the handler that opens the form that writes a record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn open_record_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(true);
    }))
}

/// Builds the handler that closes the form that writes a record.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn close_record_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        state.get_create_open().set(false);
    }))
}

/// Builds the handler that creates the record the form holds.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn create_record_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(create_record(state));
    }))
}

/// Builds the handler that saves the contact fields of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_profile_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(save_profile(state));
    }))
}

/// Builds the handler that saves the password of the profile.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn save_password_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        spawn_local(save_password(state));
    }))
}

/// Builds the handler that shows the previous page of the records.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_page_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        if page > PAGE_FIRST {
            state.get_page().set(page - 1);
            spawn_local(load_records(state));
        }
    }))
}

/// Builds the handler that shows the next page of the records.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_page_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_page().get();
        let total: i64 = state.get_total_count().get();
        let last: i32 = ((total + PAGE_SIZE as i64 - 1) / PAGE_SIZE as i64) as i32;
        if page < last {
            state.get_page().set(page + 1);
            spawn_local(load_records(state));
        }
    }))
}

/// Builds the handler that shows the previous page of the users.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn previous_users_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_users_page().get();
        if page > PAGE_FIRST {
            state.get_users_page().set(page - 1);
            spawn_local(load_users(state));
        }
    }))
}

/// Builds the handler that shows the next page of the users.
///
/// # Arguments
///
/// - `OrderState` - The state the page renders from.
///
/// # Returns
///
/// - `Option<Rc<dyn Fn(Event)>>` - The click handler.
fn next_users_handler(state: OrderState) -> Option<Rc<dyn Fn(Event)>> {
    Some(Rc::new(move |_: Event| {
        let page: i32 = state.get_users_page().get();
        if state.get_users_has_more().get() {
            state.get_users_page().set(page + 1);
            spawn_local(load_users(state));
        }
    }))
}
